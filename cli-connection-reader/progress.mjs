// The AWARE progress channel, producer side (aware-aeco/aware#405).
//
// The runtime hands a CLI transport a write-only file in `AWARE_PROGRESS_FILE` and mirrors every
// well-formed record it finds there into the run trace as it is written. That is the only way a
// `single`-lifecycle command can say anything before it finishes: stdout is the one JSON result and
// stderr is the failure channel the runtime reports verbatim when a bridge exits non-zero.
//
// ADVISORY, BY CONSTRUCTION. A read that produces correct geometry must not fail because the
// channel is missing, unwritable, or full — the node output remains the authoritative result — so
// every failure here is swallowed. The counterpart guarantee lives in the runtime: a record over
// 8 KiB, or one naming an unsafe artifact id, is dropped rather than mirrored.

import { appendFileSync } from 'node:fs';

const CHANNEL = process.env.AWARE_PROGRESS_FILE || null;

/** Is the runtime listening? Absent means nobody drains the channel — do not pay to describe work. */
export function progressEnabled() {
  return CHANNEL !== null;
}

/**
 * Publish one record. `phase` is required by the runtime (a consumer branches on it), so callers
 * pass at least `{ phase }`.
 *
 * One append per call, not a held file handle: the runtime tails the file by offset and a buffered
 * writer would deliver "I have a batch ready" long after the batch was ready, which is the entire
 * latency this exists to remove. Records are a few hundred bytes and arrive per batch, not per
 * object, so the syscall is not a cost worth optimising against that.
 */
export function emitProgress(record) {
  if (!CHANNEL) return;
  try {
    appendFileSync(CHANNEL, `${JSON.stringify({ '$aware-progress': record })}\n`);
  } catch {
    /* the channel is advisory: never fail a read because progress could not be written */
  }
}
