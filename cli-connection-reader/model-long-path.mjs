import path from 'node:path';

const VERBATIM_UNC = '\\\\?\\UNC\\';
const VERBATIM = '\\\\?\\';

// Node's `fs.mkdtemp` is one of the few fs calls that does not pass its path through
// `path.toNamespacedPath`, and `CreateDirectoryW` without the `\\?\` prefix stops at 248
// UTF-16 units whatever `LongPathsEnabled` says. Under a deep AWARE_HOME every `mkdir`,
// `open` and `rm` around it succeeded while the scratch directory itself failed with ENOENT
// (#593). The prefix is namespaced for the call and stripped from the result, so callers keep
// the plain path they joined from — every other fs call namespaces it again on its own.
export async function mkdtempBeyondMaxPath(io, prefix, platform = process.platform) {
  if (platform !== 'win32') return io.mkdtemp(prefix);
  const created = await io.mkdtemp(path.win32.toNamespacedPath(prefix));
  if (prefix.startsWith(VERBATIM)) return created;
  if (created.startsWith(VERBATIM_UNC)) return `\\\\${created.slice(VERBATIM_UNC.length)}`;
  if (created.startsWith(VERBATIM)) return created.slice(VERBATIM.length);
  return created;
}
