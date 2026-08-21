//! `aware credential ...` — the opaque-credential lifecycle (#436).
//!
//! `aware connect` provisions the OAuth integrations AWARE ships a client for.
//! It validates its `INTEGRATION` argument against that list *before* any import
//! path runs (`commands::connect::run_connect` calls
//! `auth::config::for_integration` ahead of `--from-file` / `--from-env`), so it
//! cannot provision a handle AWARE has no OAuth config for.
//!
//! The REST transport has no such restriction. An agent's declared
//! `auth: { scheme: bearer, secret: <handle> }` resolves through
//! `runtime::invoker::resolve_rest_credential`, which falls through to the raw
//! stored value for any handle that is not a registered integration. So the
//! runtime reads credentials the supported CLI could not write, and callers that
//! mint their own short-lived REST bearers were left writing
//! `~/.aware/credentials/<handle>.json` by hand — owning AWARE's storage
//! internals, and with no defined way to rotate or revoke.
//!
//! This group is the missing provisioning half of that seam: put / delete /
//! status over an arbitrary validated handle. The secret is read from stdin, a
//! file, or an environment variable — never argv, where it would land in shell
//! history and in every process listing on the machine. Where the bytes end up
//! (OS keychain or the credentials-file fallback) stays AWARE's business:
//! `auth::keychain` picks, and every verb here goes through it.

use clap::{Args, Subcommand};

use crate::auth::keychain::{StoredToken, TokenSource};
use crate::context::Context;
use crate::error::AwareError;

#[derive(Subcommand, Debug)]
pub enum CredentialCommand {
    /// Store the secret behind an opaque handle, replacing any current value.
    ///
    /// The secret is read from stdin by default:
    /// `printf %s "$TOKEN" | aware credential put my-service`.
    Put(PutArgs),

    /// Remove the credential behind a handle. Idempotent — a handle that was
    /// already absent is success, not an error.
    Delete(HandleArgs),

    /// Report whether a handle has a usable credential. Never prints the secret.
    Status(HandleArgs),
}

#[derive(Args, Debug)]
pub struct PutArgs {
    /// Credential handle — the same string an agent manifest names in
    /// `auth: { secret: <handle> }`.
    pub handle: String,

    /// Account alias for multi-account setups. Stored as `<handle>.<alias>`,
    /// which is the handle an agent manifest then names.
    #[arg(long = "as")]
    pub r#as: Option<String>,

    /// Read the secret from a file instead of stdin (plain text; trailing
    /// newline ignored).
    #[arg(long = "from-file")]
    pub from_file: Option<std::path::PathBuf>,

    /// Read the secret from env var `AWARE_TOKEN_<HANDLE>` (upper-cased, with
    /// `-` and `.` as `_`) instead of stdin.
    #[arg(long = "from-env")]
    pub from_env: bool,
}

#[derive(Args, Debug)]
pub struct HandleArgs {
    /// Credential handle.
    pub handle: String,

    /// Account alias, as passed to `put --as`.
    #[arg(long = "as")]
    pub r#as: Option<String>,
}

pub fn dispatch(cmd: CredentialCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        CredentialCommand::Put(args) => run_put(args, ctx),
        CredentialCommand::Delete(args) => run_delete(args, ctx),
        CredentialCommand::Status(args) => run_status(args, ctx),
    }
}

// ── put ───────────────────────────────────────────────────────────────────────

fn run_put(args: PutArgs, ctx: &Context) -> Result<(), AwareError> {
    let account = validated_account(&args.handle, args.r#as.as_deref())?;
    if args.from_file.is_some() && args.from_env {
        return Err(AwareError::Validation(
            "at most one of --from-file, --from-env may be set (stdin is the default)".into(),
        ));
    }
    let secret = read_secret(&args, &account)?;

    // Whether this is a first provision or a rotation, purely so the operator is
    // told which one happened. An unreadable current value still counts as a
    // rotation: something was there and this call replaces it.
    let replacing = !matches!(
        crate::auth::keychain::load_token(
            &args.handle,
            args.r#as.as_deref(),
            &ctx.paths.aware_home
        ),
        Ok(None)
    );

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let token = StoredToken {
        access_token: secret,
        refresh_token: None,
        // 0 is the "no known expiry" sentinel `--from-env` already stores. AWARE
        // runs no refresh flow for an opaque handle and does not police its
        // lifetime: expiry is the issuer's business, and the answer to a lapsed
        // one is another `put`. Recording a deadline here would read as an
        // enforcement the runtime does not perform — `Paste` credentials never
        // reach the expiry branch in `credential_status_json`.
        expires_at: 0,
        scope: String::new(),
        token_type: "Bearer".into(),
        integration: args.handle.clone(),
        obtained_at: now,
        // `Paste` = user-managed, no refresh token. That is exactly an opaque
        // bearer, and it keeps `connect --list` / `doctor` from offering
        // `--refresh` on a credential nothing can refresh.
        source: TokenSource::Paste,
    };
    crate::auth::keychain::store_token(&token, args.r#as.as_deref(), &ctx.paths.aware_home)?;

    let status = if replacing { "rotated" } else { "stored" };
    if ctx.json {
        println!(
            "{}",
            serde_json::json!({
                "status": status,
                // The full resolvable handle, alias folded in — the string an
                // agent manifest's `auth.secret` has to carry to reach this
                // credential, which is the one thing the caller needs back.
                "handle": account,
            })
        );
    } else {
        println!(
            "\u{2713} {status} credential {account} (OS keychain or ~/.aware/credentials fallback)"
        );
    }
    Ok(())
}

/// Read the secret from exactly one of stdin (default), a file, or an env var.
///
/// Never from argv: an argument is visible in shell history and to every process
/// listing on the machine for as long as the command runs.
fn read_secret(args: &PutArgs, account: &str) -> Result<String, AwareError> {
    let (raw, origin) = if let Some(path) = &args.from_file {
        let body = std::fs::read_to_string(path)
            .map_err(|e| AwareError::NotFound(format!("{}: {e}", path.display())))?;
        (body, path.display().to_string())
    } else if args.from_env {
        let var = env_var_name(account);
        let body = std::env::var(&var).map_err(|_| {
            AwareError::Validation(format!(
                "env var {var} not set; either set it, use --from-file, or pipe the secret to stdin"
            ))
        })?;
        (body, var)
    } else {
        let mut body = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin(), &mut body)
            .map_err(|e| AwareError::Validation(format!("reading secret from stdin: {e}")))?;
        (
            body,
            "stdin (pipe it: printf %s \"$TOKEN\" | aware credential put <handle>)".to_string(),
        )
    };

    let secret = raw.trim();
    if secret.is_empty() {
        return Err(AwareError::Validation(format!(
            "no secret from {origin} — refusing to store an empty credential, which would report \
             {account} as provisioned while every call it authorizes fails"
        )));
    }
    // A bearer is interpolated straight into a request header
    // (`runtime::invoker::inject_auth` builds `Authorization: Bearer {cred}`), so
    // a control character in one is a header-injection primitive: a CR/LF would
    // end the header and start attacker-chosen ones. Refuse at provisioning time,
    // where there is a human to tell, rather than at each call. Interior spaces
    // and non-ASCII are left alone — some issuers really do mint those, and
    // neither can terminate a header.
    if secret.contains(char::is_control) {
        return Err(AwareError::Validation(format!(
            "the secret from {origin} contains a control character — a credential is interpolated \
             into an Authorization header, so a newline in one would inject headers. Check for a \
             stray line break or a file with CRLF endings"
        )));
    }
    Ok(secret.to_string())
}

/// `AWARE_TOKEN_<ACCOUNT>` — the variable `--from-env` reads.
///
/// The same derivation `connect --from-env` uses (upper-case, `-` → `_`), with
/// `.` mapped too so an alias-qualified account still names a legal variable.
/// For an alias-free handle the two are byte-identical, so a caller that already
/// knows the `connect` convention does not have to learn a second one.
fn env_var_name(account: &str) -> String {
    format!(
        "AWARE_TOKEN_{}",
        account.to_uppercase().replace(['-', '.'], "_")
    )
}

// ── delete ────────────────────────────────────────────────────────────────────

fn run_delete(args: HandleArgs, ctx: &Context) -> Result<(), AwareError> {
    let account = validated_account(&args.handle, args.r#as.as_deref())?;
    // Probed only to report which of the two idempotent outcomes happened. An
    // unreadable credential is still something to revoke, so it takes the
    // `revoked` branch rather than being reported as absent.
    let existed = !matches!(
        crate::auth::keychain::load_token(
            &args.handle,
            args.r#as.as_deref(),
            &ctx.paths.aware_home
        ),
        Ok(None)
    );
    // Clears the keychain entry AND the file fallback, and errors if either
    // removal fails — a revoke that leaves the credential readable must not exit 0.
    crate::auth::keychain::delete_token(&args.handle, args.r#as.as_deref(), &ctx.paths.aware_home)?;

    let status = if existed { "revoked" } else { "absent" };
    if ctx.json {
        println!(
            "{}",
            serde_json::json!({ "status": status, "handle": account })
        );
    } else if existed {
        println!("\u{2713} revoked credential {account}");
    } else {
        println!("\u{2713} no credential stored for {account} (nothing to revoke)");
    }
    Ok(())
}

// ── status ────────────────────────────────────────────────────────────────────

fn run_status(args: HandleArgs, ctx: &Context) -> Result<(), AwareError> {
    let account = validated_account(&args.handle, args.r#as.as_deref())?;
    // `present` rather than `valid`: AWARE cannot validate an opaque bearer — it
    // knows only that a credential is stored and readable. Only the issuing
    // service can say whether it still works.
    //
    // All three states exit 0. This is a query, and reporting the state IS the
    // answer; a script branches on the field
    // (`aware --json credential status <handle> | jq -r .status`), not on the
    // exit code. That matches `connect --list`, which also reports a missing
    // credential at exit 0.
    let status = match crate::auth::keychain::load_token(
        &args.handle,
        args.r#as.as_deref(),
        &ctx.paths.aware_home,
    ) {
        Ok(Some(_)) => "present",
        Ok(None) => "missing",
        // `load_token` reads a NARROWER set of shapes than the REST transport
        // does: a `StoredToken`, or a blob with `access_token` / `token`. The
        // transport also takes a bare JSON string and an object keyed `key` /
        // `apikey` / `api_key` / `value` / `secret`. So a credential the runtime
        // authenticates with every call lands here — and answering `unreadable`
        // would send its owner off to re-provision something that works.
        //
        // The question this command is asked is "will my agent authenticate?",
        // so the transport's own rules decide before we call it unreadable.
        // Nothing is lost on the genuinely-corrupt path: a file that will not
        // parse fails both readers.
        //
        // Only the error arm needs this. `Ok(None)` means the store holds
        // nothing at all, and the transport's secondary lookup reads the same
        // file, so it has nothing to find either.
        Err(_)
            if crate::runtime::invoker::stored_secret_is_usable(
                &ctx.paths.aware_home,
                &account,
            ) =>
        {
            "present"
        }
        // Distinct from `missing` on purpose: a credential that is stored but
        // unreadable (corrupt JSON, an unreachable keychain) sends a caller down
        // a different road than one that was never provisioned.
        Err(_) => "unreadable",
    };

    if ctx.json {
        println!(
            "{}",
            serde_json::json!({ "status": status, "handle": account })
        );
    } else {
        match status {
            "present" => println!("\u{2713} {account}  present"),
            // The hint names the ACCOUNT, not the bare handle. With `--as session`
            // the bare handle provisions a different credential and leaves this one
            // missing, so the recovery command would report success and fix
            // nothing. Safe because every account is itself a valid handle — the
            // invariant `validated_account` enforces.
            "missing" => {
                println!("\u{2717} {account}  missing  run: aware credential put {account}")
            }
            _ => println!("? {account}  unreadable (stored, but the store could not be read)"),
        }
    }
    Ok(())
}

// ── handle validation ─────────────────────────────────────────────────────────

/// Longest accepted handle, alias, or the account the two combine into.
/// Generous for a name, short of the limits a credential store or a filesystem
/// starts refusing.
const MAX_LABEL: usize = 64;

/// Names Windows resolves as devices whatever extension follows, so
/// `credentials/con.json` never becomes a file there. Rejected on every platform
/// so a handle cannot work on Linux and fail on Windows.
const RESERVED_DEVICE_NAMES: &[&str] = &[
    "con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8",
    "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9",
];

/// Validate handle and alias, and return the account name the store will use.
///
/// The returned string is also what the caller must put in an agent manifest's
/// `auth.secret`, so returning it here keeps the CLI's echo and the runtime's
/// lookup derived from one place.
fn validated_account(handle: &str, alias: Option<&str>) -> Result<String, AwareError> {
    // Dots are the handle's own separator (`resolve_rest_credential` splits on
    // the first one), so they are legal in a handle and not in an alias — an
    // alias with a dot in it would silently re-partition the handle.
    validate_label("handle", handle, true)?;
    if let Some(alias) = alias {
        validate_label("alias", alias, false)?;
    }

    if handle == "oauth-app" || handle.starts_with("oauth-app.") {
        return Err(AwareError::Validation(format!(
            "handle {handle} is reserved: `oauth-app.` is where BYO OAuth client secrets are \
             stored (aware connect --set-app-secret), and a credential here would collide with one"
        )));
    }

    // `resolve_rest_credential` splits a handle at the FIRST dot and treats the
    // base as a possible integration, preferring the OAuth path when it is one.
    // A credential put here under such a base would therefore be shadowed at run
    // time by the refreshed OAuth token — so refuse it and name the command that
    // does own it, rather than storing something the runtime will not read.
    let base = handle.split_once('.').map_or(handle, |(base, _)| base);
    if crate::auth::config::for_integration(base).is_ok() {
        return Err(AwareError::Validation(format!(
            "{base} is a registered OAuth integration — provision it with \
             `aware connect {base}` (which manages its refresh token). \
             `aware credential` is for opaque handles AWARE runs no OAuth flow for"
        )));
    }

    let account = match alias {
        Some(alias) => format!("{handle}.{alias}"),
        None => handle.to_string(),
    };

    // THE INVARIANT: every account this command can create is itself a valid
    // handle. `put <handle> --as <alias>` and `put <handle>.<alias>` reach the
    // same credential, an agent manifest's `auth.secret` carries the combined
    // form, and the runtime's missing-credential hint names it — so an account
    // this command produced but would refuse to take back is a recovery
    // instruction the user cannot carry out.
    //
    // Length is the only way to break it, which is why one check closes it.
    // Given a valid handle and a valid alias, the joined string satisfies every
    // other rule by construction: its character set is the union of two allowed
    // sets plus the separator; it starts on the handle's first character and
    // ends on the alias's last, both alphanumeric; the separator cannot form
    // `..` because the characters either side of it are those same alphanumerics;
    // and each dot-part was checked against the reserved names already — the
    // alias being one whole part, since a dotted alias is refused above.
    if account.len() > MAX_LABEL {
        return Err(AwareError::Validation(format!(
            "handle and alias combine to {} characters, over the {MAX_LABEL}-character limit — \
             shorten one. The combined name {account:?} is the credential's real identity: it is \
             the file it is stored as, the string an agent manifest's `auth.secret` must carry, \
             and the handle `aware credential put {account}` would take, so it has to satisfy the \
             same limit a handle does",
            account.len()
        )));
    }
    Ok(account)
}

/// Accept only names that are safe as a keychain account AND as a file stem
/// under `~/.aware/credentials/`, on every platform AWARE ships to.
fn validate_label(kind: &str, value: &str, allow_dots: bool) -> Result<(), AwareError> {
    let reject = |why: &str| {
        Err(AwareError::Validation(format!(
            "{kind} {value:?} is not usable: {why}. Use lowercase letters, digits, `-` and `_`\
             {}, starting and ending with a letter or digit",
            if allow_dots {
                ", with `.` between parts"
            } else {
                ""
            }
        )))
    };

    if value.is_empty() {
        return reject("it is empty");
    }
    if value.len() > MAX_LABEL {
        return reject(&format!("it is longer than {MAX_LABEL} characters"));
    }
    // An uppercase handle is refused rather than folded: the keychain account and
    // the fallback filename disagree about case sensitivity across platforms, so
    // `MyApi` and `myapi` would be one credential on Windows and two on Linux.
    if !value.chars().all(|c| {
        c.is_ascii_lowercase()
            || c.is_ascii_digit()
            || matches!(c, '-' | '_')
            || (allow_dots && c == '.')
    }) {
        return reject("it contains characters outside the allowed set");
    }
    // Anchoring both ends is what keeps `..`, a leading dot, and a path separator
    // out of the filename this becomes — and a leading `-` from being read as a
    // flag by whatever shells the handle around.
    let ends_alnum = |c: Option<char>| c.is_some_and(|c| c.is_ascii_alphanumeric());
    if !ends_alnum(value.chars().next()) || !ends_alnum(value.chars().last()) {
        return reject("it must start and end with a letter or digit");
    }
    if value.contains("..") {
        return reject("it contains an empty part (`..`)");
    }
    if let Some(part) = value
        .split('.')
        .find(|part| RESERVED_DEVICE_NAMES.contains(part))
    {
        return reject(&format!(
            "`{part}` is a reserved device name on Windows, so it could never be stored there"
        ));
    }
    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn put_args(handle: &str) -> PutArgs {
        PutArgs {
            handle: handle.to_string(),
            r#as: None,
            from_file: None,
            from_env: false,
        }
    }

    // ── handle validation ────────────────────────────────────────────────────

    #[test]
    fn a_plain_handle_is_its_own_account() {
        assert_eq!(
            validated_account("floless-workspace", None).unwrap(),
            "floless-workspace"
        );
        assert_eq!(validated_account("svc_1", None).unwrap(), "svc_1");
    }

    /// An alias is folded into the account exactly as `auth::keychain`'s
    /// `account_name` does, because that string has to match the handle an agent
    /// manifest names for the runtime to find the credential at all.
    #[test]
    fn an_alias_is_folded_into_the_account_the_runtime_looks_up() {
        assert_eq!(
            validated_account("floless-workspace", Some("session")).unwrap(),
            "floless-workspace.session"
        );
    }

    /// The characters that would escape `~/.aware/credentials/<account>.json`.
    /// Each is refused before anything is written, not sanitised into something
    /// else — a silently rewritten handle would store the credential where the
    /// runtime does not look.
    #[test]
    fn a_handle_that_could_escape_the_credentials_directory_is_refused() {
        for bad in [
            "../etc/passwd",
            "a/b",
            "a\\b",
            "..",
            ".hidden",
            "trailing.",
            "a..b",
            "-leading-dash",
        ] {
            assert!(
                validated_account(bad, None).is_err(),
                "expected {bad:?} to be refused"
            );
        }
    }

    /// Case is refused rather than folded: the keychain account and the fallback
    /// filename disagree about case sensitivity across platforms, so accepting
    /// both spellings makes one credential on Windows and two on Linux.
    #[test]
    fn an_uppercase_handle_is_refused_rather_than_lowercased() {
        let err = validated_account("MyApi", None).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)), "{err:?}");
    }

    /// THE INVARIANT, asserted directly: anything `validated_account` accepts is
    /// itself an accepted handle. `put <h> --as <a>` and `put <h>.<a>` reach the
    /// same credential, and the runtime's missing-credential hint names the
    /// combined form — so an account this command produces but will not take back
    /// is a recovery instruction that exits 3.
    ///
    /// Codex found exactly that on this PR: the length limit was per-label, so a
    /// 64-character handle plus `--as session` stored a 72-character account the
    /// same validator then refused as a handle.
    #[test]
    fn every_account_this_accepts_is_itself_an_accepted_handle() {
        let cases: &[(&str, Option<&str>)] = &[
            ("my-api", None),
            ("my.api.key", None),
            ("floless-workspace", Some("session")),
            ("a", Some("b")),
            // The boundary: exactly MAX_LABEL once combined.
            (&"a".repeat(MAX_LABEL - 8), Some("session")),
        ];
        for (handle, alias) in cases {
            let account = validated_account(handle, *alias)
                .unwrap_or_else(|e| panic!("{handle:?}/{alias:?} rejected: {e}"));
            assert!(
                validated_account(&account, None).is_ok(),
                "account {account:?} came out of {handle:?}/{alias:?} but is not itself an \
                 accepted handle — the hint that names it would exit 3"
            );
        }
    }

    /// The other half of that invariant: a pair whose combination would break it
    /// is refused at `put` time, rather than stored and unrecoverable later.
    #[test]
    fn a_handle_and_alias_that_combine_past_the_limit_are_refused() {
        let err = validated_account(&"a".repeat(MAX_LABEL), Some("session")).unwrap_err();
        let AwareError::Validation(message) = &err else {
            panic!("expected Validation, got {err:?}");
        };
        // Names the real problem — the combination, not either part, both of
        // which are individually legal.
        assert!(message.contains("combine"), "{message}");
        // One character under the limit is accepted, so this is a boundary and
        // not a blanket refusal of aliases on long handles.
        assert!(validated_account(&"a".repeat(MAX_LABEL - 8), Some("session")).is_ok());
    }

    #[test]
    fn an_empty_or_overlong_handle_is_refused() {
        assert!(validated_account("", None).is_err());
        assert!(validated_account(&"a".repeat(MAX_LABEL + 1), None).is_err());
        // The boundary itself is accepted, so the limit is off-by-one-proof.
        assert!(validated_account(&"a".repeat(MAX_LABEL), None).is_ok());
    }

    /// A dot partitions a handle (`resolve_rest_credential` splits on the first
    /// one), so it is legal in a handle and not in an alias — an alias carrying
    /// one would silently re-partition the account.
    #[test]
    fn a_dot_is_legal_in_a_handle_and_refused_in_an_alias() {
        assert!(validated_account("my.api.key", None).is_ok());
        assert!(validated_account("my-api", Some("prod.eu")).is_err());
        assert!(validated_account("my-api", Some("prod")).is_ok());
    }

    /// `oauth-app.` is the keychain prefix BYO client secrets live under
    /// (`auth::keychain::app_account_name`). A credential stored there would
    /// overwrite one.
    #[test]
    fn the_oauth_app_prefix_is_reserved() {
        assert!(validated_account("oauth-app", None).is_err());
        assert!(validated_account("oauth-app.google-workspace", None).is_err());
        // Only the prefix as a whole part — a handle that merely starts with the
        // same letters is fine.
        assert!(validated_account("oauth-application", None).is_ok());
    }

    /// A registered integration is refused with the command that does own it.
    /// Storing one here would be shadowed at run time anyway: for a registered
    /// base, `resolve_rest_credential` prefers the refreshed OAuth token.
    #[test]
    fn a_registered_oauth_integration_is_refused_and_names_connect() {
        for handle in ["trimble-connect", "microsoft-365", "google-workspace"] {
            let err = validated_account(handle, None).unwrap_err();
            let AwareError::Validation(message) = &err else {
                panic!("expected Validation, got {err:?}");
            };
            assert!(message.contains("aware connect"), "{message}");
        }
        // The rule follows the runtime's own split, so an alias-qualified handle
        // over a registered base is caught too.
        assert!(validated_account("google-workspace.personal", None).is_err());
    }

    /// Rejected on every platform, not just Windows: a handle that worked on the
    /// author's Linux box and failed on a user's Windows one is worse than one
    /// refused everywhere.
    #[test]
    fn a_windows_reserved_device_name_is_refused_on_every_platform() {
        assert!(validated_account("con", None).is_err());
        assert!(validated_account("nul", None).is_err());
        // Reserved only as a whole dot-part.
        assert!(validated_account("console", None).is_ok());
        assert!(validated_account("my.con.key", None).is_err());
    }

    // ── secret reading ───────────────────────────────────────────────────────

    #[test]
    fn from_file_reads_the_secret_and_drops_the_trailing_newline() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("secret.txt");
        std::fs::write(&path, "tk_opaque_123\n").unwrap();
        let mut args = put_args("my-api");
        args.from_file = Some(path);
        assert_eq!(read_secret(&args, "my-api").unwrap(), "tk_opaque_123");
    }

    /// An empty secret is refused rather than stored. Storing it would report the
    /// handle as provisioned while every call it authorizes fails with an empty
    /// bearer — the same trap `connect --from-env` already guards.
    #[test]
    fn a_blank_secret_is_refused() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("empty.txt");
        std::fs::write(&path, "   \n\n").unwrap();
        let mut args = put_args("my-api");
        args.from_file = Some(path);
        let err = read_secret(&args, "my-api").unwrap_err();
        assert!(
            matches!(&err, AwareError::Validation(m) if m.contains("empty credential")),
            "{err:?}"
        );
    }

    /// The header-injection guard. A credential is interpolated into
    /// `Authorization: Bearer {cred}`, so an embedded CRLF would terminate that
    /// header and start attacker-chosen ones.
    #[test]
    fn a_secret_carrying_a_line_break_is_refused_before_it_reaches_a_header() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("injected.txt");
        std::fs::write(&path, "tk_abc\r\nX-Admin: true").unwrap();
        let mut args = put_args("my-api");
        args.from_file = Some(path);
        let err = read_secret(&args, "my-api").unwrap_err();
        assert!(
            matches!(&err, AwareError::Validation(m) if m.contains("control character")),
            "{err:?}"
        );
    }

    /// Interior spaces and non-ASCII survive: some issuers mint them, and neither
    /// can terminate a header. Without this the guard above would be a
    /// printable-ASCII rule refusing legitimate credentials.
    #[test]
    fn a_secret_with_an_interior_space_or_non_ascii_is_kept() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("spaced.txt");
        std::fs::write(&path, "  tk one \u{00e9}  ").unwrap();
        let mut args = put_args("my-api");
        args.from_file = Some(path);
        assert_eq!(read_secret(&args, "my-api").unwrap(), "tk one \u{00e9}");
    }

    #[test]
    fn from_file_reports_a_missing_path_as_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        let mut args = put_args("my-api");
        args.from_file = Some(tmp.path().join("absent.txt"));
        let err = read_secret(&args, "my-api").unwrap_err();
        assert!(matches!(err, AwareError::NotFound(_)), "{err:?}");
    }

    /// The variable name matches `connect --from-env` exactly for an alias-free
    /// handle, so a caller who knows one convention does not have to learn two.
    /// The issue's own repro sets `AWARE_TOKEN_FLOLESS_WORKSPACE_SESSION`.
    #[test]
    fn the_env_var_name_matches_the_connect_convention() {
        assert_eq!(env_var_name("microsoft-365"), "AWARE_TOKEN_MICROSOFT_365");
        assert_eq!(
            env_var_name("floless-workspace.session"),
            "AWARE_TOKEN_FLOLESS_WORKSPACE_SESSION"
        );
    }

    #[test]
    fn from_env_reads_the_derived_variable() {
        let _env = crate::test_env::EnvVarGuard::set("AWARE_TOKEN_MY_API", "  tk_env  ");
        let mut args = put_args("my-api");
        args.from_env = true;
        assert_eq!(read_secret(&args, "my-api").unwrap(), "tk_env");
    }

    /// With the variable absent the error names the variable it looked for and
    /// both other ways in — the user never typed that name, so they cannot be
    /// expected to guess its spelling.
    #[test]
    fn from_env_names_the_absent_variable_and_the_alternatives() {
        let _env = crate::test_env::EnvVarGuard::scope(&[("AWARE_TOKEN_MY_API", None)]);
        let mut args = put_args("my-api");
        args.from_env = true;
        let err = read_secret(&args, "my-api").unwrap_err();
        let AwareError::Validation(message) = &err else {
            panic!("expected Validation, got {err:?}");
        };
        assert!(message.contains("AWARE_TOKEN_MY_API"), "{message}");
        assert!(message.contains("--from-file"), "{message}");
        assert!(message.contains("stdin"), "{message}");
    }
}
