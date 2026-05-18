//! npm package → AWARE agent (via TypeScript declarations).
//!
//! Fetches a `.tgz` from registry.npmjs.org, extracts the package's
//! TypeScript declarations (`.d.ts` files), and statically scans them
//! for class / interface / function / method definitions.
//!
//! Static line-based parsing — no Node.js runtime required. Catches
//! `export declare class/interface/function/type` at the top level and
//! method/property declarations inside class bodies. Misses generic
//! constraint edge cases, nested namespaces, mapped types — but covers
//! the common public-API surface of typical libraries (xeokit, three.js,
//! React, etc.).
//!
//! Best for libraries that ship TypeScript types (the npm package.json
//! has a `types` or `typings` field). Libraries without `.d.ts` fall back
//! to scanning for `.d.ts` files anywhere in the package; if none are
//! found, returns a Validation error.

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::io::Read;

use flate2::read::GzDecoder;
use tar::Archive;

use crate::builder::{GeneratedAgent, GeneratedCommand, GeneratedSkill, Provenance, now_iso};
use crate::error::AwareError;

pub fn build_from_npm(spec: &str, agent_id: Option<&str>) -> Result<GeneratedAgent, AwareError> {
    let (name, version) = match spec.split_once('@') {
        // Handle scoped packages like `@xeokit/xeokit-sdk@2.6.109`. The first
        // `@` belongs to the scope; the version `@` is whatever comes after
        // the last `/<lastsegment>` part.
        Some(_) if spec.starts_with('@') => {
            // scope present — find the SECOND '@'
            let after_scope = &spec[1..];
            match after_scope.rsplit_once('@') {
                Some((pkg_after, v)) => (format!("@{pkg_after}"), v.to_string()),
                None => {
                    return Err(AwareError::Validation(
                        "--from-npm requires <pkg>@<version>".into(),
                    ));
                }
            }
        }
        Some((p, v)) => (p.to_string(), v.to_string()),
        None => {
            return Err(AwareError::Validation(
                "--from-npm requires <pkg>@<version>".into(),
            ));
        }
    };

    // Build tarball URL. Scoped packages keep the scope in the URL path but
    // the tarball filename drops the scope.
    //   @xeokit/xeokit-sdk@2.6.109 → registry/.../@xeokit/xeokit-sdk/-/xeokit-sdk-2.6.109.tgz
    //   typescript@5.4.5           → registry/.../typescript/-/typescript-5.4.5.tgz
    let tarball_basename = name
        .rsplit('/')
        .next()
        .map(String::from)
        .unwrap_or_else(|| name.clone());
    let url = format!("https://registry.npmjs.org/{name}/-/{tarball_basename}-{version}.tgz");

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
    // npm tarball is gzipped tar; entries are under "package/" prefix.
    let gz = GzDecoder::new(bytes);
    let mut archive = Archive::new(gz);

    let mut package_json: Option<String> = None;
    let mut dts_sources: BTreeMap<String, String> = BTreeMap::new();
    for entry in archive
        .entries()
        .map_err(|e| AwareError::Validation(format!("npm tar: {e}")))?
    {
        let mut entry = entry.map_err(|e| AwareError::Validation(format!("npm entry: {e}")))?;
        let path = entry
            .path()
            .map_err(|e| AwareError::Validation(format!("entry path: {e}")))?;
        let p = path.to_string_lossy().to_string();
        if p == "package/package.json" {
            let mut s = String::new();
            entry
                .read_to_string(&mut s)
                .map_err(|e| AwareError::Validation(format!("package.json: {e}")))?;
            package_json = Some(s);
        } else if p.ends_with(".d.ts") {
            let mut s = String::new();
            entry
                .read_to_string(&mut s)
                .map_err(|e| AwareError::Validation(format!("read {p}: {e}")))?;
            // Strip the "package/" prefix for readability.
            let stem = p.strip_prefix("package/").unwrap_or(&p).to_string();
            dts_sources.insert(stem, s);
        }
    }

    let (description, license, homepage) = parse_package_json(package_json.as_deref());

    if dts_sources.is_empty() {
        return Err(AwareError::Validation(format!(
            "package {name}@{version} ships no .d.ts files — --from-npm needs TypeScript types"
        )));
    }

    let (commands, skills) = extract_surface(name, &dts_sources);

    let id = agent_id.map(String::from).unwrap_or_else(|| {
        // Strip leading scope from "@xeokit/xeokit-sdk" → "xeokit-sdk"
        name.rsplit('/')
            .next()
            .unwrap_or(name)
            .to_lowercase()
            .replace('_', "-")
    });

    let provenance = Provenance {
        generated_by: "aware-agent-builder".into(),
        generator_version: env!("CARGO_PKG_VERSION").into(),
        source: serde_json::json!({
            "type": "npm",
            "package": name,
            "version": version,
            "license": license,
            "homepage": homepage,
            "dts_files": dts_sources.len(),
        }),
        generated_at: now_iso(),
    };

    Ok(GeneratedAgent {
        id,
        version: "0.1.0".into(),
        sdk_target: Some(version.to_string()),
        description: if description.is_empty() {
            format!("Generated from npm package {name}@{version}")
        } else {
            description
        },
        commands,
        skills,
        provenance,
        stateful: false,
        license,
    })
}

/// Parse package.json for the fields we care about — description, license, homepage.
fn parse_package_json(json: Option<&str>) -> (String, String, String) {
    let Some(text) = json else {
        return (String::new(), "UNKNOWN".into(), String::new());
    };
    let v: serde_json::Value = match serde_json::from_str(text) {
        Ok(v) => v,
        Err(_) => return (String::new(), "UNKNOWN".into(), String::new()),
    };
    let description = v["description"].as_str().unwrap_or("").to_string();
    let license = v["license"]
        .as_str()
        .map(String::from)
        .or_else(|| v["license"]["type"].as_str().map(String::from))
        .unwrap_or_else(|| "UNKNOWN".into());
    let homepage = v["homepage"].as_str().unwrap_or("").to_string();
    (description, license, homepage)
}

/// Walk each .d.ts source, extract class / interface / function / method
/// declarations via line-based scanning. Tracks the "current container" as
/// a simple latch — each top-level `class`/`interface` updates the bucket
/// subsequent method declarations land in. Good enough for idiomatic TS.
fn extract_surface(
    pkg_name: &str,
    dts_sources: &BTreeMap<String, String>,
) -> (BTreeMap<String, GeneratedCommand>, Vec<GeneratedSkill>) {
    let mut commands: BTreeMap<String, GeneratedCommand> = BTreeMap::new();
    let mut by_class: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();

    for (path, src) in dts_sources.iter() {
        let mut current: Option<String> = None;
        let mut brace_depth: i32 = 0;
        // The depth at which `current`'s body started, so we know when to
        // pop on `}` matching its open `{`.
        let mut current_depth: i32 = -1;

        for raw_line in src.lines() {
            let line = raw_line.trim();
            // Track brace depth (very loose — ignores strings/comments, but
            // good enough for d.ts files which have few of those).
            let opens = line.matches('{').count() as i32;
            let closes = line.matches('}').count() as i32;

            // Detect top-level class / interface declarations.
            if current.is_none()
                && brace_depth == 0
                && let Some(name) = parse_container_decl(line)
            {
                current = Some(name);
                current_depth = brace_depth;
                brace_depth += opens - closes;
                continue;
            }

            // Inside a class/interface body — look for method declarations.
            if let Some(cur) = current.as_ref()
                && brace_depth > current_depth
                && let Some((mname, margs)) = parse_method_decl(line)
            {
                let cmd_id = kebab(&format!("{cur}-{mname}"));
                let display = if margs.is_empty() {
                    format!("{cur}.{mname}()")
                } else {
                    format!("{cur}.{mname}({margs})")
                };
                commands.insert(
                    cmd_id,
                    GeneratedCommand {
                        lifecycle: "single".into(),
                        description: display,
                        inputs_yaml: String::new(),
                        outputs_yaml: String::new(),
                    },
                );
                by_class
                    .entry(cur.clone())
                    .or_default()
                    .push((mname, margs));
            }

            // Top-level functions (outside any container).
            if current.is_none()
                && brace_depth == 0
                && let Some((fname, fargs)) = parse_function_decl(line)
            {
                let cmd_id = kebab(&fname);
                commands.insert(
                    cmd_id,
                    GeneratedCommand {
                        lifecycle: "single".into(),
                        description: if fargs.is_empty() {
                            format!("{fname}()")
                        } else {
                            format!("{fname}({fargs})")
                        },
                        inputs_yaml: String::new(),
                        outputs_yaml: String::new(),
                    },
                );
                by_class
                    .entry("TopLevel".into())
                    .or_default()
                    .push((fname, fargs));
            }

            brace_depth += opens - closes;
            if brace_depth <= current_depth {
                current = None;
                current_depth = -1;
            }
        }
        let _ = path; // could be used to add source-file provenance per skill
    }

    let mut skills: Vec<GeneratedSkill> = by_class
        .into_iter()
        .map(|(class_name, methods)| {
            let stem = kebab(&class_name);
            let listing = methods
                .iter()
                .map(|(m, a)| {
                    if a.is_empty() {
                        format!("- `{m}()`")
                    } else {
                        format!("- `{m}({a})`")
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            let body = format!(
                "---\nname: {pkg}-{stem}\ndescription: {class_name} declarations from {pkg}\n---\n\n# {class_name}\n\n## Methods\n\n{listing}\n",
                pkg = pkg_name.rsplit('/').next().unwrap_or(pkg_name),
                stem = stem,
                class_name = class_name,
                listing = listing,
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

/// Detect `export declare class Foo`, `export declare interface IFoo`,
/// `export class Foo`, `declare class Foo`, `class Foo`. Returns the name.
fn parse_container_decl(line: &str) -> Option<String> {
    let cleaned = strip_modifiers(line);
    for kw in ["class ", "interface ", "namespace ", "module "] {
        if let Some(rest) = cleaned.strip_prefix(kw) {
            let name: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '$')
                .collect();
            if !name.is_empty() {
                return Some(name);
            }
        }
    }
    None
}

/// Detect `export declare function foo(...)`, `export function foo(...)`,
/// `declare function foo(...)`. Returns `(name, args)`.
fn parse_function_decl(line: &str) -> Option<(String, String)> {
    let cleaned = strip_modifiers(line);
    let rest = cleaned.strip_prefix("function ")?;
    let name_end = rest
        .char_indices()
        .find(|(_, c)| !c.is_ascii_alphanumeric() && *c != '_' && *c != '$')
        .map(|(i, _)| i)?;
    let name = rest[..name_end].to_string();
    let after = &rest[name_end..];
    let args = extract_params(after);
    Some((name, args))
}

/// Strip leading `export`, `declare`, `public`, `private`, `protected`,
/// `static`, `readonly`, `abstract`, `async` modifiers.
fn strip_modifiers(line: &str) -> &str {
    let mut s = line;
    loop {
        let trimmed = s.trim_start();
        let mut found = false;
        for kw in [
            "export ",
            "declare ",
            "default ",
            "public ",
            "private ",
            "protected ",
            "static ",
            "readonly ",
            "abstract ",
            "async ",
        ] {
            if let Some(rest) = trimmed.strip_prefix(kw) {
                s = rest;
                found = true;
                break;
            }
        }
        if !found {
            return s.trim_start();
        }
    }
}

/// Detect a method declaration line inside a class/interface body.
/// Matches patterns like `methodName(args): ReturnType;`, `methodName():`,
/// `methodName(args)`, and optionally returning `<T>` generics.
/// Skips constructors, accessor declarations, and field declarations.
fn parse_method_decl(line: &str) -> Option<(String, String)> {
    let cleaned = strip_modifiers(line);
    // Skip lines that don't look like callable declarations.
    if cleaned.starts_with('/')
        || cleaned.starts_with('*')
        || cleaned.starts_with("//")
        || cleaned.is_empty()
        || cleaned.starts_with('{')
        || cleaned.starts_with('}')
        || cleaned.starts_with("constructor")
    {
        return None;
    }
    // Method name is the leading identifier; must be followed by `(` (skipping
    // optional generic args `<T>`).
    let name_end = cleaned
        .char_indices()
        .find(|(_, c)| !c.is_ascii_alphanumeric() && *c != '_' && *c != '$')
        .map(|(i, _)| i)?;
    if name_end == 0 {
        return None;
    }
    let name = cleaned[..name_end].to_string();
    let mut after = &cleaned[name_end..];
    // Skip generic params `<T, U>`.
    if let Some(rest) = after.strip_prefix('<')
        && let Some(close) = rest.find('>')
    {
        after = &rest[close + 1..];
    }
    // Now we should see `(` for a method, `:` for a field — only methods.
    if !after.trim_start().starts_with('(') {
        return None;
    }
    // `get foo()` / `set foo(v)` — accessors are property reads/writes more than
    // discrete method calls. Skip for now; users can `loadModel` to set values.
    if name == "get" || name == "set" {
        return None;
    }
    let args = extract_params(after);
    Some((name, args))
}

/// Given a string starting at-or-before the `(args)` portion, extract the
/// inside of the first balanced parenthesis pair on this line. Returns ""
/// if the line doesn't contain balanced parens (multi-line declaration).
fn extract_params(s: &str) -> String {
    let s = s.trim_start();
    let bytes = s.as_bytes();
    let mut idx = 0;
    while idx < bytes.len() && bytes[idx] != b'(' {
        idx += 1;
    }
    if idx >= bytes.len() {
        return String::new();
    }
    let mut depth = 0i32;
    let start = idx + 1;
    let mut end = start;
    while idx < bytes.len() {
        match bytes[idx] {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    end = idx;
                    break;
                }
            }
            _ => {}
        }
        idx += 1;
    }
    if depth == 0 && end > start {
        s[start..end].trim().to_string()
    } else {
        String::new()
    }
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

    #[test]
    fn parse_container_decl_class() {
        assert_eq!(
            parse_container_decl("export declare class Viewer {").as_deref(),
            Some("Viewer")
        );
        assert_eq!(
            parse_container_decl("declare class Foo {").as_deref(),
            Some("Foo")
        );
        assert_eq!(parse_container_decl("class Bar {").as_deref(), Some("Bar"));
    }

    #[test]
    fn parse_container_decl_interface() {
        assert_eq!(
            parse_container_decl("export interface IViewer {").as_deref(),
            Some("IViewer")
        );
        assert_eq!(
            parse_container_decl("export declare interface ICam {").as_deref(),
            Some("ICam")
        );
    }

    #[test]
    fn parse_container_decl_ignores_type_alias() {
        // `export declare type X = ...;` shouldn't match as a class.
        assert_eq!(
            parse_container_decl("export declare type Foo = string;"),
            None
        );
    }

    #[test]
    fn parse_method_decl_basic() {
        let (n, a) = parse_method_decl("getCamera(): Camera;").unwrap();
        assert_eq!(n, "getCamera");
        assert_eq!(a, "");
    }

    #[test]
    fn parse_method_decl_with_args() {
        let (n, a) =
            parse_method_decl("loadModel(src: string, opts?: LoadOpts): Promise<Model>;").unwrap();
        assert_eq!(n, "loadModel");
        assert!(a.contains("src: string"));
    }

    #[test]
    fn parse_method_decl_skips_constructor() {
        assert!(parse_method_decl("constructor(cfg: ViewerConfiguration);").is_none());
    }

    #[test]
    fn parse_method_decl_skips_fields() {
        // `propertyName: Type;` is a field, not a method.
        assert!(parse_method_decl("scene: Scene;").is_none());
    }

    #[test]
    fn parse_function_decl_top_level() {
        let (n, a) =
            parse_function_decl("export declare function createViewer(cfg: any): Viewer;").unwrap();
        assert_eq!(n, "createViewer");
        assert!(a.contains("cfg: any"));
    }

    #[test]
    fn extract_surface_finds_class_methods() {
        let dts = r#"
            export declare class Viewer {
                constructor(cfg: ViewerConfiguration);
                loadModel(src: string): Promise<Model>;
                getCamera(): Camera;
                setOption(name: string, value: any): void;
            }
        "#;
        let mut srcs = BTreeMap::new();
        srcs.insert("types/Viewer.d.ts".into(), dts.into());
        let (cmds, skills) = extract_surface("xeokit", &srcs);
        assert!(cmds.contains_key("viewer-load-model"));
        assert!(cmds.contains_key("viewer-get-camera"));
        assert!(cmds.contains_key("viewer-set-option"));
        // Constructor is skipped.
        assert!(
            !cmds
                .iter()
                .any(|(_, c)| c.description.contains("constructor"))
        );
        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0].filename, "viewer.md");
    }

    #[test]
    fn extract_surface_finds_top_level_function() {
        let dts = "export declare function pickColor(rgb: number[]): string;\n";
        let mut srcs = BTreeMap::new();
        srcs.insert("types/util.d.ts".into(), dts.into());
        let (cmds, _) = extract_surface("xeokit", &srcs);
        assert!(cmds.contains_key("pick-color"));
    }

    #[test]
    fn parse_package_json_extracts_license_and_description() {
        let json = r#"{"name":"foo","version":"1.0.0","description":"A test","license":"MIT","homepage":"https://foo.dev"}"#;
        let (d, l, h) = parse_package_json(Some(json));
        assert_eq!(d, "A test");
        assert_eq!(l, "MIT");
        assert_eq!(h, "https://foo.dev");
    }

    #[test]
    fn parse_package_json_handles_license_object() {
        let json = r#"{"name":"foo","license":{"type":"Apache-2.0","url":"x"}}"#;
        let (_, l, _) = parse_package_json(Some(json));
        assert_eq!(l, "Apache-2.0");
    }

    #[test]
    fn missing_at_version_is_validation_error() {
        let err = build_from_npm("just-a-name", None).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn scoped_package_spec_parses_correctly() {
        // We can't reach the network in unit tests, but we can confirm the
        // pre-network parse succeeds (the URL-build step would run before the
        // first network attempt). Use an unreachable host to skip the actual
        // fetch — the test only checks we recognized the scope.
        // Easier: just check that the split logic doesn't fail by reaching
        // build_from_npm and having it return a network error rather than a
        // validation error.
        let result = build_from_npm("@scope/pkg@1.0.0", None);
        assert!(
            matches!(result, Err(AwareError::Network(_))),
            "expected network error, got {result:?}",
        );
    }
}
