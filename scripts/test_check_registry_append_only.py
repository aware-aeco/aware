"""Regression tests for the published registry contract."""

import importlib.util
import unittest
from pathlib import Path


SCRIPT = Path(__file__).with_name("check_registry_append_only.py")
SPEC = importlib.util.spec_from_file_location("check_registry_append_only", SCRIPT)
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def index(versions):
    return {"agents": {"reader": {"versions": versions}}, "bundles": {}}


class AppendOnlyTests(unittest.TestCase):
    def setUp(self):
        self.original = {"tarball": "https://example.test/commit.tar.gz", "subdir": "archive/reader", "bundle-digest": "sha256:abc"}
        self.base = index({"0.7.3": self.original})

    def test_preserved_release_and_new_version_pass(self):
        candidate = index({"0.7.3": dict(self.original), "0.8.0": {"tarball": "new"}})
        self.assertEqual(MODULE.violations(self.base, candidate), [])

    def test_removed_release_fails(self):
        self.assertIn("reader@0.7.3: published version was removed", MODULE.violations(self.base, index({})))

    def test_repointed_archive_fails(self):
        changed = dict(self.original, tarball="https://example.test/other.tar.gz")
        self.assertIn("reader@0.7.3: published release record was changed", MODULE.violations(self.base, index({"0.7.3": changed})))

    def test_rewritten_digest_fails(self):
        changed = dict(self.original, **{"bundle-digest": "sha256:def"})
        self.assertIn("reader@0.7.3: published release record was changed", MODULE.violations(self.base, index({"0.7.3": changed})))

    MAIN = "https://github.com/aware-aeco/aware/archive/refs/heads/main.tar.gz"
    SHA = "d1338d0da61c4f4592af8ab6238396ce4f1d7a23"

    def main_tracking(self):
        return {
            "tarball": self.MAIN,
            "subdir": "aware-main/20-agents/aeco/engineering/tekla",
            "manifest-agent": "tekla",
            "manifest-version": "0.1.5",
            "bundle-digest": "sha256:abc",
        }

    def pinned(self, **overrides):
        record = dict(
            self.main_tracking(),
            tarball=f"https://github.com/aware-aeco/aware/archive/{self.SHA}.tar.gz",
            subdir=f"aware-{self.SHA}/20-agents/aeco/engineering/tekla",
        )
        record.update(overrides)
        return record

    def test_freezing_a_main_tracking_release_at_a_commit_passes(self):
        base = index({"2025.0.1": self.main_tracking()})
        self.assertEqual(MODULE.violations(base, index({"2025.0.1": self.pinned()})), [])

    def test_a_freeze_that_changes_anything_naming_the_bytes_fails(self):
        base = index({"2025.0.1": self.main_tracking()})
        changed = "reader@2025.0.1: published release record was changed"
        for overrides in (
            {"bundle-digest": "sha256:def"},
            {"manifest-version": "0.1.6"},
            {"manifest-agent": "other"},
            {"subdir": f"aware-{self.SHA}/20-agents/aeco/engineering/other"},
            {"subdir": "aware-main/20-agents/aeco/engineering/tekla"},
            {"tarball": "https://github.com/someone/else/archive/" + self.SHA + ".tar.gz"},
            {"tarball": "https://github.com/aware-aeco/aware/archive/v1.tar.gz"},
        ):
            with self.subTest(overrides=overrides):
                self.assertIn(changed, MODULE.violations(base, index({"2025.0.1": self.pinned(**overrides)})))

    def test_a_pinned_release_can_never_be_repinned(self):
        base = index({"2025.0.1": self.pinned()})
        other = "e" * 40
        moved = self.pinned(
            tarball=f"https://github.com/aware-aeco/aware/archive/{other}.tar.gz",
            subdir=f"aware-{other}/20-agents/aeco/engineering/tekla",
        )
        self.assertIn("reader@2025.0.1: published release record was changed", MODULE.violations(base, index({"2025.0.1": moved})))

    def test_a_main_tracking_release_without_a_digest_cannot_be_frozen(self):
        original = self.main_tracking()
        del original["bundle-digest"]
        candidate = self.pinned()
        del candidate["bundle-digest"]
        self.assertIn(
            "reader@2025.0.1: published release record was changed",
            MODULE.violations(index({"2025.0.1": original}), index({"2025.0.1": candidate})),
        )

    def test_missing_agents_fails_closed(self):
        with self.assertRaises(ValueError):
            MODULE.violations(self.base, {"bundles": {}})


if __name__ == "__main__":
    unittest.main()
