use assert_cmd::Command;

fn cli_path() -> std::path::PathBuf {
    std::env::var_os("AWARE_GEOMETRY_TEST_CLI")
        .unwrap_or_else(|| env!("CARGO_BIN_EXE_aware").into())
        .into()
}

#[cfg(all(feature = "geometry-dev", windows, target_arch = "x86_64"))]
mod development {
    use super::*;
    use serde_json::{Value, json};

    fn fixture() -> Value {
        let operand = |id| {
            json!({
                "id": id, "offset_mm": [0, 0, 0], "brep_b64": null,
                "profile": {"xy_mm": [["0", "0"], ["1", "0"], ["0", "1"]], "z_mm": ["0", "1"]},
                "mesh": {"positions_mm": [[0, 0, 0]], "triangles": []}
            })
        };
        json!({"schema": "aware.geometry.exec.request/v1", "op": "boolean", "kind": "difference",
            "frame": {"units": "mm", "origin_mm": [0, 0, 0], "axes": [[1, 0, 0], [0, 1, 0], [0, 0, 1]]},
            "chord_height_mm": 0.01, "a": operand("plate"), "b": operand("clip")})
    }

    fn error(input: Vec<u8>, file: bool, expected: &str) {
        let home = tempfile::tempdir().unwrap();
        let mut cmd = Command::new(cli_path());
        cmd.env("AWARE_HOME", home.path())
            .args(["geometry", "exec", "--json"]);
        if file {
            let path = home.path().join("request.json");
            std::fs::write(&path, input).unwrap();
            cmd.arg(format!("@{}", path.display()));
        } else {
            cmd.arg("-").write_stdin(input);
        }
        let output = cmd.output().unwrap();
        assert!(!output.status.success());
        let reply: Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|err| {
            panic!(
                "expected CLI error envelope: {err}; stderr={}",
                String::from_utf8_lossy(&output.stderr)
            )
        });
        assert_eq!(reply["ok"], false);
        assert_eq!(reply["error"]["code"], "E_GEOMETRY_REQUEST");
        assert!(
            reply["error"]["message"]
                .as_str()
                .unwrap()
                .contains(expected),
            "{reply}"
        );
    }

    #[test]
    fn omitted_brep_b64() {
        let mut request = fixture();
        request["a"].as_object_mut().unwrap().remove("brep_b64");
        error(request.to_string().into_bytes(), false, "brep_b64");
    }

    #[test]
    fn missing_operand_id() {
        let mut request = fixture();
        request["b"].as_object_mut().unwrap().remove("id");
        error(request.to_string().into_bytes(), false, "`id`");
    }

    #[test]
    fn unknown_request_fields() {
        let mut request = fixture();
        request["extra"] = json!(true);
        error(request.to_string().into_bytes(), false, "unknown field");
        let mut request = fixture();
        request["a"]["mesh"]["extra"] = json!(true);
        error(request.to_string().into_bytes(), true, "unknown field");
    }

    #[test]
    fn bad_frame() {
        let mut request = fixture();
        request["frame"]["units"] = json!("in");
        error(request.to_string().into_bytes(), false, "units");
        let mut request = fixture();
        request["frame"]["axes"][2] = json!([0, 0, -1]);
        error(request.to_string().into_bytes(), false, "right-handed");
    }

    #[test]
    fn oversized_request_file_and_stdin() {
        for file in [false, true] {
            error(vec![b' '; 32 * 1024 * 1024 + 1], file, "32 MiB");
        }
    }

    #[test]
    fn development_command_hidden() {
        let output = Command::new(cli_path()).arg("--help").output().unwrap();
        assert!(output.status.success());
        assert!(
            !String::from_utf8(output.stdout)
                .unwrap()
                .contains("geometry")
        );
        let output = Command::new(cli_path())
            .args(["geometry", "--help"])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn mesh_exhausted() {
        let home = tempfile::tempdir().unwrap();
        let standalone = home.path().join("aware.exe");
        std::fs::copy(cli_path(), &standalone).unwrap();
        let output = Command::new(standalone)
            .env("AWARE_HOME", home.path())
            .args(["geometry", "exec", "-", "--json"])
            .write_stdin(fixture().to_string())
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stderr.is_empty());
        assert!(output.stdout.ends_with(b"\n") && output.stdout.len() <= 32 * 1024 * 1024);
        let reply: Value = serde_json::from_slice(&output.stdout).unwrap();
        let data = reply["data"].as_object().unwrap();
        assert_eq!(data["status"], "unavailable");
        assert_eq!(data["exact_status"], "unavailable");
        assert_eq!(data["result"], Value::Null);
        assert_eq!(data["reason_code"], "mesh_worker_failed");
        assert_eq!(data["operand_ids"], json!({"a": "plate", "b": "clip"}));
    }

    fn install_fixture() -> (tempfile::TempDir, std::path::PathBuf, std::path::PathBuf) {
        let home = tempfile::tempdir().unwrap();
        let python =
            std::env::var_os("AWARE_GEOMETRY_TEST_PYTHON").expect("absolute base Python path");
        let wheels =
            std::env::var_os("AWARE_GEOMETRY_TEST_WHEELHOUSE").expect("verified wheelhouse path");
        (home, python.into(), wheels.into())
    }

    fn install(
        home: &std::path::Path,
        python: &std::path::Path,
        wheels: &std::path::Path,
    ) -> std::process::Output {
        Command::new(cli_path())
            .env("AWARE_HOME", home)
            .args(["geometry", "install", "--json", "--occt", "--python"])
            .arg(python)
            .arg("--wheelhouse")
            .arg(wheels)
            .output()
            .unwrap()
    }

    #[test]
    #[ignore = "requires explicit test Python and wheelhouse paths"]
    fn local_lock_install() {
        let (home, python, wheels) = install_fixture();
        let output = install(home.path(), &python, &wheels);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stderr.is_empty());
        let envelope: Value = serde_json::from_slice(&output.stdout).unwrap();
        let root = std::path::Path::new(envelope["data"]["path"].as_str().unwrap());
        assert!(root.starts_with(home.path()));
        assert!(root.join("complete.json").is_file());
        assert!(root.join("licenses/LICENSE_LGPL_21.txt").is_file());
        assert!(root.join("licenses/OCCT_LGPL_EXCEPTION.txt").is_file());
        assert!(root.join("licenses/OCP-LICENSE.txt").is_file());
        assert!(root.join("licenses/VTK-LICENSE.txt").is_file());
        assert!(root.join("wheel-manifest.json").is_file());
        assert_eq!(envelope["data"]["probe"]["native_fuzzy_floor_mm"], 1e-7);
    }

    #[test]
    #[ignore = "requires explicit test Python and wheelhouse paths"]
    fn offline_bad_hash() {
        let (home, python, _) = install_fixture();
        let bad = tempfile::tempdir().unwrap();
        std::fs::write(
            bad.path()
                .join("cadquery_ocp-8.0.1.1.0-cp312-cp312-win_amd64.whl"),
            b"modified wheel",
        )
        .unwrap();
        let output = install(home.path(), &python, bad.path());
        assert!(!output.status.success());
        let envelope: Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
            panic!(
                "{error}; stderr={}",
                String::from_utf8_lossy(&output.stderr)
            )
        });
        assert!(
            envelope["error"]["message"]
                .as_str()
                .unwrap()
                .contains("wheel digest mismatch")
        );
        let platform = home.path().join("runtimes/occt/cp312-win_amd64");
        assert!(
            platform.read_dir().unwrap().all(|entry| !entry
                .unwrap()
                .path()
                .join("complete.json")
                .exists())
        );
    }

    #[test]
    #[ignore = "requires a staged mesh helper and a producer Python"]
    fn no_runtime_flush() {
        let home = tempfile::tempdir().unwrap();
        let staged =
            std::env::var_os("AWARE_GEOMETRY_TEST_STAGE").expect("staged development directory");
        let binary = std::path::Path::new(&staged).join("aware.exe");
        std::fs::copy(env!("CARGO_BIN_EXE_aware"), &binary).unwrap();
        let python =
            std::env::var_os("AWARE_GEOMETRY_TEST_PRODUCER").expect("locked producer Python");
        let output = Command::new(python)
            .arg("-I")
            .arg(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../geometry-worker/fixtures/generate_difference.py"),
            )
            .arg("flush_stress")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let output = Command::new(binary)
            .env("AWARE_HOME", home.path())
            .env_remove("PATH")
            .args(["geometry", "exec", "-", "--json"])
            .write_stdin(output.stdout)
            .output()
            .unwrap();
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
        assert!(output.stdout.ends_with(b"\n") && output.stdout.len() <= 32 * 1024 * 1024);
        let reply: Value = serde_json::from_slice(&output.stdout).unwrap();
        let data = &reply["data"];
        assert_eq!(data["status"], "mesh_fallback", "{reply}");
        assert_eq!(data["exact_status"], "unavailable");
        assert_eq!(data["warnings"][0]["code"], "occt_runtime_missing");
        assert_eq!(data["warnings"][1]["code"], "chord_height_unverified");
        assert_eq!(data["result"]["brep_b64"], Value::Null);
        assert!(
            !data["result"]["mesh"]["triangles"]
                .as_array()
                .unwrap()
                .is_empty()
        );
        assert!((data["result"]["volume_mm3"].as_f64().unwrap() - 1_973_000.).abs() < 1e-8);
    }

    #[test]
    #[ignore = "requires a staged mesh helper, a producer Python, and a verified wheelhouse"]
    fn flush_origin() {
        let (home, base, wheels) = install_fixture();
        let installed = install(home.path(), &base, &wheels);
        assert!(
            installed.status.success(),
            "{}",
            String::from_utf8_lossy(&installed.stderr)
        );
        let staged =
            std::env::var_os("AWARE_GEOMETRY_TEST_STAGE").expect("staged development directory");
        let binary = std::path::Path::new(&staged).join("aware.exe");
        std::fs::copy(env!("CARGO_BIN_EXE_aware"), &binary).unwrap();
        let python =
            std::env::var_os("AWARE_GEOMETRY_TEST_PRODUCER").expect("locked producer Python");
        let request = Command::new(python)
            .arg("-I")
            .arg(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../geometry-worker/fixtures/generate_difference.py"),
            )
            .arg("flush_origin")
            .output()
            .unwrap();
        assert!(
            request.status.success(),
            "{}",
            String::from_utf8_lossy(&request.stderr)
        );
        let path = home.path().join("request.json");
        std::fs::write(&path, request.stdout).unwrap();
        let output = Command::new(binary)
            .env("AWARE_HOME", home.path())
            .env_remove("PATH")
            .args([
                "geometry",
                "exec",
                "--json",
                &format!("@{}", path.display()),
            ])
            .output()
            .unwrap();
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
        assert!(output.stdout.ends_with(b"\n") && output.stdout.len() <= 32 * 1024 * 1024);
        let reply: Value = serde_json::from_slice(&output.stdout).unwrap();
        let data = &reply["data"];
        assert_eq!(data["status"], "validated_brep", "{reply}");
        assert_eq!(data["exact_status"], "validated");
        assert!(
            data["result"]["brep_b64"]
                .as_str()
                .is_some_and(|s| !s.is_empty())
        );
        assert!((data["result"]["volume_mm3"].as_f64().unwrap() - 1_973_000.).abs() < 1e-8);
    }
}

#[cfg(not(feature = "geometry-dev"))]
#[test]
fn release_command_absent() {
    Command::new(cli_path())
        .args(["geometry", "--help"])
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "unrecognized subcommand 'geometry'",
        ));
}
