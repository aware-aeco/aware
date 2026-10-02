#!/usr/bin/env python3
"""Reject removal or rewriting of published registry id@version entries.

Compare the candidate index with the index at the PR base (or the previous
main tip on a push). A new version is free to be added, but an existing
version's complete release record is an immutable install contract.
"""

import argparse
import json
import re
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


MAIN_TARBALL = "https://github.com/aware-aeco/aware/archive/refs/heads/main.tar.gz"
COMMIT_TARBALL = re.compile(r"^https://github\.com/aware-aeco/aware/archive/([0-9a-f]{40})\.tar\.gz$")


def is_freezing_repin(original: object, current: object) -> bool:
    """A main-tracking release re-pinned, byte for byte, to an immutable commit.

    A record whose tarball is `refs/heads/main` was never immutable: any change to
    its agent folder on main changes the bytes it installs, so the record cannot
    stay valid and the agent can never change. This admits exactly one rewrite of
    such a record — freezing it at a commit — and only when everything that names
    the bytes stays identical: the same `bundle-digest` (which must be present),
    `manifest-agent` and `manifest-version`, and the same path inside the archive.
    `aware agent reindex --check` then proves the pinned commit's tree hashes to
    that digest, so the install contract is unchanged; it simply stops drifting.
    """
    if not isinstance(original, dict) or not isinstance(current, dict):
        return False
    if original.get("tarball") != MAIN_TARBALL or not original.get("bundle-digest"):
        return False
    match = COMMIT_TARBALL.match(str(current.get("tarball", "")))
    if not match:
        return False
    original_subdir = str(original.get("subdir", ""))
    if not original_subdir.startswith("aware-main/"):
        return False
    expected_subdir = f"aware-{match.group(1)}/" + original_subdir[len("aware-main/"):]
    if current.get("subdir") != expected_subdir:
        return False
    rest = lambda record: {k: v for k, v in record.items() if k not in ("tarball", "subdir")}
    return rest(original) == rest(current)


def violations(base: object, candidate: object) -> list[str]:
    published = releases(base, "base registry-index.json")
    current = releases(candidate, "candidate registry-index.json")
    errors = []
    for key, original in sorted(published.items()):
        label = "@".join(key)
        if key not in current:
            errors.append(f"{label}: published version was removed")
        elif current[key] != original and not is_freezing_repin(original, current[key]):
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
