mod common;

use assert_cmd::Command;
use predicates::prelude::*;

/// Locate a built `aware-roslyn` binary (debug build is fine — we only read source metadata).
fn roslyn_binary() -> Option<std::path::PathBuf> {
    let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    for cfg in ["Debug", "Release"] {
        for name in ["aware-roslyn.exe", "aware-roslyn"] {
            let candidate = repo_root.join(format!("cli-roslyn/bin/{cfg}/net10.0/{name}"));
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    // Release self-contained publish layouts.
    for rid in ["win-x64", "linux-x64", "osx-arm64", "osx-x64"] {
        for name in ["aware-roslyn.exe", "aware-roslyn"] {
            let candidate = repo_root.join(format!(
                "cli-roslyn/bin/Release/net10.0/{rid}/publish/{name}"
            ));
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

#[test]
fn roslyn_e2e_reflects_csharp_source_into_agent() {
    let Some(roslyn) = roslyn_binary() else {
        eprintln!("[skip] aware-roslyn binary not found; run `dotnet build cli-roslyn`");
        return;
    };

    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let src = tmp.path().join("Greeter.cs");
    std::fs::write(
        &src,
        "namespace Demo {\n  /// <summary>Greets people.</summary>\n  public class Greeter {\n    /// <summary>Say hi to the given name.</summary>\n    public string Greet(string name) => name;\n  }\n}\n",
    )
    .unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_ROSLYN", &roslyn)
        .args(["build", "agent", "--from-csharp"])
        .arg(&src)
        .args(["--output", "e2e-csharp"])
        .assert()
        .success()
        .stdout(predicate::str::contains("generated e2e-csharp"));

    let manifest_path = aware.join("agents/e2e-csharp/manifest.yaml");
    assert!(manifest_path.is_file(), "manifest not written");
    let manifest = std::fs::read_to_string(&manifest_path).unwrap();
    assert!(
        manifest.contains("greeter-greet"),
        "manifest did not contain expected Greeter command:\n{manifest}"
    );
}

/// The Tekla recipe fires on SOURCE too (#180): a self-contained .cs declaring the
/// Tekla-shaped graph (PluginBase + [Plugin] + [StructuresField] + a plug-in taking its data
/// contract via ctor) yields the same `insert-demo-plugin` command as the compiled path.
#[test]
fn roslyn_e2e_tekla_recipe_on_source() {
    let Some(roslyn) = roslyn_binary() else {
        eprintln!("[skip] aware-roslyn binary not found");
        return;
    };

    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let src = tmp.path().join("Plugin.cs");
    std::fs::write(
        &src,
        r#"
namespace Tekla.Structures.Plugins {
    public abstract class PluginBase { public abstract bool Run(System.Collections.Generic.List<object> i); }
    [System.AttributeUsage(System.AttributeTargets.Class)]
    public sealed class PluginAttribute : System.Attribute { public PluginAttribute(string name){Name=name;} public string Name {get;} }
    [System.AttributeUsage(System.AttributeTargets.Field)]
    public sealed class StructuresFieldAttribute : System.Attribute { public StructuresFieldAttribute(string n){AttributeName=n;} public string AttributeName {get;} }
}
namespace App {
    public class WidgetData {
        [Tekla.Structures.Plugins.StructuresField("Width")] public double Width;
        [Tekla.Structures.Plugins.StructuresField("Label")] public string Label = "";
    }
    [Tekla.Structures.Plugins.Plugin("Widget Maker")]
    public class WidgetPlugin : Tekla.Structures.Plugins.PluginBase {
        public WidgetPlugin(WidgetData d) { Data = d; }
        public WidgetData Data { get; }
        public override bool Run(System.Collections.Generic.List<object> i) => true;
    }
}
"#,
    )
    .unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_ROSLYN", &roslyn)
        .args(["build", "agent", "--from-csharp"])
        .arg(&src)
        .args(["--output", "e2e-src-tekla"])
        .assert()
        .success();

    let manifest =
        std::fs::read_to_string(aware.join("agents/e2e-src-tekla/manifest.yaml")).unwrap();
    assert!(
        manifest.contains("insert-widget-maker"),
        "source recipe did not emit insert command:\n{manifest}"
    );
    assert!(
        !manifest.contains("widget-plugin-run"),
        "per-method command should be replaced:\n{manifest}"
    );
    for field in ["width", "label", "input-points"] {
        assert!(
            manifest.contains(field),
            "manifest missing input '{field}':\n{manifest}"
        );
    }
}

#[test]
fn roslyn_missing_binary_returns_not_found() {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("Foo.cs");
    std::fs::write(&src, "public class Foo {}\n").unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env(
            "AWARE_ROSLYN",
            "C:/this-binary-does-not-exist-12345-aware-roslyn.exe",
        )
        .args(["build", "agent", "--from-csharp"])
        .arg(&src)
        .assert()
        .failure(); // NotFound — roslyn reader missing
}
