"""Exercise release-zip installation without touching the user's Revit add-ins."""

import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


@unittest.skipUnless(os.name == "nt" and shutil.which("pwsh"), "Windows PowerShell required")
class RevitAddinInstallerTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="aware-revit-installer-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.bridge = self.root / "bridges"
        self.bridge.mkdir()
        source = Path(__file__).resolve().parents[1] / "cli-revit"
        shutil.copyfile(source / "install-addin.ps1", self.bridge / "install-addin.ps1")
        self.manifest = source / "AwareRevit" / "AwareRevit.addin"
        self.env = dict(os.environ, APPDATA=str(self.root / "appdata"))

    def payload(self, folder, year):
        path = self.bridge / "AwareRevit" / folder
        path.mkdir(parents=True, exist_ok=True)
        (path / "AwareRevit.dll").write_bytes(b"test add-in payload")
        (path / "aware-revit-version.txt").write_text(year)
        shutil.copyfile(self.manifest, path / "AwareRevit.addin")

    def install(self, year):
        return subprocess.run(
            ["pwsh", "-NoProfile", "-File", str(self.bridge / "install-addin.ps1"),
             "-RevitVersion", year],
            env=self.env, capture_output=True, text=True, timeout=30,
        )

    def test_installs_matching_2027_payload_and_rewrites_manifest(self):
        self.payload("2027", "2027")
        result = self.install("2027")
        self.assertEqual(0, result.returncode, result.stderr)
        target = self.root / "appdata/Autodesk/Revit/Addins/2027"
        dll = target / "AwareRevit/AwareRevit.dll"
        self.assertEqual(b"test add-in payload", dll.read_bytes())
        self.assertIn(str(dll), (target / "AwareRevit.addin").read_text(encoding="utf-8-sig"))

    def test_refuses_2026_payload_labelled_as_2027(self):
        self.payload("2027", "2026")
        result = self.install("2027")
        self.assertNotEqual(0, result.returncode)
        self.assertIn("targets Revit 2026", result.stderr)
        self.assertFalse((self.root / "appdata/Autodesk").exists())

    def test_legacy_2026_payload_cannot_satisfy_2027(self):
        self.payload("", "2026")
        result = self.install("2027")
        self.assertNotEqual(0, result.returncode)
        self.assertFalse((self.root / "appdata/Autodesk").exists())


if __name__ == "__main__":
    unittest.main()
