// Shared payload contract for postinstall.js and bin/aware.js (#287).
//
// The files without which the CLI (or its aware-roslyn companion host) cannot
// start. aware-roslyn ships as a self-contained FOLDER (hundreds of framework
// DLLs); exhaustively manifesting it is disproportionate here — these four are
// the startup-critical set (apphost, app dll, runtime config, deps graph), and
// the extraction paths themselves fail loudly on any entry error, so a payload
// passing this check with an arbitrary missing middle DLL requires the
// extractor to have lied about its exit code.
const path = require('path');

// Agent transport binaries bundled in the archive. The CLI resolves these beside the
// running `aware`, so a payload missing them extracts "successfully" and then fails at
// run time on every steel-detailer lookup — the exact silent-partial-extraction case
// this contract exists to catch. Safe to require: the npm package version and the
// release archive it downloads are always the same version, so these are present in
// every archive this shim can fetch.
const TRANSPORTS = [
  'aware-steel-detailer-us',
  'aware-steel-detailer-uk',
  'aware-steel-detailer-eu',
];

function requiredFiles(exeSuffix) {
  return [
    `aware${exeSuffix}`,
    `aware-sidecar${exeSuffix}`,
    ...TRANSPORTS.map((t) => `${t}${exeSuffix}`),
    path.join('aware-roslyn', `aware-roslyn${exeSuffix}`),
    path.join('aware-roslyn', 'aware-roslyn.dll'),
    path.join('aware-roslyn', 'aware-roslyn.runtimeconfig.json'),
    path.join('aware-roslyn', 'aware-roslyn.deps.json'),
  ];
}

module.exports = { requiredFiles, TRANSPORTS };
