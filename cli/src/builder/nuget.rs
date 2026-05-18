//! NuGet package → AWARE agent.
//!
//! Fetches a `.nupkg`, extracts the `.nuspec` (for metadata + license) and
//! the contained DLLs. Prefers `ref/<tfm>/*.dll` (reference assemblies —
//! the vendor's curated public-API contract, metadata-only) and falls back
//! to `lib/<tfm>/*.dll` (runtime DLLs which may include internal types).
//! Picks the best TFM (highest .NET version) and reflects via the C#
//! sidecar — same code path as `--from-dlls`. Packages that ship no DLLs
//! fall back to XML-doc-only skills.

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
    // XML doc files paired with DLLs for the sidecar to pick up as siblings.
    // Also kept as a flat-map fallback for content-only packages with no DLLs.
    let mut xml_docs: BTreeMap<String, String> = BTreeMap::new();
    // Reference assemblies (vendor's curated public-API contract — preferred).
    // Each tfm bucket holds .dll AND .xml files so the sidecar can find the
    // matching XML doc for every DLL via Path.ChangeExtension(".xml").
    let mut ref_tfm_files: BTreeMap<String, Vec<(String, Vec<u8>)>> = BTreeMap::new();
    // Runtime DLLs (fallback — may include internal types).
    let mut lib_tfm_files: BTreeMap<String, Vec<(String, Vec<u8>)>> = BTreeMap::new();
    for i in 0..zip.len() {
        let mut entry = zip
            .by_index(i)
            .map_err(|e| AwareError::Validation(format!("zip entry: {e}")))?;
        let name = entry.name().to_string();
        let lower = name.to_lowercase();
        if name.ends_with(".nuspec") && !name.contains('/') {
            entry
                .read_to_string(&mut nuspec_text)
                .map_err(|e| AwareError::Validation(format!("nuspec read: {e}")))?;
        } else if (lower.starts_with("lib/") || lower.starts_with("ref/"))
            && (lower.ends_with(".dll") || lower.ends_with(".xml"))
        {
            // <root>/<tfm>/<file>.{dll,xml} — bucket by (root, tfm) so DLLs and
            // their sibling XML doc files travel together to the sidecar.
            let parts: Vec<&str> = name.splitn(3, '/').collect();
            if parts.len() == 3 {
                let root = parts[0].to_lowercase();
                let tfm = parts[1].to_lowercase();
                let file = parts[2].to_string();
                let mut bytes_buf = Vec::new();
                entry
                    .read_to_end(&mut bytes_buf)
                    .map_err(|e| AwareError::Validation(format!("read {name}: {e}")))?;
                if lower.ends_with(".xml") {
                    // Also remember XML docs in the flat map for the
                    // DLL-less content-only fallback below.
                    if let Ok(text) = std::str::from_utf8(&bytes_buf) {
                        let basename = name.rsplit('/').next().unwrap_or(&name).to_string();
                        xml_docs.entry(basename).or_insert_with(|| text.to_string());
                    }
                }
                let target = if root == "ref" {
                    &mut ref_tfm_files
                } else {
                    &mut lib_tfm_files
                };
                target.entry(tfm).or_default().push((file, bytes_buf));
            }
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

    // Prefer ref/ DLLs (vendor public-API contract, metadata-only) over
    // lib/ DLLs (runtime, may include internals). Both go through the same
    // PEReader reflection path in the sidecar — but ref/ is the cleaner
    // source. Fall back to XML-doc dumps when no DLLs at all.
    let (commands, skills) = if let Some(best_tfm) = pick_best_tfm(&ref_tfm_files) {
        reflect_tfm_files("ref", &best_tfm, &ref_tfm_files[&best_tfm], pkg, version)?
    } else if let Some(best_tfm) = pick_best_tfm(&lib_tfm_files) {
        reflect_tfm_files("lib", &best_tfm, &lib_tfm_files[&best_tfm], pkg, version)?
    } else {
        let mut xs: Vec<GeneratedSkill> = xml_docs.iter().map(|(name, body)| {
            let stem = name.trim_end_matches(".xml").to_lowercase();
            let skill_name = format!("{stem}.md");
            let skill_body = format!(
                "---\nname: {pkg_lower}-{stem}\ndescription: API reference extracted from {name}\n---\n\n# {stem} API reference\n\n```xml\n{body}\n```\n"
            );
            GeneratedSkill { filename: skill_name, body: skill_body }
        }).collect();
        xs.sort_by(|a, b| a.filename.cmp(&b.filename));
        (BTreeMap::new(), xs)
    };

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
        version: "0.1.0".into(),
        sdk_target: Some(version.to_string()),
        description,
        commands,
        skills,
        provenance,
        stateful: false,
        license,
    })
}

/// Write the chosen TFM's files (DLLs + sibling XML doc files) to a temp dir
/// and route the DLLs through the C# sidecar's reflection path. Writes both
/// DLL and XML files so the sidecar can pair `Foo.dll` with its sibling
/// `Foo.xml` for vendor docstring extraction.
fn reflect_tfm_files(
    root: &str,
    tfm: &str,
    files: &[(String, Vec<u8>)],
    pkg: &str,
    version: &str,
) -> Result<
    (
        BTreeMap<String, crate::builder::GeneratedCommand>,
        Vec<GeneratedSkill>,
    ),
    AwareError,
> {
    let tmp = tempfile::Builder::new()
        .prefix(&format!("aware-nuget-{pkg}-{version}-{root}-{tfm}-"))
        .tempdir()
        .map_err(|e| AwareError::Internal(format!("temp dir: {e}")))?;

    let mut dll_paths: Vec<String> = Vec::new();
    for (filename, bytes) in files {
        let path = tmp.path().join(filename);
        std::fs::write(&path, bytes)
            .map_err(|e| AwareError::Internal(format!("write {filename}: {e}")))?;
        if filename.to_lowercase().ends_with(".dll") {
            dll_paths.push(path.to_string_lossy().to_string());
        }
    }

    let args = crate::sidecar::ReflectDllsArgs {
        globs: dll_paths,
        agent_id: None,
    };
    let reflected = crate::sidecar::reflect_dlls(args)?;
    // tmp is dropped here — auto-cleans on Drop.
    Ok((reflected.commands, reflected.skills))
}

/// Pick the "best" TFM directory from those that ship DLLs.
///
/// Preference: highest `net{N}.0` (net9.0 > net8.0 > …) > netstandard
/// (2.1 > 2.0) > netcoreapp > net4xy. Returns the chosen TFM key.
fn pick_best_tfm(tfm_files: &BTreeMap<String, Vec<(String, Vec<u8>)>>) -> Option<String> {
    tfm_files
        .iter()
        // A bucket without any .dll is a content-only entry (e.g. just .xml
        // doc files) — skip it so we don't ship the sidecar an empty file
        // list. The XML-doc-only fallback in build_from_bytes handles those.
        .filter(|(_, v)| {
            v.iter()
                .any(|(name, _)| name.to_lowercase().ends_with(".dll"))
        })
        .max_by_key(|(tfm, _)| tfm_rank(tfm))
        .map(|(tfm, _)| tfm.clone())
}

fn tfm_rank(tfm: &str) -> i32 {
    let lower = tfm.to_lowercase();
    // net{N}.0 — the modern .NET family. net9.0 = 100090, net5.0 = 100050.
    if let Some(rest) = lower.strip_prefix("net")
        && let Some(idx) = rest.find('.')
        && let Ok(n) = rest[..idx].parse::<i32>()
        && rest[idx..].starts_with(".0")
        && n >= 5
    {
        return 100000 + n * 10;
    }
    // netstandard{X}.{Y} — cross-platform layer. 2.1 = 50021.
    if let Some(rest) = lower.strip_prefix("netstandard")
        && let Some((m, n)) = rest.split_once('.')
        && let (Ok(mi), Ok(ni)) = (m.parse::<i32>(), n.parse::<i32>())
    {
        return 50000 + mi * 10 + ni;
    }
    // netcoreapp{X}.{Y} — pre-net5. 3.1 = 40031.
    if let Some(rest) = lower.strip_prefix("netcoreapp")
        && let Some((m, n)) = rest.split_once('.')
        && let (Ok(mi), Ok(ni)) = (m.parse::<i32>(), n.parse::<i32>())
    {
        return 40000 + mi * 10 + ni;
    }
    // net4XY (.NET Framework). Digit-by-digit: net48 = 4.8 = 34080,
    // net472 = 4.7.2 = 34072 (so net48 > net472 holds semantically).
    if let Some(rest) = lower.strip_prefix("net")
        && rest.starts_with('4')
        && rest.chars().all(|c| c.is_ascii_digit())
    {
        let digits: Vec<i32> = rest
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        let major = digits.first().copied().unwrap_or(0);
        let minor = digits.get(1).copied().unwrap_or(0);
        let patch = digits.get(2).copied().unwrap_or(0);
        return 30000 + major * 1000 + minor * 10 + patch;
    }
    // net2x/net3x — very old. net20 = 22000, net35 = 23050.
    if let Some(rest) = lower.strip_prefix("net")
        && (rest.starts_with('2') || rest.starts_with('3'))
        && rest.chars().all(|c| c.is_ascii_digit())
    {
        let digits: Vec<i32> = rest
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        let major = digits.first().copied().unwrap_or(0);
        let minor = digits.get(1).copied().unwrap_or(0);
        return 20000 + major * 1000 + minor * 10;
    }
    0
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
        assert_eq!(agent.version, "0.1.0");
        assert_eq!(agent.sdk_target.as_deref(), Some("1.0.0"));
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

    #[test]
    fn tfm_rank_prefers_modern_net_over_framework() {
        assert!(tfm_rank("net9.0") > tfm_rank("net8.0"));
        assert!(tfm_rank("net8.0") > tfm_rank("net6.0"));
        assert!(tfm_rank("net6.0") > tfm_rank("netstandard2.1"));
        assert!(tfm_rank("netstandard2.1") > tfm_rank("netstandard2.0"));
        assert!(tfm_rank("netstandard2.0") > tfm_rank("netcoreapp3.1"));
        assert!(tfm_rank("netcoreapp3.1") > tfm_rank("net48"));
        assert!(tfm_rank("net48") > tfm_rank("net472"));
        assert!(tfm_rank("net472") > tfm_rank("net45"));
        assert!(tfm_rank("net45") > tfm_rank("garbage"));
    }

    #[test]
    fn pick_best_tfm_chooses_highest_net() {
        let mut tfms: BTreeMap<String, Vec<(String, Vec<u8>)>> = BTreeMap::new();
        tfms.insert("net48".into(), vec![("a.dll".into(), vec![1])]);
        tfms.insert("netstandard2.0".into(), vec![("a.dll".into(), vec![1])]);
        tfms.insert("net8.0".into(), vec![("a.dll".into(), vec![1])]);
        assert_eq!(pick_best_tfm(&tfms).as_deref(), Some("net8.0"));
    }

    #[test]
    fn pick_best_tfm_skips_empty_buckets() {
        let mut tfms: BTreeMap<String, Vec<(String, Vec<u8>)>> = BTreeMap::new();
        tfms.insert("net9.0".into(), vec![]); // empty
        tfms.insert("net48".into(), vec![("a.dll".into(), vec![1])]);
        assert_eq!(pick_best_tfm(&tfms).as_deref(), Some("net48"));
    }

    #[test]
    fn pick_best_tfm_returns_none_for_empty_map() {
        let tfms: BTreeMap<String, Vec<(String, Vec<u8>)>> = BTreeMap::new();
        assert!(pick_best_tfm(&tfms).is_none());
    }
}
