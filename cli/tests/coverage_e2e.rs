mod common;

use std::path::Path;

#[test]
fn phase_a_e2e_generates_three_artefact_types() {
    // Skip if sidecar binary is absent — this test exercises the BUILT binary,
    // not just the Rust crates. CI runs `dotnet publish` before `cargo test`.
    let sidecar_release =
        Path::new("../cli-sidecar/bin/Release/net10.0/win-x64/publish/aware-sidecar.exe");
    if !sidecar_release.exists() {
        eprintln!(
            "skipping E2E: sidecar binary not found at {}",
            sidecar_release.display()
        );
        return;
    }

    let tmp = tempfile::tempdir().unwrap();
    let ir_fixture =
        std::path::PathBuf::from("../cli-sidecar/Ingest/Generator/Tests/fixtures/minimal.ir.json");
    assert!(
        ir_fixture.exists(),
        "fixture must exist at {}",
        ir_fixture.display()
    );

    let output = tmp.path().join("demo-1.0");
    std::fs::create_dir_all(&output).unwrap();

    // The sidecar uses stdin JSON envelope, not positional args.
    // Envelope shape: {"op": "coverage-generate", "args": {"ir_path": ..., "out_dir": ..., "agent_id": ..., "vendor": ..., "vertical": ...}}
    let envelope = serde_json::json!({
        "op": "coverage-generate",
        "args": {
            "ir_path": ir_fixture.to_string_lossy(),
            "out_dir": output.to_string_lossy(),
            "agent_id": "demo-1.0",
            "vendor": "example",
            "vertical": "engineering"
        }
    });

    use std::io::Write;
    let mut child = std::process::Command::new(sidecar_release)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn sidecar");

    {
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(envelope.to_string().as_bytes()).unwrap();
        stdin.write_all(b"\n").unwrap();
    }

    let result = child
        .wait_with_output()
        .expect("failed to collect sidecar output");
    let stdout = String::from_utf8_lossy(&result.stdout);
    let stderr = String::from_utf8_lossy(&result.stderr);

    assert!(
        result.status.success(),
        "sidecar exit {:?}\nstdout: {}\nstderr: {}",
        result.status.code(),
        stdout,
        stderr
    );

    // Parse the response envelope and verify ok:true + coverage data present.
    let response: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("sidecar stdout not JSON");
    assert_eq!(
        response["ok"],
        serde_json::Value::Bool(true),
        "response: {response}"
    );
    assert!(
        response["data"]["coverage"].is_object(),
        "no coverage data in response"
    );

    // Verify the three artefact types exist on disk.
    assert!(
        output.join("manifest.yaml").is_file(),
        "manifest.yaml not produced at {}",
        output.display()
    );
    assert!(
        output.join("skills").join("example-api.md").is_file(),
        "skills/example-api.md not produced"
    );
    assert!(
        output
            .join("catalog")
            .join("Example.Api.Foo.json")
            .is_file(),
        "catalog/Example.Api.Foo.json not produced"
    );

    // Quick sanity check on manifest content
    let manifest = std::fs::read_to_string(output.join("manifest.yaml")).unwrap();
    assert!(
        manifest.contains("agent:        demo-1.0"),
        "manifest missing agent name"
    );
    assert!(
        manifest.contains("sdk-target:   1.0"),
        "manifest missing sdk-target"
    );
    assert!(
        manifest.contains("foo-do-thing:"),
        "manifest missing the DoThing verb"
    );
}
