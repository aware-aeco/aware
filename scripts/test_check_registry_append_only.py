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

    def test_missing_agents_fails_closed(self):
        with self.assertRaises(ValueError):
            MODULE.violations(self.base, {"bundles": {}})


if __name__ == "__main__":
    unittest.main()
