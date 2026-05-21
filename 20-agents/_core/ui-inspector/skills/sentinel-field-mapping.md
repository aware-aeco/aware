---
name: ui-inspector-sentinel-field-mapping
description: This skill should be used when implementing the ui-inspector's map-fields command — recovering which named setting each opaque dialog field controls, with no per-app field list. Encodes the sentinelization trick (write known sentinel values into a preset copy, re-read the dialog, match visible text back to attributes) including the numeric-stride and token strategies and their matching rules.
---

# Sentinel field mapping

`map-fields` answers: *"this dialog has 200 nameless edit boxes — which named
setting does each one control?"* — without a hand-maintained per-app field list.

## The trick

A dialog that loads a **preset** (a saved settings file / attribute set) is a
function from named attributes to on-screen field values. Invert it empirically:

1. **Copy** the preset (never touch the user's original).
2. **Sentinelize** the copy: replace each attribute's value with a *unique,
   recognizable* sentinel derived from the attribute.
3. **Load** the sentinelized copy into the dialog via the caller-supplied
   `load-via` trigger (a control click, menu item, or host-agent verb). This is
   the one **host-specific** step — every app loads presets through different UI —
   so the caller, who knows the host, provides the trigger; the sidecar invokes it.
4. **Enumerate + read** every field (see
   [win32-control-enumeration](win32-control-enumeration.md)).
5. **Match** each visible field value back to the sentinel that produced it →
   `field-id -> attribute`.

Because every sentinel is unique and traceable to its attribute, a field showing
that sentinel *is* that attribute. The *matching* needs no app-specific knowledge
— it works for any app that round-trips settings through a dialog. Only the load
step (3) is host-specific, and the caller supplies it via `load-via`.

## Two strategies

### `numeric-stride` (default — for numeric-heavy presets)

Assign attribute *i* (1-based) the value `i * 11.111111`:

```
attr[1] -> 11.111111   attr[2] -> 22.222222   attr[3] -> 33.333333 …
```

`11.111111` is chosen so values are visually distinct, never collide under the
dialog's rounding, and stay clear of common real defaults (0, 1, 90, 100). On
re-read, **fuzzy-match within ±0.5**: a field showing `22.2` or `22.22` matches
`attr[2]`. The tolerance absorbs the dialog's display rounding while the stride
keeps adjacent attributes far enough apart to never alias.

### `token` (for textual presets)

Replace each string attribute with a token: `__attr_<name>__`
(e.g. `__attr_bolt_standard__`). On re-read, **match literally** — a field whose
text is `__attr_bolt_standard__` maps to `bolt_standard`. Use this when the
preset is mostly strings/enums, where numeric stride doesn't apply.

A real preset usually mixes both: stride the numeric attributes, tokenize the
string ones, in one pass.

## Output + confidence

Emit `field-id -> attribute` with a `confidence` 0..1:

- `1.0` — exact literal token match, or numeric within ±0.05.
- `0.5..0.99` — numeric within ±0.5 (rounded display).
- a field matching no sentinel is left out; an attribute that no field showed is
  reported under `unmatched` (it may be conditional / on an unvisited tab).

## Gotchas

- **Round-trip through the app's own loader**, not by poking fields directly —
  that's what proves the field truly *is* bound to the attribute.
- **Walk all tabs first** (`walk-tabs`): an attribute on an unvisited tab reads
  as `unmatched` purely because its field was never laid out.
- **Restore the live dialog before returning** — the sentinel *file* is a
  disposable copy (the user's real preset is never modified), but loading it
  changes the **open dialog's** visible values. Re-load the original preset (or
  close the dialog without applying) so you leave the host exactly as found —
  otherwise a later OK/apply by the user or downstream automation would commit
  sentinel values. This leave-as-found guarantee is what keeps `map-fields`
  read-mode despite transiently "writing" sentinels; a sidecar that can't
  guarantee it must run the command write-mode + safety-gated.
- **Collisions** mean your stride is too small or two attributes share a field
  (alias) — widen the stride or report both candidates rather than guessing.
