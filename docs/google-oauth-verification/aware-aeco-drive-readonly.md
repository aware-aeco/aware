# Google OAuth verification package — `aware-aeco` project, Drive read (#662)

Prepared 2026-10-08 with aware-aeco/aware#618 (the Drive list-files read). Owner: Pawel (records the
video and submits). Tracking: aware-aeco/aware#662.

**Project.** Google Cloud project `aware-aeco` (project number `52384839180`) — the project that holds
AWARE's bundled Desktop OAuth client `52384839180-asibd6jtj2jv9nh63l5ji5i28d47ns5t.apps.googleusercontent.com`
(`cli/src/auth/config.rs`). The separate `floless` project (FloLess's own Sheets client, with its own
draft submission for `drive.readonly` + `spreadsheets`) is **not** part of this package.

> **Owner action in the Cloud console (aware-aeco):** the consent screen's *Data access* list must hold
> exactly the scopes below — `openid`, `.../auth/userinfo.email`, `.../auth/gmail.send`,
> `.../auth/drive.readonly`. (Reported done for Testing on 2026-10-08: Drive API enabled, these four
> scopes saved, Pawel as test user.) A scope AWARE requests that is missing from this list shows users
> the unverified-app screen even after verification
> ([Unverified apps](https://support.google.com/cloud/answer/7454865)).

Placeholders the owner fills in before submitting are marked **⟦…⟧**.

## 1. Scopes and justifications

Google's classification, checked 2026-10-08 against
[Drive API scopes](https://developers.google.com/workspace/drive/api/guides/api-specific-auth) and
[Gmail API scopes](https://developers.google.com/workspace/gmail/api/auth/scopes):

| Scope | Class | Requested when |
|---|---|---|
| `openid` | Non-sensitive | Every Google connection |
| `https://www.googleapis.com/auth/userinfo.email` | Non-sensitive | Every Google connection |
| `https://www.googleapis.com/auth/gmail.send` | Sensitive | Every Google connection (default consent) |
| `https://www.googleapis.com/auth/drive.readonly` | **Restricted** | Only when the user connects for Drive (`aware connect google-workspace --oauth --scopes https://www.googleapis.com/auth/drive.readonly`) |

### `openid`, `userinfo.email` — justification text

> AWARE is a command-line runtime that runs on the user's own computer. It uses `openid` and
> `userinfo.email` to read which Google account the user connected (the OpenID userinfo `sub` and
> verified email). AWARE shows that email so the user can see which account a command will use, and
> before every Drive read it re-checks that the stored sign-in still belongs to the same account the
> user selected; if it does not, the read is refused. No other profile data is read.

### `gmail.send` — justification text

> AWARE lets a user send one plain-text or HTML email from their own Gmail account as a step of a
> workflow they wrote and approved (for example, sending a finished report to a client). It calls only
> `users.messages.send` for a message the user's workflow composed, records each send so it is never
> sent twice, and never reads, lists or deletes mail. `gmail.send` is the narrowest scope that allows
> sending; read scopes such as `gmail.readonly` are not requested.

### `drive.readonly` — justification text

> AWARE lists the files in the user's Google Drive so the user — or the AI assistant they are working
> with in FloLess, AWARE's desktop companion — can find a project's files by name and type (for
> example, "list the issued PDFs in this project's share"). AWARE calls exactly one Drive method,
> `files.list`, with a fixed field mask: file id, name, MIME type, size and modified time, plus whether
> more results exist. It never downloads, exports, modifies, shares or deletes a file, and never calls
> any other Drive method; this is fixed in AWARE's code (`cli/src/runtime/agent_call.rs`, pinned by
> the test `a_drive_readonly_slot_still_only_lists_metadata`), not configurable by a user, a workflow
> or an AI.
>
> Why narrower scopes do not work: `drive.file` only covers files the app created or the user opened
> through the app, so it cannot list the files already in the user's Drive, which is the feature.
> `drive.metadata.readonly` would also cover listing; we request `drive.readonly` because ⟦reason the
> owner chose it — e.g. it is the scope already used by this product's other Drive reads⟧. AWARE
> accepts either scope and its code reads only metadata with both.
>
> Drive access is opt-in: it is not part of AWARE's default consent. A user is asked for it only when
> they choose to connect their Google account for Drive.

## 2. Data handling statement

For the submission's data-use questions and for the privacy policy ⟦privacy policy URL⟧, which must
say the same thing.

**Where the data lives.** AWARE runs entirely on the user's computer. Google sign-in tokens are stored
in the operating system's credential store (or a file readable only by the user, when no credential
store is available), on that computer. AWARE's publisher operates no server that receives, stores or
processes Google user data, tokens or Drive metadata. Requests go directly from the user's computer to
Google (`www.googleapis.com`, `openidconnect.googleapis.com`, `oauth2.googleapis.com`); AWARE's code
refuses to send the token anywhere else and does not follow redirects.

**What is read.** Drive file metadata only (id, name, MIME type, size, modified time) for the files a
`files.list` query returns, at most 100 per request; the account's `sub` and email. No file content,
no mail content.

**Where Drive metadata goes — including AI chat (disclosed).** The listing is returned to the program
that asked for it on the user's computer:
- in an AWARE workflow, to the next step of the user's own workflow, on the same computer;
- in **FloLess's AI chat**, to the AI assistant the user is chatting with. FloLess asks the user to
  approve each Drive read before it runs (per-read consent). The approved listing then becomes part
  of the conversation with the user's AI provider — Anthropic (Claude) or OpenAI (Codex), under the
  user's own account with that provider — and so **leaves the user's computer for that provider's
  servers**, where it is processed to answer the user and handled under the user's agreement with the
  provider.

**Limited Use.** AWARE's use of Google user data complies with the Google API Services User Data
Policy, including the Limited Use requirements
([policy](https://developers.google.com/workspace/workspace-api-user-data-developer-policy)):
- the data is used only to provide the user-facing feature the user invoked (listing their files);
- it is transferred to a third party (the user's AI provider) only to provide that feature, and only
  with the user's approval of each read;
- AWARE and its publisher do not use it, and do not allow it to be used, to create, train or improve
  any AI or machine-learning model; no human at the publisher can read it, because it never reaches
  the publisher;
- it is not sold, used for advertising, or used to determine credit-worthiness.
⟦Owner to confirm and state in the privacy policy how the user's AI provider settings — e.g. whether
the provider may use chat content for model training — are addressed, since that content can include
Drive metadata.⟧

**Security assessment.** Google requires an annual security assessment (CASA) for apps that can access
restricted data "from or through a third-party server"
([restricted scope verification](https://developers.google.com/identity/protocols/oauth2/production-readiness/restricted-scope-verification)).
AWARE itself keeps the data on the user's device, but the FloLess chat path sends Drive metadata
through the AI provider's servers, so **a security assessment may be required** and is accepted if
Google asks for it (owner ruling 2026-10-08: disclose and allow).

**Deletion.** Disconnecting (`aware disconnect google-workspace`, or removing the account in FloLess)
deletes the stored token from the user's computer; the user can also revoke AWARE at
<https://myaccount.google.com/permissions>. AWARE stores no Drive metadata of its own; listings exist
only in the output of the workflow or chat that requested them.

## 3. Demo video — shot-by-shot script

Requirements this script follows ([verification requirements](https://support.google.com/cloud/answer/13464321)):
show the end-to-end flow including the OAuth grant, the app as submitted (name and branding), the
complete consent screen with exactly the requested scopes and the language set to **English**, and
each scope in use. Upload unlisted; ⟦video URL⟧. Record on a clean machine profile; never show a
token, the credential store, or a client secret.

| # | Show | Say / caption |
|---|---|---|
| 1 | Title card: "AWARE — Google Drive & Gmail access (project aware-aeco)". | What AWARE is: a local runtime; FloLess is its desktop companion. |
| 2 | Terminal: `aware --version`. | The version being verified. |
| 3 | Terminal: `aware connect google-workspace --oauth --scopes https://www.googleapis.com/auth/drive.readonly`. The browser opens. | Drive access is opt-in; this is the command that asks for it. |
| 4 | Browser address bar, zoomed: the Google authorization URL showing `client_id=52384839180-asibd6jtj2jv9nh63l5ji5i28d47ns5t.apps.googleusercontent.com` and the `scope=` list. | The client id matches the submission. |
| 5 | Google account chooser → pick the test account. | — |
| 6 | **"Google hasn't verified this app"** screen, shown in full; click *Advanced* → *Go to ⟦app name⟧ (unsafe)*. | Shown because the app is not yet verified; this is the screen verification removes. |
| 7 | The **complete consent screen**, language English (set it with the footer language picker if needed): app name ⟦app name⟧, and the requested access — see your primary email address; send email on your behalf; see and download all your Google Drive files. Pause so every line is legible; scroll if needed. | Exactly the four requested scopes. |
| 8 | Click *Continue*/*Allow*; browser shows AWARE's "connected" page; terminal confirms the slot. | Tokens stay on this computer. |
| 9 | Terminal: `aware --json agent call-capabilities google-workspace list-files` — highlight `presentation_label` (the email) and `available: true`. | openid + userinfo.email: AWARE shows which account is connected. |
| 10 | Terminal: run a Drive listing (`aware --json agent call @request.json`, page size 5) — highlight file names, MIME types, modified times. | drive.readonly in use: only `files.list`, metadata only. |
| 11 | Code view: `cli/src/runtime/agent_call.rs` — the fixed field mask and `files.list` URL. | The only Drive call AWARE makes; no download path exists. |
| 12 | FloLess: open an AI chat, ask "What PDFs are in my Drive?"; FloLess's **approval prompt** for the Drive read appears; approve it; the assistant answers from the listing. | Drive metadata goes to the user's AI provider only after this approval. |
| 13 | A workflow step sending one email with `gmail.send` (a test message to the tester's own address); show it arriving. | gmail.send in use: one message the user's workflow composed. |
| 14 | Terminal: `aware disconnect google-workspace`; browser: myaccount.google.com/permissions showing the app, then removing it. | How a user deletes AWARE's access. |
| 15 | End card: privacy policy URL ⟦privacy policy URL⟧. | — |

Shots 12 and 13 need FloLess's chat Drive tool and a gmail.send workflow to be released; record them
once those builds are available. ⟦Owner: confirm the app name and logo on the aware-aeco consent screen
match the submission.⟧
