# `aware-skill-builder.eval` — run skill-creator's eval on an existing skill

Stateless. Tests whether a skill triggers correctly on a set of prompts. Surfaces false positives, false negatives, and suggested fixes. Mandatory before any commit involving a skill change.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `agent` | string | Agent that owns the skill. |
| `skill-name` | string | Short filename (no extension). |
| `test-prompts` | array | List of prompts to test. Each marked as positive (should trigger) or negative (should not). |

If `test-prompts` is omitted, the eval uses a default set derived from the skill's frontmatter description (the trigger phrases enumerated there become positives; common unrelated AECO prompts become negatives).

## Test prompt structure

```yaml
test-prompts:
  - prompt: "Upload my Tekla drawing to Trimble Connect"
    should-trigger: true
  - prompt: "Push the IFC export to TC fab folder"
    should-trigger: true
  - prompt: "List files in the TC project root folder"
    should-trigger: true

  - prompt: "Send a Slack message about the new assembly"
    should-trigger: false                       # negative
  - prompt: "Update Excel cell B12 with the part count"
    should-trigger: false                       # negative
```

A good eval set has at least:

- **5 positive prompts** covering diverse phrasings of the same intent
- **5 negative prompts** spanning unrelated AECO contexts (so the test catches over-triggering)

Below 5+5 the accuracy number is noisy and not trustworthy.

## Outputs

```yaml
accuracy:           number              # 0.0–1.0 (fraction of test prompts judged correctly)
true-positives:     int                 # positives that triggered as expected
false-positives:    array               # negatives that triggered (bad — skill is too broad)
false-negatives:    array               # positives that didn't trigger (bad — skill is too narrow)
suggested-fixes:    array               # skill-creator's recommendations
status:             enum                # pass | needs-tightening | needs-broadening | inconclusive
```

| Status | Meaning |
|---|---|
| `pass` | accuracy ≥ 0.90 — ship it |
| `needs-tightening` | accuracy < 0.90 due to false positives — description too broad |
| `needs-broadening` | accuracy < 0.90 due to false negatives — description too narrow |
| `inconclusive` | test set too small or unbalanced — add more prompts |

## CLI form

```bash
aware skill eval \
  --agent trimble-connect \
  --skill-name files \
  --test-prompts-file ./tc-files-eval.yaml
```

Or without a file (uses default test set):

```bash
aware skill eval --agent trimble-connect --skill-name files
```

## When to invoke

- **Before committing** any new or modified skill — gated check
- **After porting** a FloLess skill — verify it still triggers on the original use cases
- **On a regular cadence** (weekly?) across all skills as part of repo health checks
- **When a contributor reports** the skill behaves unexpectedly — eval first, then `modify` based on the failing prompts

## Interpreting failures

### False positives (skill triggers when it shouldn't)

The description is too broad. Common causes:

- Description uses generic AECO terms ("BIM", "project", "model") with no qualifier
- Trigger phrases overlap with other agents (e.g., a Procore skill triggering on "upload to construction PM" might also pull a TC skill)

Fix: `aware skill modify --issue "tighten — false positive on X"` with the failing negatives passed in.

### False negatives (skill doesn't trigger when it should)

The description is too narrow. Common causes:

- Trigger phrases don't include common synonyms ("push", "send", "sync" for "upload")
- Description uses vendor-specific jargon ("BCF document_reference") instead of user-side phrasing ("link file to issue")

Fix: `aware skill modify --issue "broaden — false negative on X"` with the failing positives passed in.

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `skill.not-found` | filename doesn't exist in agent | check spelling |
| `eval.test-set-too-small` | < 5 positives or < 5 negatives | add more prompts |
| `eval.inconclusive` | positives and negatives overlap semantically | re-phrase negatives to be unambiguously unrelated |

## See also

- [`pipeline.md`](../skills/pipeline.md) — Step 6 of the pipeline
- [`modify.md`](./modify.md) — for fixing skills that fail eval
- [`skill-creator-invocation.md`](../skills/skill-creator-invocation.md) — what skill-creator's eval mode does under the hood
