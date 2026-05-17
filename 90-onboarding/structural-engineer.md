# First hour — Structural Engineer

Goal: peer-review-delta running with a reproducibility receipt — the audit artefact your insurer asks for.

## What you need

- Tekla Structural Designer 2026 (TSD)
- A peer-review checkpoint (any prior model state — git, archived `.tsmd`, or a `~/.aware/snapshots/...` directory)
- 30 minutes

## 0-10 min: install AWARE

```powershell
npm install -g @aware-aeco/cli
aware --version

aware agent install aware-aeco
```

The engineering vertical specifically includes `tsd-26`, `idea-statica-26`, `csi-api`, `tekla-2026`, all with engineering-envelope pinnable declarations.

## 10-20 min: confirm the engineering pins

```powershell
aware agent describe tsd-26
```

You'll see (under "engineering pinnable"):

```
code-of-practice    required  example: eurocode-3@2022+uk-na
section-catalogue   required  example: en-10365@2017
material-catalogue  required  example: en-10025-2@2019
psi-factors                   example: en-1990@2002+uk-na-2002
solver-build        required  (auto-resolved at run time)
```

Your apps MUST pin every required field. The peer-review-delta reference app already does — but adjust the pins to *your* project's design codes.

## 20-35 min: install the reference app

```powershell
aware app install https://github.com/aware-aeco/aware/raw/main/30-apps/_examples/engineer-peer-review-delta.flo
```

Edit the `engineering.pins` block to match your project. Example: a UK steel project running EC3+UK NA on EN 10365 sections:

```yaml
engineering:
  pins:
    code-of-practice:    'eurocode-3@2022+uk-na'
    section-catalogue:   'en-10365@2017'
    material-catalogue:  'en-10025-2@2019'
    psi-factors:         'en-1990@2002+uk-na-2002'
    solver-build:        'tsd-26.0.3-build-19834'   # ← match your install
  output-seal:
    artifact:    '{{ delta-report.path }}'
    operator:    'P.Lisowski CEng MIStructE'        # ← your reference
    credential:  ''                                  # ← optional cert path
```

The transport will fail-loudly if a required pin doesn't match what's installed.

## 35-50 min: take the previous checkpoint

```powershell
aware app run engineer-peer-review-delta -- \
  --tsd-file 'C:\projects\acme\acme.tsmd' \
  --previous-checkpoint 'baseline-2026-04-01' \
  --report-dir 'C:\projects\acme\peer-review' \
  --engineer-id 'P.Lisowski CEng MIStructE' \
  --project 'Acme Tower' \
  --peer-reviewer 'reviewer@acme-eng.com' \
  --dry-run
```

If the prior snapshot doesn't exist yet, create one first:

```powershell
# Snapshot the current model state for future delta runs
aware snapshot create --of tsd-26 --target 'C:\projects\acme\acme.tsmd' --name 'baseline-2026-04-01'
```

## 50-60 min: run for real

Drop `--dry-run` and let it run. Output:

```
✓ run complete; trace at C:\Users\you\.aware\logs\engineer-peer-review-delta\default\<run-id>.jsonl
✓ delta report:    C:\projects\acme\peer-review\peer-review-delta-2026-05-17.html
✓ receipt:         C:\projects\acme\peer-review\peer-review-delta-2026-05-17.aware-receipt.json
```

The receipt JSON contains:

```json
{
  "artifact-sha256": "...",
  "pins": {
    "code-of-practice":    "eurocode-3@2022+uk-na",
    "section-catalogue":   "en-10365@2017",
    "material-catalogue":  "en-10025-2@2019",
    "psi-factors":         "en-1990@2002+uk-na-2002",
    "solver-build":        "tsd-26.0.3-build-19834"
  },
  "operator":  "P.Lisowski CEng MIStructE",
  "run-id":    "...",
  "timestamp": "2026-05-17T14:23:00Z",
  "agent-versions": {
    "tsd-26":      "26.0.1",
    "html-report": "1.0.0",
    "microsoft-365": "1.0.0"
  }
}
```

This receipt accompanies your stamped PDF — chain-of-custody for the underwriter.

## What you've achieved

- Replaces 4 hours of bi-weekly per-project peer-review pre-work
- ~100 engineer-hours/year reclaimed
- Audit receipt accompanies every delta — code revision pinned, solver build hashed, operator named

## Next

- Add IDEA connection-design batch: install `idea-statica-26`, pin its engineering envelope, wire `connection.check-batch` into a downstream node
- Calc-pack assembly: stitch the delta report + TSD output + IDEA outputs + cover sheet via the `html-report` agent
- Storey-add what-if: use the v0.19 `sweep:` primitive to iterate storey counts + tabulate utilisations
