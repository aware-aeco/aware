# Trimble Connect reads from `aware agent invoke` (floless.app#2466) — PLAN

Date: 2026-11-04. Base: `main` 9badb965 (v0.171.0).

## Goal

A terminal AI, and FloLess AI Chat (which runs connected-tool commands exactly as the terminal does,
`aware agent invoke <agent> <cmd> --as <alias>`), can run Trimble Connect **reads** — list projects,
folders, files, BCF topics, comments, viewpoints, ToDos — without composing a `.flo`. Writes stay as they are.

## What is true today (measured, not assumed)

Measured in a temp `AWARE_HOME` with `AWARE_DISABLE_KEYRING=1`, the source manifest (0.5.0) installed and a fake
token:

- **Declarative reads already run under `invoke`.** `list-projects`, `get-current-user` reach Trimble and return its
  401 as data; `get-project`, `list-folders`, `get-todo` validate their path inputs. They go through the generic REST
  path, which under `invoke` already resolves `--as` before any network call (`alias_credential_handle`), never follows
  redirects, and rebases to the project's region (`trimble_region`).
- **Code-handled reads refuse.** `find-item`, `list-folder-by-path` (`trimble_ops`) and every `trimble_bcf` read
  (`list-topics`, `get-topic`, `list-comments`, `list-viewpoints`, `list-todos`, …) answer
  `[E_AGENT_INVOKE_PINNED_FLOW]` from `invoker.rs` because `trimble_ops::handles()` is refused wholesale under `invoke`.
  The stated reason — "they resolve their own credential and follow redirects" — is true of them today:
  `trimble_files::auth_and_base` reads only the manifest's default slot, and `trimble_ops` uses `http_agent()`, which
  follows ureq's default 5 redirects. (`trimble_bcf` already uses `redirects(0)`.)
- **Users get 4 commands.** The registry pins `trimble-connect` 2.4.0 → manifest 0.2.0 (`list-projects`,
  `list-folders`, `upload`, `download`). The source manifest is 0.5.0 with 46 commands.

So the handoff's idea — new `agent_call` `OPERATIONS` rows for Trimble — is not needed and would be the wrong fit:
`agent_call` routes are static origins, while Trimble's host is discovered per project/region (and BCF uses a second
host family). The Trimble modules already own that routing and its host allowlists.

## Change

1. **A code-owned read set** in the Trimble modules:
   - `trimble_ops::DIRECT_READS = ["find-item", "list-folder-by-path"]`
   - `trimble_bcf::DIRECT_READS = ["list-topics", "get-topic", "list-comments", "list-viewpoints", "get-viewpoint",
     "list-todos", "list-todo-types", "get-project-extensions", "list-related-topics", "list-document-references",
     "list-bcf-documents"]`
   - Left out on purpose: `get-viewpoint-snapshot` and `download-bcf-document` (binary bodies, a transfer agent;
     nothing a chat can show), and every write.
   - A test pins the set against the manifest: every listed command exists and is `mode: read` (the default), every
     `handles()` command that is not listed is either `mode: write` or one of the two named binary reads. So a new
     read added to a module is a conscious decision, and a write can never be listed.
2. **The chosen slot, resolved before any network call.** `trimble_files::auth_and_base_for(catalogue, alias)`:
   `None` = today's behaviour; `Some(alias)` goes through the same `alias_credential_handle` the generic path uses
   (made `pub(crate)`), so an alias that is not connected refuses with `E_AGENT_INVOKE_ALIAS_MISSING` and sends
   nothing — never falls back to the default account.
3. **No redirects under `invoke`.** `trimble_ops` takes the HTTP agent from the caller: workflow path unchanged
   (`http_agent()`), direct path a `redirects(0)` agent with the same timeouts. The `/regions` lookup already uses
   `redirects(0)`. BCF is already `redirects(0)`.
4. **Entry points.** `trimble_ops::invoke_direct(catalogue, command, args, alias)` and
   `trimble_bcf::invoke_direct(...)`, each refusing (defensive) a command outside its `DIRECT_READS`. `invoker.rs`
   narrows the refusal: under `direct`, trimble-connect refuses `upload`, `download` and any `handles()` command
   not in `DIRECT_READS` (unchanged message and code); a `DIRECT_READS` command runs through `invoke_direct`.
   Workflow path untouched.
5. **Stable failure codes under `invoke` only.** The Trimble modules' HTTP failure text includes the service body
   (`"{what}: HTTP {code}: {body}"`). Under `invoke` the error is re-shaped to AWARE's own sentence plus code, the form
   `agent_call::workflow_error` uses: `The service answered the call with a failure. [E_CALL_FAILED status=<code>]`,
   with no body; a missing default credential becomes `No usable credential is stored for this account.
   [E_CREDENTIAL_MISSING]`. To avoid sniffing our own strings, `Fetched::failure` gains a typed sibling: the direct
   path carries a `TrimbleHttpStatus(u16)` through `AwareError` … (implementation detail: if `AwareError` has no
   suitable variant, the direct wrapper maps the failure at the one place `fetch` builds it, via a small
   `DirectFailure` enum returned by the direct-path helpers). Workflow-path messages are unchanged.
6. **Manifest 0.6.0** declares `minimum-cli-version: 0.172.0`, the release this ships in, so an older CLI refuses the
   new manifest instead of misrunning it (the #744 precedent). Its description is unchanged otherwise.
7. **Docs.** `10-core/cli-spec.md` `agent invoke`: Trimble Connect reads run directly; Trimble writes and transfers
   stay workflow-only.

## Tests

- `trimble_ops` / `trimble_bcf` against the existing 127.0.0.1 wire mocks:
  - a direct read sends `Authorization: Bearer <alias token>` for `--as team`, and the default token without it;
  - `--as missing` sends **no request at all** (mock asserts zero connections) and yields `E_AGENT_INVOKE_ALIAS_MISSING`;
  - a 302 from the Core host on a direct `find-item` is a failure, the `Location` is never requested;
  - a 401 on a direct `list-topics` yields `[E_CALL_FAILED status=401]` and the error text holds no response body;
  - a write (`create-topic`, `delete-folder`) under direct is still `E_AGENT_INVOKE_PINNED_FLOW`; `upload`/`download` too.
- The manifest pin test of change 1.
- `cli/tests/agent_invoke_rest.rs`: one end-to-end `aware agent invoke trimble-connect list-todos` against the mock.
- Registry: none in this PR (see Release).

## Release and registry (order)

1. This PR → review → merge to `main`.
2. Release `v0.172.0` — **only with Pawel's go-ahead**.
3. Registry PR: `trimble-connect` 2.5.0 → manifest 0.5.1 pinned to the v0.172.0 release commit, bundle tracks it
   (#744 shape, with its contract test).
4. Comment `Shipped in v0.172.0` on the tracking issue.

## FloLess side (separate repo, after the release)

`server/tool-command-routes.ts`: per-agent AWARE floor (trimble-connect 0.172.0, microsoft-365 0.170.0), and the
`aware-update-needed` copy names the service instead of Microsoft 365. Map `E_AGENT_INVOKE_ALIAS_MISSING` to the
same next step as `E_CREDENTIAL_MISSING`. Scopes: AWARE declares `openid profile TrimbleConnect offline_access`;
FloLess requests what AWARE declares.

## Risks / questions for review

- Is narrowing the PINNED_FLOW refusal safe for `list-folder-by-path` and `find-item`, which make several GETs
  (folder walk)? They are read-only, bounded by the existing walk limits, same host family, now no-redirect.
- BCF reads first resolve the project's topic host via `/regions`; under `--as` the token used for any authenticated
  lookup must be the alias token (it is: one `Client` per call).
- Any read in the set with a side effect? `get-project-extensions` / `list-bcf-documents` are GETs only (verify).

## Revision after the Codex plan review (REJECT, 7 findings) — what was built

1. **Starting host pinned.** `trimble_region::require_core_master`: under `invoke` the manifest base must be a published
   Core host (loopback only in test builds) before any authenticated request — `E_CALL_ORIGIN_NOT_ALLOWED`, nothing sent.
2. **Only a 2xx is an answer.** `trimble_files::fetch` fails anything outside 2xx (the redirect wording BCF had moved
   into `Fetched::failure`, so workflows keep it); `trimble_region` project lookup and `trimble_ops::get_optional` match
   `200..=299` explicitly, 404 stays data. Fixes both paths: a 3xx body was parsed as an answer before.
3. **No typed failures.** Internal error text is unchanged (the BCF `"region: project: HTTP 404"` fallback still matches).
4. **One failure boundary.** `trimble_files::direct::failure` wraps everything after the credential is resolved: only
   the HTTP status from AWARE's own `HTTP <code>` wording, or which bound was hit, survives — never a body, a
   `Location` or a value read from an answer.
5. **Manifest floor.** 0.5.1 is `status: requires-runtime`, `minimum-cli-version: 0.159.0` — the CLI that first runs
   its handled commands. Not 0.172.0: on 0.159–0.171 the direct reads refuse safely (`E_AGENT_INVOKE_PINNED_FLOW`) and
   workflows run, so a 0.172 floor would only lock those CLIs out. The registry can therefore publish 0.5.1 without
   waiting for the release; FloLess gates its chat promise on AWARE 0.172. It is 0.5.1, not 0.6.0: the commands and inputs are unchanged, so the published `0.5.x` example pins still hold.
6. **Bounded.** `trimble_files::direct`: a thread-local budget on the call's blocking thread — 40 requests, 8 MiB of
   bodies, 60 s — enforced in `fetch_any`, which every Trimble request goes through. Workflows run unbounded as before.
7. **Tests.** Every direct BCF/ToDo read against a catch-all mock: only GETs, something beyond discovery, the `--as`
   token on every authenticated hop and never the default; ops: slot choice, missing slot sends nothing, unpinned base
   sends nothing, 302 with an answer-shaped body fails, 403 text does not leak, writes still refused; the manifest pin
   test (`mode_of`); budget and sanitiser unit tests; and a binary-level test (the production binary refuses the
   loopback base, so it proves routing and the pin without weakening the host check).
