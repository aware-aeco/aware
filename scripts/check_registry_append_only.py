#!/usr/bin/env python3
"""Reject removal or rewriting of published registry id@version entries.

Compare the candidate index with the index at the PR base (or the previous
main tip on a push). A new version is free to be added, but an existing
version's complete release record is an immutable install contract.
"""

import argparse
import json
import subprocess
import sys
from pathlib import Path


def releases(document: object, source: str) -> dict[tuple[str, str], object]:
    if not isinstance(document, dict) or not isinstance(document.get("agents"), dict):
        raise ValueError(f"{source}: expected an agents object")
    result = {}
    for agent_id, agent in document["agents"].items():
        if not isinstance(agent_id, str) or not isinstance(agent, dict):
            raise ValueError(f"{source}: invalid agent entry {agent_id!r}")
        versions = agent.get("versions")
        if not isinstance(versions, dict):
            raise ValueError(f"{source}: {agent_id} has no versions object")
        for version, release in versions.items():
            if not isinstance(version, str) or not isinstance(release, dict):
                raise ValueError(f"{source}: invalid release {agent_id}@{version}")
            result[(agent_id, version)] = release
    return result


def violations(base: object, candidate: object) -> list[str]:
    published = releases(base, "base registry-index.json")
    current = releases(candidate, "candidate registry-index.json")
    errors = []
    for key, original in sorted(published.items()):
        label = "@".join(key)
        if key not in current:
            errors.append(f"{label}: published version was removed")
        elif current[key] != original:
            errors.append(f"{label}: published release record was changed")
    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base-ref", required=True, help="Git commit/ref before this change")
    parser.add_argument(
        "--index", type=Path, default=Path("registry-index.json"), help="candidate index path"
    )
    args = parser.parse_args()
    try:
        base_bytes = subprocess.run(
            ["git", "show", f"{args.base_ref}:registry-index.json"],
            check=True,
            capture_output=True,
        ).stdout
        base = json.loads(base_bytes)
        candidate = json.loads(args.index.read_bytes())
        errors = violations(base, candidate)
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        print(f"registry append-only check could not run: {error}", file=sys.stderr)
        return 2
    if errors:
        for error in errors:
            print(f"registry append-only violation: {error}", file=sys.stderr)
        return 1
    print("registry published id@version entries are unchanged")
    return 0


if __name__ == "__main__":
    sys.exit(main())
