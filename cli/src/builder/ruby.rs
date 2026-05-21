//! Rubygems package → AWARE agent.
//!
//! Fetches a `.gem` from rubygems.org, extracts the metadata + source tree,
//! statically scans `lib/**/*.rb` for class/module/method definitions.
//!
//! Static scan only — no Ruby interpreter required. Catches plain
//! `def`/`def self.`/class/module patterns; misses metaprogramming-defined
//! methods (`define_method`, `attr_accessor`, etc.). For most APIs that's
//! 90%+ of the public surface.

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::io::Read;

use flate2::read::GzDecoder;
use tar::Archive;

use crate::builder::{GeneratedAgent, GeneratedCommand, GeneratedSkill, Provenance, now_iso};
use crate::error::AwareError;

pub fn build_from_ruby(spec: &str, agent_id: Option<&str>) -> Result<GeneratedAgent, AwareError> {
    let (name, version) = match spec.split_once('@') {
        Some((n, v)) => (n.to_string(), v.to_string()),
        None => {
            return Err(AwareError::Validation(
                "--from-ruby requires <gem>@<version>".into(),
            ));
        }
    };
    let url = format!("https://rubygems.org/downloads/{name}-{version}.gem");
    let resp = ureq::get(&url)
        .call()
        .map_err(|e| AwareError::Network(format!("GET {url}: {e}")))?;
    let mut bytes = Vec::new();
    resp.into_reader()
        .read_to_end(&mut bytes)
        .map_err(|e| AwareError::Network(format!("read: {e}")))?;
    build_from_bytes(&bytes, &name, &version, agent_id)
}

pub(crate) fn build_from_bytes(
    bytes: &[u8],
    name: &str,
    version: &str,
    agent_id: Option<&str>,
) -> Result<GeneratedAgent, AwareError> {
    // Outer container is a tar (no gzip on the outer .gem layer).
    let mut outer = Archive::new(bytes);
    let mut metadata_yaml = String::new();
    let mut data_tar_gz: Vec<u8> = Vec::new();
    for entry in outer
        .entries()
        .map_err(|e| AwareError::Validation(format!("gem tar: {e}")))?
    {
        let mut entry = entry.map_err(|e| AwareError::Validation(format!("gem entry: {e}")))?;
        let path = entry
            .path()
            .map_err(|e| AwareError::Validation(format!("entry path: {e}")))?;
        let p = path.to_string_lossy().to_string();
        if p == "metadata.gz" {
            let mut gz_bytes = Vec::new();
            entry
                .read_to_end(&mut gz_bytes)
                .map_err(|e| AwareError::Validation(format!("read metadata.gz: {e}")))?;
            let mut gz = GzDecoder::new(&gz_bytes[..]);
            gz.read_to_string(&mut metadata_yaml)
                .map_err(|e| AwareError::Validation(format!("ungzip metadata: {e}")))?;
        } else if p == "data.tar.gz" {
            entry
                .read_to_end(&mut data_tar_gz)
                .map_err(|e| AwareError::Validation(format!("read data.tar.gz: {e}")))?;
        }
    }

    let (description, license) = parse_gemspec(&metadata_yaml);

    // Untar data.tar.gz, collect lib/**/*.rb sources.
    let mut rb_sources: BTreeMap<String, String> = BTreeMap::new();
    if !data_tar_gz.is_empty() {
        let gz = GzDecoder::new(&data_tar_gz[..]);
        let mut data_archive = Archive::new(gz);
        for entry in data_archive
            .entries()
            .map_err(|e| AwareError::Validation(format!("data tar: {e}")))?
        {
            let mut entry =
                entry.map_err(|e| AwareError::Validation(format!("data entry: {e}")))?;
            let path = entry
                .path()
                .map_err(|e| AwareError::Validation(format!("entry path: {e}")))?;
            let p = path.to_string_lossy().to_string();
            if p.starts_with("lib/") && p.ends_with(".rb") {
                let mut src = String::new();
                entry
                    .read_to_string(&mut src)
                    .map_err(|e| AwareError::Validation(format!("read {p}: {e}")))?;
                rb_sources.insert(p, src);
            }
        }
    }

    let (commands, skills) = extract_surface(name, &rb_sources);

    let id = agent_id
        .map(String::from)
        .unwrap_or_else(|| name.to_lowercase().replace('_', "-"));
    let provenance = Provenance {
        generated_by: "aware-agent-builder".into(),
        generator_version: env!("CARGO_PKG_VERSION").into(),
        source: serde_json::json!({
            "type": "ruby",
            "gem": name,
            "version": version,
            "license": license,
        }),
        generated_at: now_iso(),
    };

    Ok(GeneratedAgent {
        id,
        version: "0.1.0".into(),
        sdk_target: Some(version.to_string()),
        description: if description.is_empty() {
            format!("Generated from Rubygems package {name}@{version}")
        } else {
            description
        },
        commands,
        skills,
        provenance,
        stateful: false,
        license,
        rest: None,
    })
}

/// Walk each .rb source, build a map of (class → [(method, args)]).
/// Tracks the "current class" as a simple stack-free latching: each `class`
/// or `module` line updates the bucket subsequent `def`s land in. Good enough
/// for most idiomatic Ruby and far less work than full AST parsing.
fn extract_surface(
    pkg_name: &str,
    rb_sources: &BTreeMap<String, String>,
) -> (BTreeMap<String, GeneratedCommand>, Vec<GeneratedSkill>) {
    let mut commands: BTreeMap<String, GeneratedCommand> = BTreeMap::new();
    let mut by_class: BTreeMap<String, Vec<(String, String, bool)>> = BTreeMap::new();

    for src in rb_sources.values() {
        let mut current_class: String = "TopLevel".into();
        for line in src.lines() {
            let trimmed = line.trim_start();
            if let Some(rest) = trimmed.strip_prefix("class ") {
                current_class = parse_class_name(rest);
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("module ") {
                current_class = parse_class_name(rest);
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("def ")
                && let Some((name, args, is_self)) = parse_method_def(rest)
            {
                let cmd_id = kebab(&format!("{current_class}-{name}"));
                let sep = if is_self { "." } else { "#" };
                let display = if args.is_empty() {
                    format!("{current_class}{sep}{name}")
                } else {
                    format!("{current_class}{sep}{name}({args})")
                };
                commands.insert(
                    cmd_id,
                    GeneratedCommand {
                        lifecycle: "single".into(),
                        description: display,
                        inputs_yaml: String::new(),
                        outputs_yaml: String::new(),
                        ..Default::default()
                    },
                );
                by_class
                    .entry(current_class.clone())
                    .or_default()
                    .push((name, args, is_self));
            }
        }
    }

    let mut skills: Vec<GeneratedSkill> = by_class
        .into_iter()
        .map(|(class_name, methods)| {
            let stem = kebab(&class_name);
            let listing = methods
                .iter()
                .map(|(m, a, is_self)| {
                    let prefix = if *is_self { "self." } else { "" };
                    if a.is_empty() {
                        format!("- `{prefix}{m}`")
                    } else {
                        format!("- `{prefix}{m}({a})`")
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            let body = format!(
                "---\nname: {pkg_name}-{stem}\ndescription: {class_name} methods reflected from {pkg_name}\n---\n\n# {class_name}\n\n## Methods\n\n{listing}\n"
            );
            GeneratedSkill {
                filename: format!("{stem}.md"),
                body,
            }
        })
        .collect();
    skills.sort_by(|a, b| a.filename.cmp(&b.filename));

    (commands, skills)
}

/// Parse the right-hand side of a `class ` or `module ` line.
/// Strips inheritance (`< Parent`), takes the first token.
fn parse_class_name(rest: &str) -> String {
    let rest = rest.trim();
    let name = rest.split_whitespace().next().unwrap_or(rest);
    name.trim_end_matches('<').to_string()
}

/// Parse the right-hand side of a `def ` line. Returns `(name, args, is_self)`.
/// Returns `None` for lines that don't look like method definitions.
fn parse_method_def(rest: &str) -> Option<(String, String, bool)> {
    let rest = rest.trim();
    if rest.is_empty() {
        return None;
    }
    let (is_self, body) = if let Some(after) = rest.strip_prefix("self.") {
        (true, after)
    } else {
        (false, rest)
    };
    // Method name: identifier chars + Ruby's allowed trailing chars (! ? =).
    let name_end = body
        .char_indices()
        .find(|(_, c)| {
            !c.is_ascii_alphanumeric() && *c != '_' && *c != '!' && *c != '?' && *c != '='
        })
        .map(|(i, _)| i)
        .unwrap_or(body.len());
    if name_end == 0 {
        return None;
    }
    let name = body[..name_end].to_string();
    let after_name = body[name_end..].trim_start();
    let args = if let Some(after_paren) = after_name.strip_prefix('(') {
        // up to matching close paren on same line — good enough for most idiomatic Ruby
        let close = after_paren.find(')').unwrap_or(0);
        after_paren[..close].trim().to_string()
    } else {
        // `def foo` (no parens) — args terminate at end-of-line / `;` / newline
        let stop = after_name
            .find([';', '\n', '#'])
            .unwrap_or(after_name.len());
        after_name[..stop].trim().to_string()
    };
    Some((name, args, is_self))
}

/// Parse a gemspec YAML for the fields we care about — description + license.
/// Rubygems gemspec YAML is verbose and Ruby-flavored; we just look for the
/// well-known top-level keys.
fn parse_gemspec(yaml: &str) -> (String, String) {
    let mut description = String::new();
    let mut license = "UNKNOWN".to_string();
    let mut in_licenses_array = false;
    for line in yaml.lines() {
        let trimmed = line.trim_start();
        if let Some(d) = trimmed.strip_prefix("description: ") {
            let d = d.trim().trim_matches(|c: char| c == '"' || c == '\'');
            if !d.is_empty() && d != "~" {
                description = d.to_string();
            }
        }
        if let Some(l) = trimmed.strip_prefix("license: ") {
            let l = l.trim().trim_matches(|c: char| c == '"' || c == '\'');
            if !l.is_empty() && l != "~" {
                license = l.to_string();
            }
        }
        if trimmed.starts_with("licenses:") {
            in_licenses_array = true;
            continue;
        }
        if in_licenses_array {
            if let Some(item) = trimmed.strip_prefix("- ") {
                let item = item.trim().trim_matches(|c: char| c == '"' || c == '\'');
                if !item.is_empty() && license == "UNKNOWN" {
                    license = item.to_string();
                }
            } else if !trimmed.starts_with("- ") && !trimmed.is_empty() {
                in_licenses_array = false;
            }
        }
    }
    (description, license)
}

fn kebab(s: &str) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_ascii_uppercase() && !out.is_empty() && !out.ends_with('-') {
                out.push('-');
            }
            out.push(ch.to_ascii_lowercase());
        } else if !out.is_empty() && !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn parse_method_def_basic() {
        let (n, a, s) = parse_method_def("foo(x, y)").unwrap();
        assert_eq!(n, "foo");
        assert_eq!(a, "x, y");
        assert!(!s);
    }

    #[test]
    fn parse_method_def_self() {
        let (n, _, s) = parse_method_def("self.create(name)").unwrap();
        assert_eq!(n, "create");
        assert!(s);
    }

    #[test]
    fn parse_method_def_no_parens() {
        let (n, a, _) = parse_method_def("inspect").unwrap();
        assert_eq!(n, "inspect");
        assert_eq!(a, "");
    }

    #[test]
    fn parse_method_def_special_chars() {
        let (n, _, _) = parse_method_def("valid?").unwrap();
        assert_eq!(n, "valid?");
        let (n2, _, _) = parse_method_def("empty!").unwrap();
        assert_eq!(n2, "empty!");
        let (n3, _, _) = parse_method_def("name=(v)").unwrap();
        assert_eq!(n3, "name=");
    }

    #[test]
    fn parse_class_name_strips_inheritance() {
        assert_eq!(parse_class_name("Foo"), "Foo");
        assert_eq!(parse_class_name("Foo < Bar"), "Foo");
        assert_eq!(parse_class_name("Foo::Bar"), "Foo::Bar");
    }

    #[test]
    fn parse_gemspec_extracts_description_and_license() {
        let yaml = r#"---
name: mygem
version: 1.0.0
description: A useful gem
license: MIT
"#;
        let (d, l) = parse_gemspec(yaml);
        assert_eq!(d, "A useful gem");
        assert_eq!(l, "MIT");
    }

    #[test]
    fn parse_gemspec_handles_licenses_array() {
        let yaml = r#"---
name: mygem
licenses:
- MIT
- Apache-2.0
"#;
        let (_, l) = parse_gemspec(yaml);
        assert_eq!(l, "MIT");
    }

    #[test]
    fn extract_surface_basic() {
        let src = r#"
module Tools
  class FooBar
    def initialize(name)
    end
    def self.create(opts)
    end
    def name
    end
    def valid?
    end
  end
end
"#;
        let mut srcs = BTreeMap::new();
        srcs.insert("lib/tools/foo_bar.rb".into(), src.into());
        let (cmds, skills) = extract_surface("mygem", &srcs);
        assert!(cmds.contains_key("foo-bar-initialize"));
        assert!(cmds.contains_key("foo-bar-create"));
        assert!(cmds.contains_key("foo-bar-name"));
        assert!(cmds.contains_key("foo-bar-valid"));
        assert!(skills.iter().any(|s| s.filename == "foo-bar.md"));
    }

    fn build_fake_gem(
        name: &str,
        version: &str,
        gemspec_yaml: &str,
        rb_files: &[(&str, &str)],
    ) -> Vec<u8> {
        // Build inner data.tar.gz containing the rb files
        let mut data_tar = Vec::new();
        {
            let mut data_tar_builder = tar::Builder::new(&mut data_tar);
            for (path, body) in rb_files {
                let mut header = tar::Header::new_gnu();
                header.set_path(path).unwrap();
                header.set_size(body.len() as u64);
                header.set_cksum();
                data_tar_builder.append(&header, body.as_bytes()).unwrap();
            }
            data_tar_builder.finish().unwrap();
        }
        let mut data_tar_gz = Vec::new();
        {
            let mut gz =
                flate2::write::GzEncoder::new(&mut data_tar_gz, flate2::Compression::default());
            gz.write_all(&data_tar).unwrap();
            gz.finish().unwrap();
        }

        // Build metadata.gz
        let mut metadata_gz = Vec::new();
        {
            let mut gz =
                flate2::write::GzEncoder::new(&mut metadata_gz, flate2::Compression::default());
            gz.write_all(gemspec_yaml.as_bytes()).unwrap();
            gz.finish().unwrap();
        }

        // Build outer .gem (a tar of metadata.gz + data.tar.gz)
        let mut gem_bytes = Vec::new();
        {
            let mut builder = tar::Builder::new(&mut gem_bytes);
            let mut h1 = tar::Header::new_gnu();
            h1.set_path("metadata.gz").unwrap();
            h1.set_size(metadata_gz.len() as u64);
            h1.set_cksum();
            builder.append(&h1, &metadata_gz[..]).unwrap();
            let mut h2 = tar::Header::new_gnu();
            h2.set_path("data.tar.gz").unwrap();
            h2.set_size(data_tar_gz.len() as u64);
            h2.set_cksum();
            builder.append(&h2, &data_tar_gz[..]).unwrap();
            builder.finish().unwrap();
        }

        let _ = name;
        let _ = version;
        gem_bytes
    }

    #[test]
    fn build_from_bytes_end_to_end() {
        let gemspec =
            "---\nname: fakegem\nversion: 1.0.0\ndescription: A fake gem for tests\nlicense: MIT\n";
        let rb = "module Foo\n  class Bar\n    def hello(name)\n    end\n  end\nend\n";
        let bytes = build_fake_gem("fakegem", "1.0.0", gemspec, &[("lib/fakegem.rb", rb)]);
        let agent = build_from_bytes(&bytes, "fakegem", "1.0.0", None).unwrap();
        assert_eq!(agent.id, "fakegem");
        assert_eq!(agent.license, "MIT");
        assert_eq!(agent.description, "A fake gem for tests");
        assert!(agent.commands.contains_key("bar-hello"));
    }

    #[test]
    fn missing_at_version_is_validation_error() {
        let err = build_from_ruby("just-a-name", None).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }
}
