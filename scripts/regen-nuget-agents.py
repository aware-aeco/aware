"""
Regenerate the 8 NuGet-sourced reflected agents with enriched XML doc
descriptions (post-v0.6.2 fix), preserving each agent's curated manifest
frontmatter (display-name, vendor, license, homepage, keywords, …).

Strategy per agent:
  1. Run `aware build agent --from-nuget <pkg>@<version> --output <id>` into
     a scratch AWARE_HOME.
  2. Replace `commands/` and `skills/` subdirs in the repo location.
  3. Splice the fresh `commands:` and `skills:` blocks into the existing
     manifest.yaml — keeping everything above `provenance:` (display-name,
     vendor, license, keywords, …) and refreshing `provenance:`,
     `transport:`, `commands:`, `skills:` from the regenerated manifest.

Run from the repo root:
  python scripts/regen-nuget-agents.py
"""

import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent
AWARE_EXE = REPO_ROOT / "cli" / "target" / "release" / "aware.exe"
SIDECAR_EXE = REPO_ROOT / "cli-sidecar" / "bin" / "Debug" / "net9.0" / "aware-sidecar.exe"

# (agent-id, nuget-pkg@version, repo-path)
AGENTS = [
    ("tekla-2025",     "Tekla.Structures.Model@2025.0.0",   "20-agents/aeco/engineering/tekla-2025"),
    ("tekla-2026",     "Tekla.Structures.Model@2026.0.3",   "20-agents/aeco/engineering/tekla-2026"),
    ("revit-2025",     "Autodesk.Revit.SDK@2025.0.2.419",   "20-agents/aeco/architecture/revit-2025"),
    ("revit-2026",     "Autodesk.Revit.SDK@2026.0.0.9999",  "20-agents/aeco/architecture/revit-2026"),
    ("rhino-7",        "RhinoCommon@7.38.24338.17001",      "20-agents/aeco/architecture/rhino-7"),
    ("rhino-8",        "RhinoCommon@8.31.26126.13431",      "20-agents/aeco/architecture/rhino-8"),
    ("grasshopper-7",  "Grasshopper@7.38.24338.17001",      "20-agents/aeco/architecture/grasshopper-7"),
    ("grasshopper-8",  "Grasshopper@8.31.26126.13431",      "20-agents/aeco/architecture/grasshopper-8"),
]


def splice_manifest(curated: str, fresh: str) -> str:
    """Keep top-of-file (curated fields) from `curated`, swap `provenance:`
    through end-of-file with the matching tail from `fresh`. Splits at the
    line `provenance:` which appears in both."""
    def split_at_provenance(text: str) -> tuple[str, str]:
        marker = "\nprovenance:"
        idx = text.find(marker)
        if idx == -1:
            raise RuntimeError("manifest missing 'provenance:' marker")
        return text[: idx + 1], text[idx + 1 :]  # include the trailing newline in head

    head, _ = split_at_provenance(curated)
    _, tail = split_at_provenance(fresh)
    if not head.endswith("\n"):
        head += "\n"
    return head + tail


def regen_one(agent_id: str, nuget_spec: str, repo_path: str) -> None:
    print(f"=== {agent_id} ({nuget_spec}) ===", flush=True)
    repo_dir = REPO_ROOT / repo_path
    if not (repo_dir / "manifest.yaml").is_file():
        raise RuntimeError(f"no curated manifest at {repo_dir / 'manifest.yaml'}")
    curated_manifest = (repo_dir / "manifest.yaml").read_text(encoding="utf-8")

    with tempfile.TemporaryDirectory(prefix=f"aware-regen-{agent_id}-") as scratch:
        env = {
            **os.environ,
            "AWARE_HOME": scratch,
            "AWARE_SIDECAR": str(SIDECAR_EXE),
        }
        cmd = [
            str(AWARE_EXE),
            "build",
            "agent",
            "--from-nuget",
            nuget_spec,
            "--accept-license",
            "--output",
            agent_id,
        ]
        result = subprocess.run(cmd, env=env, capture_output=True, text=True)
        if result.returncode != 0:
            raise RuntimeError(f"aware build failed for {agent_id}: {result.stderr}")
        print(result.stdout.strip(), flush=True)

        fresh_dir = Path(scratch) / "agents" / agent_id
        if not fresh_dir.is_dir():
            raise RuntimeError(f"fresh agent missing at {fresh_dir}")

        # 1. Replace commands/ and skills/ in the repo
        for sub in ("commands", "skills"):
            target = repo_dir / sub
            src = fresh_dir / sub
            if target.exists():
                shutil.rmtree(target)
            if src.exists():
                shutil.copytree(src, target)

        # 2. Splice manifest: keep curated head, take fresh provenance: through end
        fresh_manifest = (fresh_dir / "manifest.yaml").read_text(encoding="utf-8")
        spliced = splice_manifest(curated_manifest, fresh_manifest)
        (repo_dir / "manifest.yaml").write_text(spliced, encoding="utf-8")


def main():
    if not AWARE_EXE.exists():
        sys.exit(f"missing release binary at {AWARE_EXE}; run `cargo build --release`")
    if not SIDECAR_EXE.exists():
        sys.exit(f"missing sidecar at {SIDECAR_EXE}; run `dotnet build cli-sidecar`")
    for agent_id, nuget_spec, repo_path in AGENTS:
        regen_one(agent_id, nuget_spec, repo_path)
    print("=== done ===")


if __name__ == "__main__":
    main()
