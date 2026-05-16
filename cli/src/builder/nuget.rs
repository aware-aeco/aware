//! NuGet package → AWARE agent (docs-only).
//!
//! Fetches a `.nupkg`, extracts the `.nuspec` (for metadata + license) and
//! any `lib/*/*.xml` XML doc files (emitted as skills). No DLL decompilation —
//! that's tier C (deferred to v0.5.1 sidecar).

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::io::Read;

use crate::builder::{GeneratedAgent, GeneratedSkill, Provenance, now_iso};
use crate::error::AwareError;

pub fn build_from_nuget(
    spec: &str,
    agent_id: Option<&str>,
    accept_license: bool,
) -> Result<GeneratedAgent, AwareError> {
    let (pkg, version) = match spec.split_once('@') {
        Some((p, v)) => (p.to_string(), v.to_string()),
        None => {
            return Err(AwareError::Validation(
                "--from-nuget requires <pkg>@<version>".into(),
            ));
        }
    };
    let lower = pkg.to_lowercase();
    let url =
        format!("https://api.nuget.org/v3-flatcontainer/{lower}/{version}/{lower}.{version}.nupkg");

    let resp = ureq::get(&url)
        .call()
        .map_err(|e| AwareError::Network(format!("GET {url}: {e}")))?;
    let mut bytes = Vec::new();
    resp.into_reader()
        .read_to_end(&mut bytes)
        .map_err(|e| AwareError::Network(format!("read: {e}")))?;

    build_from_bytes(&bytes, &pkg, &version, agent_id, accept_license)
}

/// Extracted helper for tests: takes pre-fetched bytes (skips the HTTP call).
pub(crate) fn build_from_bytes(
    bytes: &[u8],
    pkg: &str,
    version: &str,
    agent_id: Option<&str>,
    accept_license: bool,
) -> Result<GeneratedAgent, AwareError> {
    let reader = std::io::Cursor::new(bytes);
    let mut zip = zip::ZipArchive::new(reader)
        .map_err(|e| AwareError::Validation(format!("nupkg parse: {e}")))?;

    let mut nuspec_text = String::new();
    let mut xml_docs: BTreeMap<String, String> = BTreeMap::new();
    for i in 0..zip.len() {
        let mut entry = zip
            .by_index(i)
            .map_err(|e| AwareError::Validation(format!("zip entry: {e}")))?;
        let name = entry.name().to_string();
        if name.ends_with(".nuspec") && !name.contains('/') {
            entry
                .read_to_string(&mut nuspec_text)
                .map_err(|e| AwareError::Validation(format!("nuspec read: {e}")))?;
        } else if name.starts_with("lib/") && name.to_lowercase().ends_with(".xml") {
            let mut body = String::new();
            entry
                .read_to_string(&mut body)
                .map_err(|e| AwareError::Validation(format!("xml read: {e}")))?;
            let basename = name.rsplit('/').next().unwrap_or(&name).to_string();
            xml_docs.entry(basename).or_insert(body);
        }
    }

    let license = extract_nuspec_license(&nuspec_text);
    let permissive = [
        "MIT",
        "Apache-2.0",
        "BSD-2-Clause",
        "BSD-3-Clause",
        "Unlicense",
        "ISC",
        "0BSD",
    ];
    if !permissive.contains(&license.as_str()) && !accept_license {
        return Err(AwareError::PermissionDenied(format!(
            "package {pkg}@{version} has license {license:?}; re-run with --accept-license to proceed"
        )));
    }

    let description = extract_nuspec_description(&nuspec_text).unwrap_or_else(|| pkg.to_string());

    let pkg_lower = pkg.to_lowercase();
    let mut skills: Vec<GeneratedSkill> = xml_docs.iter().map(|(name, body)| {
        let stem = name.trim_end_matches(".xml").to_lowercase();
        let skill_name = format!("{stem}.md");
        let skill_body = format!(
            "---\nname: {pkg_lower}-{stem}\ndescription: API reference extracted from {name}\n---\n\n# {stem} API reference\n\n```xml\n{body}\n```\n"
        );
        GeneratedSkill { filename: skill_name, body: skill_body }
    }).collect();
    skills.sort_by(|a, b| a.filename.cmp(&b.filename));

    let id = agent_id
        .map(String::from)
        .unwrap_or_else(|| pkg_lower.clone());
    let provenance = Provenance {
        generated_by: "aware-agent-builder".into(),
        generator_version: env!("CARGO_PKG_VERSION").into(),
        source: serde_json::json!({ "type": "nuget", "package": pkg, "version": version, "license": license }),
        generated_at: now_iso(),
    };

    Ok(GeneratedAgent {
        id,
        version: version.to_string(),
        description,
        commands: BTreeMap::new(),
        skills,
        provenance,
        stateful: false,
        license,
    })
}

fn extract_nuspec_license(nuspec: &str) -> String {
    if let Some(idx) = nuspec.find("<license")
        && let Some(end) = nuspec[idx..].find("</license>")
    {
        let chunk = &nuspec[idx..idx + end];
        if let Some(content_start) = chunk.find('>') {
            let content = &chunk[content_start + 1..];
            let trimmed = content.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }
    "UNKNOWN".to_string()
}

fn extract_nuspec_description(nuspec: &str) -> Option<String> {
    let start = nuspec.find("<description>")?;
    let end = nuspec[start..].find("</description>")?;
    Some(nuspec[start + 13..start + end].trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn build_fake_nupkg(
        license: &str,
        description: &str,
        xml_filename: &str,
        xml_body: &str,
    ) -> Vec<u8> {
        let mut buf = Vec::new();
        {
            let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
            let opts: zip::write::SimpleFileOptions = Default::default();

            // Top-level .nuspec
            zip.start_file("FakePkg.nuspec", opts).unwrap();
            let nuspec = format!(
                "<?xml version=\"1.0\"?>\n<package><metadata>\n<id>FakePkg</id><version>1.0.0</version>\n<description>{description}</description>\n<license type=\"expression\">{license}</license>\n</metadata></package>\n"
            );
            zip.write_all(nuspec.as_bytes()).unwrap();

            // lib/net6.0/FakePkg.xml — XML doc
            zip.start_file(format!("lib/net6.0/{xml_filename}"), opts)
                .unwrap();
            zip.write_all(xml_body.as_bytes()).unwrap();

            zip.finish().unwrap();
        }
        buf
    }

    #[test]
    fn fetches_and_parses_fake_nupkg() {
        let bytes = build_fake_nupkg(
            "MIT",
            "A test package",
            "FakePkg.xml",
            "<doc><members><member name=\"M:Foo.Bar()\"><summary>Bar method</summary></member></members></doc>",
        );
        let agent = build_from_bytes(&bytes, "FakePkg", "1.0.0", None, false).unwrap();
        assert_eq!(agent.id, "fakepkg");
        assert_eq!(agent.version, "1.0.0");
        assert_eq!(agent.license, "MIT");
        assert_eq!(agent.description, "A test package");
        assert_eq!(agent.skills.len(), 1);
        assert_eq!(agent.skills[0].filename, "fakepkg.md");
        assert!(agent.skills[0].body.contains("Bar method"));
    }

    #[test]
    fn non_permissive_license_blocked_without_accept_license() {
        let bytes = build_fake_nupkg("Proprietary", "x", "x.xml", "<doc/>");
        let err = build_from_bytes(&bytes, "Proprietary", "1.0", None, false).unwrap_err();
        assert!(matches!(err, AwareError::PermissionDenied(_)));
    }

    #[test]
    fn non_permissive_license_accepted_with_flag() {
        let bytes = build_fake_nupkg("Proprietary", "x", "x.xml", "<doc/>");
        let agent = build_from_bytes(&bytes, "Proprietary", "1.0", None, true).unwrap();
        assert_eq!(agent.license, "Proprietary");
    }

    #[test]
    fn missing_at_version_is_validation_error() {
        let err = build_from_nuget("FakePkg", None, false).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn unknown_license_treated_as_non_permissive() {
        let nuspec = "<package><metadata><description>x</description></metadata></package>";
        assert_eq!(extract_nuspec_license(nuspec), "UNKNOWN");
    }
}
