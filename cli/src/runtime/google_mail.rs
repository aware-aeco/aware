//! In-process `google-workspace.gmail.send` transport (#495).
//!
//! This deliberately does not reuse the generic REST renderer. The two Google
//! endpoints, bearer-token destinations, redirect policy, MIME construction,
//! and at-most-once journal all remain compile-time-owned by this module.

use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use base64::Engine;
use fs2::FileExt;
use mail_builder::MessageBuilder;
use mail_builder::headers::text::Text;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::auth::keychain::{StoredToken, TokenSource};
use crate::error::{AgentErrorDetails, AwareError};

const INTEGRATION: &str = "google-workspace";
const IDENTITY_URL: &str = "https://openidconnect.googleapis.com/v1/userinfo";
const GMAIL_SEND_URL: &str = "https://gmail.googleapis.com/gmail/v1/users/me/messages/send";
const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const OPENID_SCOPE: &str = "openid";
const EMAIL_SCOPE: &str = "https://www.googleapis.com/auth/userinfo.email";
const SEND_SCOPE: &str = "https://www.googleapis.com/auth/gmail.send";

const MAX_RECIPIENTS: usize = 100;
const MAX_ADDRESS_BYTES: usize = 320;
const MAX_SUBJECT_BYTES: usize = 512;
const MAX_BODY_BYTES: usize = 700 * 1024;
const MAX_ATTEMPT_ID_BYTES: usize = 128;
const MAX_ENCODED_MESSAGE_BYTES: usize = 1024 * 1024;
const MAX_RESPONSE_BYTES: usize = 64 * 1024;
const MAX_JOURNAL_BYTES: usize = 64 * 1024;
const REQUEST_DEADLINE: Duration = Duration::from_secs(30);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
struct GmailSendInput {
    to: Vec<String>,
    #[serde(default)]
    cc: Vec<String>,
    #[serde(default)]
    bcc: Vec<String>,
    subject: String,
    body: String,
    #[serde(default)]
    content_type: ContentType,
    attempt_id: String,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum ContentType {
    #[default]
    Text,
    Html,
}

#[derive(Debug, Deserialize)]
struct GoogleIdentity {
    sub: String,
    email: String,
    email_verified: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GmailSendResponse {
    id: Option<String>,
    thread_id: Option<String>,
}

#[derive(Debug)]
struct HttpResponse {
    status: u16,
    body: Vec<u8>,
}

trait GoogleHttp {
    fn identity(&self, bearer: &str) -> Result<HttpResponse, String>;
    fn send(&self, bearer: &str, request_body: &[u8]) -> Result<HttpResponse, String>;
}

struct PinnedGoogleHttp {
    agent: ureq::Agent,
}

impl PinnedGoogleHttp {
    fn new() -> Self {
        Self {
            agent: ureq::AgentBuilder::new()
                .redirects(0)
                .timeout_connect(Duration::from_secs(10))
                .timeout_write(Duration::from_secs(10))
                .timeout_read(Duration::from_secs(10))
                .timeout(REQUEST_DEADLINE)
                .build(),
        }
    }

    fn collect(result: Result<ureq::Response, ureq::Error>) -> Result<HttpResponse, String> {
        let response = match result {
            Ok(response) | Err(ureq::Error::Status(_, response)) => response,
            Err(ureq::Error::Transport(error)) => return Err(error.to_string()),
        };
        let status = response.status();
        let mut body = Vec::new();
        response
            .into_reader()
            .take((MAX_RESPONSE_BYTES + 1) as u64)
            .read_to_end(&mut body)
            .map_err(|error| format!("read response: {error}"))?;
        if body.len() > MAX_RESPONSE_BYTES {
            return Err(format!(
                "response exceeds the {MAX_RESPONSE_BYTES}-byte limit"
            ));
        }
        Ok(HttpResponse { status, body })
    }
}

impl GoogleHttp for PinnedGoogleHttp {
    fn identity(&self, bearer: &str) -> Result<HttpResponse, String> {
        Self::collect(
            self.agent
                .get(IDENTITY_URL)
                .set("Authorization", &format!("Bearer {bearer}"))
                .set("Accept", "application/json")
                .call(),
        )
    }

    fn send(&self, bearer: &str, request_body: &[u8]) -> Result<HttpResponse, String> {
        Self::collect(
            self.agent
                .post(GMAIL_SEND_URL)
                .set("Authorization", &format!("Bearer {bearer}"))
                .set("Accept", "application/json")
                .set("Content-Type", "application/json")
                .send_bytes(request_body),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum JournalState {
    Prepared,
    Dispatching,
    Accepted,
    Rejected,
    OutcomeUnknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JournalRecord {
    version: u8,
    state: JournalState,
    account_hash: String,
    attempt_hash: String,
    request_hash: String,
    credential_generation: Option<String>,
    rfc_message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    gmail_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_status: Option<u16>,
    timestamp: i64,
}

/// Dispatch `gmail.send` off the async reactor. Dropping/cancelling the caller
/// does not cancel the blocking closure, so a handed-off request still writes
/// its terminal journal record when the provider call returns.
pub async fn send(agents_dir: PathBuf, args: Value) -> Result<Value, AwareError> {
    tokio::task::spawn_blocking(move || send_blocking(&agents_dir, args, &PinnedGoogleHttp::new()))
        .await
        .map_err(|error| AwareError::Internal(format!("gmail send task join: {error}")))?
}

fn send_blocking(
    agents_dir: &Path,
    args: Value,
    http: &impl GoogleHttp,
) -> Result<Value, AwareError> {
    let input: GmailSendInput = serde_json::from_value(args).map_err(|error| {
        structured(
            "gmail.send.validation",
            "preflight",
            format!("invalid gmail.send input: {error}"),
            "gmail-send-input",
            None,
        )
    })?;
    validate_input(&input)?;

    let aware_home = agents_dir.parent().ok_or_else(|| {
        structured(
            "gmail.send.validation",
            "preflight",
            "agents directory has no AWARE home",
            "gmail-send-home",
            None,
        )
    })?;
    let manifest = crate::manifest::loader::load_agent_by_id(agents_dir, INTEGRATION)
        .map_err(|error| auth_error(format!("load Google Workspace manifest: {error}")))?;
    let auth = manifest
        .auth
        .ok_or_else(|| auth_error("Google Workspace manifest has no OAuth configuration"))?;
    if auth.scheme != "oauth2" {
        return Err(auth_error(
            "Google Workspace gmail.send requires an oauth2 credential",
        ));
    }
    let (integration, alias) = split_credential_handle(&auth.secret)?;
    if integration != INTEGRATION {
        return Err(auth_error(
            "Google Workspace gmail.send must use the google-workspace OAuth credential",
        ));
    }

    // A BYO OAuth app may replace client credentials and scopes, but this send
    // path never permits an endpoint overlay to receive the refresh token.
    let oauth = crate::auth::config::for_integration(INTEGRATION)?
        .with_profile(aware_home, alias)
        .map_err(|error| auth_error(format!("load Google OAuth configuration: {error}")))?;
    validate_token_endpoint(oauth.token_url())?;

    // Unlike the generic REST credential path, refresh failure is fatal here.
    let token = crate::auth::refresh::ensure_fresh(INTEGRATION, alias, aware_home)
        .map_err(|error| auth_error(format!("Google OAuth refresh failed: {error}")))?;
    validate_token(&token, alias)?;
    execute_authenticated(aware_home, &input, &token, http)
}

fn execute_authenticated(
    aware_home: &Path,
    input: &GmailSendInput,
    token: &StoredToken,
    http: &impl GoogleHttp,
) -> Result<Value, AwareError> {
    // A credential generation is not the durable namespace, but it is a safe
    // local hint for finding a journal whose OIDC-sub namespace was established
    // earlier. This makes an identical accepted replay require no provider call.
    if let Some(replayed) = replay_for_credential_generation(aware_home, input, token)? {
        return Ok(replayed);
    }
    let identity_response = http.identity(&token.access_token).map_err(|error| {
        structured(
            "gmail.send.identity",
            "identity",
            format!("Google identity request failed before dispatch: {error}"),
            "gmail-send-identity",
            None,
        )
    })?;
    if identity_response.status != 200 {
        return Err(structured(
            "gmail.send.identity",
            "identity",
            format!(
                "Google identity request failed before dispatch with HTTP {}",
                identity_response.status
            ),
            "gmail-send-identity",
            None,
        ));
    }
    let identity: GoogleIdentity =
        serde_json::from_slice(&identity_response.body).map_err(|_| {
            structured(
                "gmail.send.identity",
                "identity",
                "Google identity response was not valid JSON",
                "gmail-send-identity",
                None,
            )
        })?;
    validate_identity(&identity)?;

    let account_hash = sha256_hex(identity.sub.as_bytes());
    let attempt_hash = sha256_hex(input.attempt_id.as_bytes());
    let request_hash = canonical_request_hash(input)?;
    let generated_rfc_id = format!("<aware.{account_hash}.{attempt_hash}@aware.local>");
    let path = journal_path(aware_home, &account_hash, &attempt_hash)?;
    let (mut journal, is_new) = open_locked_journal(&path)?;
    let previous = read_last_record(&mut journal)?;

    let rfc_message_id = match previous {
        Some(ref record) => {
            validate_existing_record(record, &account_hash, &attempt_hash, &request_hash, input)?;
            match record.state {
                JournalState::Accepted => return accepted_output(input, record),
                JournalState::Rejected => {
                    return Err(rejected_error(
                        record.provider_status.unwrap_or(400),
                        &attempt_hash,
                    ));
                }
                JournalState::Dispatching | JournalState::OutcomeUnknown => {
                    return Err(reconciliation_error(
                        input,
                        &record.rfc_message_id,
                        &attempt_hash,
                    ));
                }
                JournalState::Prepared => record.rfc_message_id.clone(),
            }
        }
        None => generated_rfc_id,
    };

    if previous.is_none() {
        append_record(
            &mut journal,
            &JournalRecord {
                version: 1,
                state: JournalState::Prepared,
                account_hash: account_hash.clone(),
                attempt_hash: attempt_hash.clone(),
                request_hash: request_hash.clone(),
                credential_generation: token.generation.clone(),
                rfc_message_id: rfc_message_id.clone(),
                gmail_message_id: None,
                thread_id: None,
                provider_status: None,
                timestamp: chrono::Utc::now().timestamp(),
            },
        )?;
        if is_new {
            let parent = path
                .parent()
                .ok_or_else(|| outbox_error("outbox journal path has no account directory"))?;
            sync_parent(parent)?;
        }
    }

    // Identity resolution precedes MIME construction; From is never caller supplied.
    let raw = build_message(input, &identity.email, &rfc_message_id)?;
    let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(raw);
    if encoded.len() > MAX_ENCODED_MESSAGE_BYTES {
        return Err(validation_error(format!(
            "encoded message exceeds the {MAX_ENCODED_MESSAGE_BYTES}-byte limit"
        )));
    }
    let request_body = serde_json::to_vec(&json!({ "raw": encoded })).map_err(|error| {
        structured(
            "gmail.send.validation",
            "preflight",
            format!("could not encode Gmail request: {error}"),
            &attempt_hash,
            None,
        )
    })?;

    let mut dispatching = base_record(
        &account_hash,
        &attempt_hash,
        &request_hash,
        token,
        &rfc_message_id,
        JournalState::Dispatching,
    );
    append_record(&mut journal, &dispatching)?;

    let response = match http.send(&token.access_token, &request_body) {
        Ok(response) => response,
        Err(error) => {
            dispatching.state = JournalState::OutcomeUnknown;
            dispatching.timestamp = chrono::Utc::now().timestamp();
            let _ = append_record(&mut journal, &dispatching);
            return Err(outcome_unknown_error(
                input,
                &rfc_message_id,
                &attempt_hash,
                format!("Gmail send transport failed after handoff: {error}"),
            ));
        }
    };

    if (400..500).contains(&response.status) && response.status != 408 {
        dispatching.state = JournalState::Rejected;
        dispatching.provider_status = Some(response.status);
        dispatching.timestamp = chrono::Utc::now().timestamp();
        append_record(&mut journal, &dispatching)?;
        return Err(rejected_error(response.status, &attempt_hash));
    }

    if !(200..300).contains(&response.status) {
        dispatching.state = JournalState::OutcomeUnknown;
        dispatching.provider_status = Some(response.status);
        dispatching.timestamp = chrono::Utc::now().timestamp();
        let _ = append_record(&mut journal, &dispatching);
        return Err(outcome_unknown_error(
            input,
            &rfc_message_id,
            &attempt_hash,
            format!(
                "Gmail send outcome is unknown after HTTP {}",
                response.status
            ),
        ));
    }

    let parsed: GmailSendResponse = match serde_json::from_slice(&response.body) {
        Ok(parsed) => parsed,
        Err(_) => {
            dispatching.state = JournalState::OutcomeUnknown;
            dispatching.provider_status = Some(response.status);
            dispatching.timestamp = chrono::Utc::now().timestamp();
            let _ = append_record(&mut journal, &dispatching);
            return Err(outcome_unknown_error(
                input,
                &rfc_message_id,
                &attempt_hash,
                "Gmail accepted the request but returned an invalid response",
            ));
        }
    };
    let (Some(gmail_message_id), Some(thread_id)) = (parsed.id, parsed.thread_id) else {
        dispatching.state = JournalState::OutcomeUnknown;
        dispatching.provider_status = Some(response.status);
        dispatching.timestamp = chrono::Utc::now().timestamp();
        let _ = append_record(&mut journal, &dispatching);
        return Err(outcome_unknown_error(
            input,
            &rfc_message_id,
            &attempt_hash,
            "Gmail accepted the request but omitted a usable message or thread id",
        ));
    };
    if !bounded_provider_id(&gmail_message_id) || !bounded_provider_id(&thread_id) {
        dispatching.state = JournalState::OutcomeUnknown;
        dispatching.provider_status = Some(response.status);
        dispatching.timestamp = chrono::Utc::now().timestamp();
        let _ = append_record(&mut journal, &dispatching);
        return Err(outcome_unknown_error(
            input,
            &rfc_message_id,
            &attempt_hash,
            "Gmail accepted the request but returned an unusable message or thread id",
        ));
    }

    dispatching.state = JournalState::Accepted;
    dispatching.gmail_message_id = Some(gmail_message_id);
    dispatching.thread_id = Some(thread_id);
    dispatching.provider_status = Some(response.status);
    dispatching.timestamp = chrono::Utc::now().timestamp();
    if append_record(&mut journal, &dispatching).is_err() {
        return Err(outcome_unknown_error(
            input,
            &rfc_message_id,
            &attempt_hash,
            "Gmail accepted the request but its accepted state could not be durably recorded",
        ));
    }
    accepted_output(input, &dispatching)
}

fn replay_for_credential_generation(
    aware_home: &Path,
    input: &GmailSendInput,
    token: &StoredToken,
) -> Result<Option<Value>, AwareError> {
    let Some(generation) = token.generation.as_deref() else {
        return Ok(None);
    };
    let integration_dir = aware_home.join("outbox").join(INTEGRATION);
    if !integration_dir.exists() {
        return Ok(None);
    }
    ensure_private_dir(&aware_home.join("outbox"))?;
    ensure_private_dir(&integration_dir)?;
    let attempt_hash = sha256_hex(input.attempt_id.as_bytes());
    let request_hash = canonical_request_hash(input)?;
    let mut found: Option<Value> = None;

    for entry in std::fs::read_dir(&integration_dir)
        .map_err(|error| outbox_error(format!("read outbox accounts: {error}")))?
    {
        let entry = entry.map_err(|error| outbox_error(format!("read outbox account: {error}")))?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        if name.len() != 64 || !name.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            continue;
        }
        ensure_private_dir(&entry.path())?;
        let path = entry.path().join(format!("{attempt_hash}.jsonl"));
        if !path.exists() {
            continue;
        }
        let (mut journal, _) = open_locked_journal(&path)?;
        let Some(record) = read_last_record(&mut journal)? else {
            return Err(outbox_error("existing outbox journal is empty"));
        };
        if record.credential_generation.as_deref() != Some(generation) {
            continue;
        }
        validate_existing_record(&record, name, &attempt_hash, &request_hash, input)?;
        let replay = match record.state {
            JournalState::Accepted => Some(accepted_output(input, &record)?),
            JournalState::Rejected => {
                return Err(rejected_error(
                    record.provider_status.unwrap_or(400),
                    &attempt_hash,
                ));
            }
            JournalState::Dispatching | JournalState::OutcomeUnknown => {
                return Err(reconciliation_error(
                    input,
                    &record.rfc_message_id,
                    &attempt_hash,
                ));
            }
            JournalState::Prepared => None,
        };
        if let Some(replay) = replay
            && found.replace(replay).is_some()
        {
            return Err(outbox_error(
                "credential generation matched multiple outbox namespaces",
            ));
        }
    }
    Ok(found)
}

fn validate_input(input: &GmailSendInput) -> Result<(), AwareError> {
    if input.to.is_empty() {
        return Err(validation_error("`to` must contain at least one recipient"));
    }
    let count = input.to.len() + input.cc.len() + input.bcc.len();
    if count > MAX_RECIPIENTS {
        return Err(validation_error(format!(
            "recipient count exceeds the {MAX_RECIPIENTS}-recipient limit"
        )));
    }
    for address in input.to.iter().chain(&input.cc).chain(&input.bcc) {
        validate_address(address)?;
    }
    if input.subject.len() > MAX_SUBJECT_BYTES {
        return Err(validation_error(format!(
            "subject exceeds the {MAX_SUBJECT_BYTES}-byte limit"
        )));
    }
    if input.subject.chars().any(char::is_control) {
        return Err(validation_error("subject contains a control character"));
    }
    if input.body.len() > MAX_BODY_BYTES {
        return Err(validation_error(format!(
            "body exceeds the {MAX_BODY_BYTES}-byte limit"
        )));
    }
    let attempt = input.attempt_id.as_str();
    if attempt.is_empty() || attempt.len() > MAX_ATTEMPT_ID_BYTES {
        return Err(validation_error(format!(
            "`attempt-id` must be 1..={MAX_ATTEMPT_ID_BYTES} bytes"
        )));
    }
    if !attempt.is_ascii()
        || attempt
            .bytes()
            .any(|byte| byte.is_ascii_control() || byte == b' ')
    {
        return Err(validation_error(
            "`attempt-id` must be visible ASCII without whitespace",
        ));
    }
    Ok(())
}

fn validate_address(address: &str) -> Result<(), AwareError> {
    if address.is_empty()
        || address.len() > MAX_ADDRESS_BYTES
        || !address.is_ascii()
        || address.bytes().any(|byte| byte.is_ascii_control())
        || address.contains([' ', ',', ';', '<', '>'])
    {
        return Err(validation_error("recipient address is invalid or too long"));
    }
    let Some((local, domain)) = address.rsplit_once('@') else {
        return Err(validation_error("recipient address must contain `@`"));
    };
    if local.is_empty() || domain.is_empty() || domain.starts_with('.') || domain.ends_with('.') {
        return Err(validation_error("recipient address is malformed"));
    }
    Ok(())
}

fn split_credential_handle(secret: &str) -> Result<(&str, Option<&str>), AwareError> {
    let (integration, alias) = match secret.split_once('.') {
        Some((integration, alias)) if !alias.is_empty() => (integration, Some(alias)),
        Some(_) => return Err(auth_error("Google OAuth credential alias is empty")),
        None => (secret, None),
    };
    Ok((integration, alias))
}

fn validate_token_endpoint(url: &str) -> Result<(), AwareError> {
    if url != TOKEN_URL {
        return Err(auth_error(
            "gmail.send requires Google's pinned OAuth token endpoint; remove the token_url override from the Google OAuth profile",
        ));
    }
    Ok(())
}

fn validate_token(token: &StoredToken, alias: Option<&str>) -> Result<(), AwareError> {
    if token.integration != INTEGRATION
        || token.source != TokenSource::Oauth
        || !token.token_type.eq_ignore_ascii_case("bearer")
        || token.access_token.trim().is_empty()
        || token.access_token.chars().any(char::is_control)
    {
        return Err(auth_error(
            "gmail.send requires a nonblank Google OAuth bearer credential",
        ));
    }
    let actual = crate::auth::keychain::normalized_scopes(&token.scope);
    let mut required = vec![
        OPENID_SCOPE.to_string(),
        EMAIL_SCOPE.to_string(),
        SEND_SCOPE.to_string(),
    ];
    required.sort();
    if actual != required {
        let alias_hint = alias
            .map(|value| format!(" --as={value}"))
            .unwrap_or_default();
        return Err(auth_error(format!(
            "Google grant is missing required scopes or retains legacy broad scopes; run `aware disconnect {INTEGRATION}{alias_hint}` and reconnect with the narrowed Gmail grant"
        )));
    }
    Ok(())
}

fn validate_identity(identity: &GoogleIdentity) -> Result<(), AwareError> {
    if identity.sub.is_empty()
        || identity.sub.len() > 255
        || identity.sub.chars().any(char::is_control)
        || identity.email_verified != Some(true)
    {
        return Err(structured(
            "gmail.send.identity",
            "identity",
            "Google identity response did not contain a stable subject and verified email",
            "gmail-send-identity",
            None,
        ));
    }
    validate_address(&identity.email).map_err(|_| {
        structured(
            "gmail.send.identity",
            "identity",
            "Google identity response contained an invalid mailbox address",
            "gmail-send-identity",
            None,
        )
    })
}

fn canonical_request_hash(input: &GmailSendInput) -> Result<String, AwareError> {
    serde_json::to_vec(input)
        .map(|bytes| sha256_hex(&bytes))
        .map_err(|error| AwareError::Internal(format!("canonicalize Gmail request: {error}")))
}

fn build_message(
    input: &GmailSendInput,
    from: &str,
    rfc_message_id: &str,
) -> Result<Vec<u8>, AwareError> {
    let message_id = rfc_message_id
        .strip_prefix('<')
        .and_then(|value| value.strip_suffix('>'))
        .ok_or_else(|| AwareError::Internal("invalid generated RFC Message-ID".into()))?;
    let mut builder = MessageBuilder::new()
        .from(from.to_string())
        .to(input.to.clone())
        .subject(input.subject.clone())
        .message_id(message_id.to_string())
        .date(chrono::Utc::now().timestamp())
        .header("X-AWARE-Attempt-ID", Text::new(input.attempt_id.clone()));
    if !input.cc.is_empty() {
        builder = builder.cc(input.cc.clone());
    }
    // Gmail's send API has no separate envelope-recipient field. Bcc therefore
    // must be present in the submitted raw message; Gmail strips it on delivery.
    if !input.bcc.is_empty() {
        builder = builder.bcc(input.bcc.clone());
    }
    builder = match input.content_type {
        ContentType::Text => builder.text_body(input.body.clone()),
        ContentType::Html => builder.html_body(input.body.clone()),
    };
    builder.write_to_vec().map_err(|error| {
        structured(
            "gmail.send.validation",
            "preflight",
            format!("could not build RFC 5322 message: {error}"),
            "gmail-send-mime",
            None,
        )
    })
}

fn journal_path(
    aware_home: &Path,
    account_hash: &str,
    attempt_hash: &str,
) -> Result<PathBuf, AwareError> {
    let outbox = aware_home.join("outbox");
    let integration = outbox.join(INTEGRATION);
    let account = integration.join(account_hash);
    ensure_private_dir(&outbox)?;
    ensure_private_dir(&integration)?;
    ensure_private_dir(&account)?;
    Ok(account.join(format!("{attempt_hash}.jsonl")))
}

fn ensure_private_dir(path: &Path) -> Result<(), AwareError> {
    let existed = match std::fs::symlink_metadata(path) {
        Ok(_) => true,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(error) => return Err(outbox_error(format!("inspect {}: {error}", path.display()))),
    };
    if !existed {
        create_private_dir(path).map_err(|error| {
            outbox_error(format!(
                "create private directory {}: {error}",
                path.display()
            ))
        })?;
        if let Some(parent) = path.parent() {
            sync_parent(parent)?;
        }
    }
    let metadata = std::fs::symlink_metadata(path)
        .map_err(|error| outbox_error(format!("inspect {}: {error}", path.display())))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() || is_reparse_point(&metadata) {
        return Err(outbox_error(format!(
            "{} must be a private, real directory",
            path.display()
        )));
    }
    verify_private_dir_metadata(path, &metadata)?;
    Ok(())
}

#[cfg(unix)]
fn create_private_dir(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::DirBuilderExt;
    match std::fs::DirBuilder::new().mode(0o700).create(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
        Err(error) => Err(error),
    }
}

#[cfg(not(unix))]
fn create_private_dir(path: &Path) -> std::io::Result<()> {
    match std::fs::create_dir(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
        Err(error) => Err(error),
    }
}

#[cfg(unix)]
fn verify_private_dir_metadata(
    path: &Path,
    metadata: &std::fs::Metadata,
) -> Result<(), AwareError> {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};
    // SAFETY: `geteuid` has no pointer arguments or caller-side preconditions.
    let uid = unsafe { libc::geteuid() };
    if metadata.uid() != uid || metadata.permissions().mode() & 0o077 != 0 {
        return Err(outbox_error(format!(
            "{} must be owned by the effective user with mode 0700",
            path.display()
        )));
    }
    Ok(())
}

#[cfg(not(unix))]
fn verify_private_dir_metadata(
    path: &Path,
    _metadata: &std::fs::Metadata,
) -> Result<(), AwareError> {
    // AWARE_HOME is created below the current user's profile and inherits its
    // protected DACL. Reparse points are rejected above so inheritance cannot
    // be redirected to an attacker-selected location.
    let profile = std::env::var_os("USERPROFILE").map(PathBuf::from);
    if !profile.as_ref().is_some_and(|root| path.starts_with(root)) {
        return Err(outbox_error(format!(
            "{} is outside the current user's ACL-protected profile",
            path.display()
        )));
    }
    Ok(())
}

#[cfg(windows)]
fn is_reparse_point(metadata: &std::fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    metadata.file_attributes()
        & windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT
        != 0
}

#[cfg(not(windows))]
fn is_reparse_point(_: &std::fs::Metadata) -> bool {
    false
}

fn open_locked_journal(path: &Path) -> Result<(File, bool), AwareError> {
    let mut is_new = false;
    let file = match open_journal(path, true) {
        Ok(file) => {
            is_new = true;
            file
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            open_journal(path, false).map_err(|error| {
                outbox_error(format!("open outbox journal {}: {error}", path.display()))
            })?
        }
        Err(error) => {
            return Err(outbox_error(format!(
                "create outbox journal {}: {error}",
                path.display()
            )));
        }
    };
    file.lock_exclusive().map_err(|error| {
        outbox_error(format!("lock outbox journal {}: {error}", path.display()))
    })?;
    verify_private_file(path, &file)?;
    Ok((file, is_new))
}

#[cfg(unix)]
fn open_journal(path: &Path, create_new: bool) -> std::io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;
    OpenOptions::new()
        .read(true)
        .append(true)
        .create_new(create_new)
        .create(false)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
}

#[cfg(windows)]
fn open_journal(path: &Path, create_new: bool) -> std::io::Result<File> {
    use std::os::windows::fs::OpenOptionsExt;
    OpenOptions::new()
        .read(true)
        .append(true)
        .create_new(create_new)
        .create(false)
        .custom_flags(windows_sys::Win32::Storage::FileSystem::FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
}

#[cfg(not(any(unix, windows)))]
fn open_journal(path: &Path, create_new: bool) -> std::io::Result<File> {
    OpenOptions::new()
        .read(true)
        .append(true)
        .create_new(create_new)
        .create(false)
        .open(path)
}

fn verify_private_file(path: &Path, file: &File) -> Result<(), AwareError> {
    let metadata = file
        .metadata()
        .map_err(|error| outbox_error(format!("inspect {}: {error}", path.display())))?;
    if !metadata.is_file() || is_reparse_point(&metadata) {
        return Err(outbox_error(format!(
            "{} must be a private, real file",
            path.display()
        )));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        // SAFETY: `geteuid` has no pointer arguments or caller-side preconditions.
        let uid = unsafe { libc::geteuid() };
        if metadata.uid() != uid || metadata.permissions().mode() & 0o077 != 0 {
            return Err(outbox_error(format!(
                "{} must be owned by the effective user with mode 0600",
                path.display()
            )));
        }
    }
    Ok(())
}

fn read_last_record(file: &mut File) -> Result<Option<JournalRecord>, AwareError> {
    let len = file
        .metadata()
        .map_err(|error| outbox_error(format!("inspect outbox journal: {error}")))?
        .len() as usize;
    if len > MAX_JOURNAL_BYTES {
        return Err(outbox_error(format!(
            "outbox journal exceeds the {MAX_JOURNAL_BYTES}-byte limit"
        )));
    }
    file.seek(SeekFrom::Start(0))
        .map_err(|error| outbox_error(format!("seek outbox journal: {error}")))?;
    let reader = BufReader::new(&mut *file);
    let mut last = None;
    for line in reader.lines() {
        let line = line.map_err(|error| outbox_error(format!("read outbox journal: {error}")))?;
        if line.trim().is_empty() {
            continue;
        }
        last = Some(serde_json::from_str(&line).map_err(|_| {
            outbox_error("outbox journal contains a malformed record; refusing dispatch")
        })?);
    }
    file.seek(SeekFrom::End(0))
        .map_err(|error| outbox_error(format!("seek outbox journal: {error}")))?;
    Ok(last)
}

fn append_record(file: &mut File, record: &JournalRecord) -> Result<(), AwareError> {
    let mut line = serde_json::to_vec(record)
        .map_err(|error| AwareError::Internal(format!("serialize outbox record: {error}")))?;
    line.push(b'\n');
    file.write_all(&line)
        .and_then(|_| file.sync_all())
        .map_err(|error| outbox_error(format!("durably append outbox record: {error}")))
}

#[cfg(unix)]
fn sync_parent(path: &Path) -> Result<(), AwareError> {
    File::open(path)
        .and_then(|file| file.sync_all())
        .map_err(|error| outbox_error(format!("durably sync {}: {error}", path.display())))
}

#[cfg(not(unix))]
fn sync_parent(_: &Path) -> Result<(), AwareError> {
    // NTFS metadata is committed by the create/open + FlushFileBuffers on the
    // journal handle; Rust does not expose a portable directory handle sync.
    Ok(())
}

fn validate_existing_record(
    record: &JournalRecord,
    account_hash: &str,
    attempt_hash: &str,
    request_hash: &str,
    input: &GmailSendInput,
) -> Result<(), AwareError> {
    if record.version != 1
        || record.account_hash != account_hash
        || record.attempt_hash != attempt_hash
        || record.rfc_message_id.len() > 512
    {
        return Err(outbox_error(
            "outbox journal identity is invalid; refusing dispatch",
        ));
    }
    if record.request_hash != request_hash {
        return Err(structured(
            "gmail.send.attempt-conflict",
            "preflight",
            format!(
                "attempt-id {:?} was already used with different inputs",
                input.attempt_id
            ),
            attempt_hash,
            None,
        ));
    }
    Ok(())
}

fn base_record(
    account_hash: &str,
    attempt_hash: &str,
    request_hash: &str,
    token: &StoredToken,
    rfc_message_id: &str,
    state: JournalState,
) -> JournalRecord {
    JournalRecord {
        version: 1,
        state,
        account_hash: account_hash.to_string(),
        attempt_hash: attempt_hash.to_string(),
        request_hash: request_hash.to_string(),
        credential_generation: token.generation.clone(),
        rfc_message_id: rfc_message_id.to_string(),
        gmail_message_id: None,
        thread_id: None,
        provider_status: None,
        timestamp: chrono::Utc::now().timestamp(),
    }
}

fn accepted_output(input: &GmailSendInput, record: &JournalRecord) -> Result<Value, AwareError> {
    let gmail_id = record.gmail_message_id.as_deref().ok_or_else(|| {
        outbox_error("accepted outbox record has no Gmail message id; refusing dispatch")
    })?;
    let thread_id = record.thread_id.as_deref().ok_or_else(|| {
        outbox_error("accepted outbox record has no Gmail thread id; refusing dispatch")
    })?;
    Ok(json!({
        "status": "accepted",
        "message-id": gmail_id,
        "gmail-message-id": gmail_id,
        "thread-id": thread_id,
        "rfc-message-id": record.rfc_message_id,
        "attempt-id": input.attempt_id,
    }))
}

fn bounded_provider_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && value.is_ascii()
        && !value.bytes().any(|byte| byte.is_ascii_control())
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn validation_error(message: impl Into<String>) -> AwareError {
    structured(
        "gmail.send.validation",
        "preflight",
        message,
        "gmail-send-input",
        None,
    )
}

fn auth_error(message: impl Into<String>) -> AwareError {
    structured(
        "gmail.send.auth",
        "preflight",
        message,
        "gmail-send-auth",
        None,
    )
}

fn outbox_error(message: impl Into<String>) -> AwareError {
    structured(
        "gmail.send.outbox",
        "preflight",
        message,
        "gmail-send-outbox",
        None,
    )
}

fn rejected_error(status: u16, diagnostic_id: &str) -> AwareError {
    structured(
        "gmail.send.rejected",
        "dispatch",
        format!("Gmail definitively rejected the send request with HTTP {status}"),
        diagnostic_id,
        None,
    )
}

fn correlation_details(input: &GmailSendInput, rfc_message_id: &str) -> BTreeMap<String, String> {
    BTreeMap::from([
        ("attemptId".into(), input.attempt_id.clone()),
        ("rfcMessageId".into(), rfc_message_id.to_string()),
    ])
}

fn reconciliation_error(
    input: &GmailSendInput,
    rfc_message_id: &str,
    diagnostic_id: &str,
) -> AwareError {
    structured(
        "gmail.send.outcome-unknown",
        "preflight",
        "this attempt may already have been handed to Gmail; reconcile it instead of retrying",
        diagnostic_id,
        Some(correlation_details(input, rfc_message_id)),
    )
}

fn outcome_unknown_error(
    input: &GmailSendInput,
    rfc_message_id: &str,
    diagnostic_id: &str,
    message: impl Into<String>,
) -> AwareError {
    structured(
        "gmail.send.outcome-unknown",
        "dispatch",
        message,
        diagnostic_id,
        Some(correlation_details(input, rfc_message_id)),
    )
}

fn structured(
    code: &str,
    phase: &str,
    message: impl Into<String>,
    diagnostic_id: &str,
    details: Option<BTreeMap<String, String>>,
) -> AwareError {
    let mut message = message.into();
    message.truncate(512);
    AwareError::AgentStructured {
        code: code.into(),
        phase: phase.into(),
        retryable: false,
        message,
        diagnostic_id: diagnostic_id.chars().take(128).collect(),
        details: details.map(|value| Box::new(AgentErrorDetails(value))),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use mail_parser::MessageParser;

    use super::*;

    struct MockHttp {
        identity: HttpResponse,
        identity_calls: Mutex<usize>,
        sends: Mutex<Vec<Vec<u8>>>,
        send_results: Mutex<Vec<Result<HttpResponse, String>>>,
    }

    impl MockHttp {
        fn responding(result: Result<HttpResponse, String>) -> Self {
            Self {
                identity: HttpResponse {
                    status: 200,
                    body: br#"{"sub":"stable-google-sub","email":"sender@example.com","email_verified":true}"#.to_vec(),
                },
                identity_calls: Mutex::new(0),
                sends: Mutex::new(Vec::new()),
                send_results: Mutex::new(vec![result]),
            }
        }

        fn send_count(&self) -> usize {
            self.sends.lock().unwrap().len()
        }

        fn identity_count(&self) -> usize {
            *self.identity_calls.lock().unwrap()
        }
    }

    impl GoogleHttp for MockHttp {
        fn identity(&self, _: &str) -> Result<HttpResponse, String> {
            *self.identity_calls.lock().unwrap() += 1;
            Ok(HttpResponse {
                status: self.identity.status,
                body: self.identity.body.clone(),
            })
        }

        fn send(&self, _: &str, body: &[u8]) -> Result<HttpResponse, String> {
            self.sends.lock().unwrap().push(body.to_vec());
            self.send_results.lock().unwrap().remove(0)
        }
    }

    fn input(attempt: &str) -> GmailSendInput {
        GmailSendInput {
            to: vec!["to@example.com".into()],
            cc: vec!["cc@example.com".into()],
            bcc: vec!["bcc@example.com".into()],
            subject: "RFI café".into(),
            body: "Hello, 世界".into(),
            content_type: ContentType::Text,
            attempt_id: attempt.into(),
        }
    }

    fn token(generation: &str) -> StoredToken {
        StoredToken {
            access_token: "test-token".into(),
            refresh_token: Some("refresh".into()),
            expires_at: i64::MAX,
            scope: format!("{OPENID_SCOPE} {EMAIL_SCOPE} {SEND_SCOPE}"),
            token_type: "Bearer".into(),
            integration: INTEGRATION.into(),
            obtained_at: 1,
            generation: Some(generation.into()),
            source: TokenSource::Oauth,
        }
    }

    fn accepted() -> Result<HttpResponse, String> {
        Ok(HttpResponse {
            status: 200,
            body: br#"{"id":"gmail-123","threadId":"thread-456"}"#.to_vec(),
        })
    }

    fn code(error: &AwareError) -> &str {
        match error {
            AwareError::AgentStructured { code, .. } => code,
            other => panic!("not structured: {other:?}"),
        }
    }

    #[test]
    fn frozen_input_rejects_missing_attempt_and_attachments() {
        let missing = serde_json::from_value::<GmailSendInput>(json!({
            "to": ["to@example.com"], "subject": "s", "body": "b"
        }));
        assert!(missing.is_err());
        let attachment = serde_json::from_value::<GmailSendInput>(json!({
            "to": ["to@example.com"], "subject": "s", "body": "b",
            "attempt-id": "a", "attachments": []
        }));
        assert!(attachment.is_err());
        let mut invalid = input("bad\r\nBcc:x@example.com");
        assert_eq!(
            code(&validate_input(&invalid).unwrap_err()),
            "gmail.send.validation"
        );
        invalid.attempt_id = "ok".into();
        invalid.to[0] = "victim@example.com\r\nBcc:x@example.com".into();
        assert!(validate_input(&invalid).is_err());
    }

    #[test]
    fn oauth_preflight_requires_exact_narrowed_scope_set() {
        validate_token(&token("one"), None).unwrap();
        let mut broad = token("two");
        broad
            .scope
            .push_str(" https://www.googleapis.com/auth/drive");
        let error = validate_token(&broad, Some("work")).unwrap_err();
        assert_eq!(code(&error), "gmail.send.auth");
        assert!(
            error
                .to_string()
                .contains("aware disconnect google-workspace --as=work")
        );
        let mut pasted = token("three");
        pasted.source = TokenSource::Paste;
        assert!(validate_token(&pasted, None).is_err());
        validate_token_endpoint(TOKEN_URL).unwrap();
        assert_eq!(
            code(&validate_token_endpoint("https://example.invalid/token").unwrap_err()),
            "gmail.send.auth"
        );
    }

    #[test]
    fn stable_subject_not_credential_generation_namespaces_the_outbox() {
        let account = sha256_hex(b"stable-google-sub");
        assert_eq!(account, sha256_hex(b"stable-google-sub"));
        assert_ne!(account, sha256_hex(b"credential-generation-one"));
        assert_ne!(account, sha256_hex(b"credential-generation-two"));
    }

    #[test]
    fn maintained_builder_output_parses_independently_with_all_headers() {
        let input = input("floless-rfi-001-email-v1");
        let rfc_id = "<aware.test@example.invalid>";
        let raw = build_message(&input, "sender@example.com", rfc_id).unwrap();
        assert!(raw.windows(2).any(|bytes| bytes == b"\r\n"));
        let parsed = MessageParser::new().parse(&raw).expect("valid RFC message");
        assert_eq!(parsed.subject(), Some("RFI café"));
        assert_eq!(parsed.message_id(), Some("aware.test@example.invalid"));
        assert_eq!(
            parsed.header("X-AWARE-Attempt-ID").unwrap().as_text(),
            Some("floless-rfi-001-email-v1")
        );
        assert!(parsed.header("Bcc").is_some());
        assert_eq!(parsed.body_text(0).as_deref(), Some("Hello, 世界"));
    }

    #[test]
    fn accepted_attempt_is_cached_without_a_second_send() {
        let home = tempfile::tempdir().unwrap();
        let mock = MockHttp::responding(accepted());
        let first =
            execute_authenticated(home.path(), &input("attempt-accepted"), &token("g1"), &mock)
                .unwrap();
        let second =
            execute_authenticated(home.path(), &input("attempt-accepted"), &token("g1"), &mock)
                .unwrap();
        assert_eq!(mock.send_count(), 1);
        assert_eq!(mock.identity_count(), 1);
        assert_eq!(first, second);
        assert_eq!(first["message-id"], "gmail-123");
        assert_eq!(first["gmail-message-id"], "gmail-123");
        assert_eq!(first["thread-id"], "thread-456");

        // A reconnect changes credential generation, but the stable OIDC-sub
        // namespace still finds the accepted record after one identity call.
        let rotated =
            execute_authenticated(home.path(), &input("attempt-accepted"), &token("g2"), &mock)
                .unwrap();
        assert_eq!(rotated, first);
        assert_eq!(mock.send_count(), 1);
        assert_eq!(mock.identity_count(), 2);
    }

    #[test]
    fn attempt_reuse_with_different_input_is_a_conflict() {
        let home = tempfile::tempdir().unwrap();
        let mock = MockHttp::responding(accepted());
        execute_authenticated(home.path(), &input("attempt-conflict"), &token("g1"), &mock)
            .unwrap();
        let mut changed = input("attempt-conflict");
        changed.body = "different".into();
        let error = execute_authenticated(home.path(), &changed, &token("g2"), &mock).unwrap_err();
        assert_eq!(code(&error), "gmail.send.attempt-conflict");
        assert_eq!(mock.send_count(), 1);
    }

    #[test]
    fn transport_failure_is_unknown_and_same_attempt_cannot_send_again() {
        let home = tempfile::tempdir().unwrap();
        let mock = MockHttp::responding(Err("30 second deadline exceeded".into()));
        let error =
            execute_authenticated(home.path(), &input("attempt-unknown"), &token("g1"), &mock)
                .unwrap_err();
        assert_eq!(code(&error), "gmail.send.outcome-unknown");
        assert!(!error.structured_agent_error().unwrap().retryable);
        let replay =
            execute_authenticated(home.path(), &input("attempt-unknown"), &token("g1"), &mock)
                .unwrap_err();
        assert_eq!(code(&replay), "gmail.send.outcome-unknown");
        assert_eq!(mock.send_count(), 1);
        assert_eq!(mock.identity_count(), 1);
    }

    #[test]
    fn only_definitive_4xx_is_rejected() {
        for (status, expected) in [
            (400, "gmail.send.rejected"),
            (408, "gmail.send.outcome-unknown"),
            (500, "gmail.send.outcome-unknown"),
            (302, "gmail.send.outcome-unknown"),
        ] {
            let home = tempfile::tempdir().unwrap();
            let mock = MockHttp::responding(Ok(HttpResponse {
                status,
                body: vec![],
            }));
            let error = execute_authenticated(
                home.path(),
                &input(&format!("attempt-{status}")),
                &token("g"),
                &mock,
            )
            .unwrap_err();
            assert_eq!(code(&error), expected, "HTTP {status}");
            assert_eq!(mock.send_count(), 1);
        }
    }

    #[test]
    fn unusable_success_response_is_outcome_unknown() {
        let home = tempfile::tempdir().unwrap();
        let mock = MockHttp::responding(Ok(HttpResponse {
            status: 200,
            body: br#"{"threadId":"thread-only"}"#.to_vec(),
        }));
        let error = execute_authenticated(
            home.path(),
            &input("attempt-bad-success"),
            &token("g"),
            &mock,
        )
        .unwrap_err();
        assert_eq!(code(&error), "gmail.send.outcome-unknown");
    }

    #[test]
    fn encoded_message_limit_fails_before_handoff() {
        let home = tempfile::tempdir().unwrap();
        let mock = MockHttp::responding(accepted());
        let mut oversized = input("attempt-oversized");
        oversized.body = "x".repeat(MAX_BODY_BYTES + 1);
        let error = validate_input(&oversized).unwrap_err();
        assert_eq!(code(&error), "gmail.send.validation");
        assert_eq!(mock.send_count(), 0);
        drop(home);
    }

    #[cfg(unix)]
    #[test]
    fn outbox_components_and_journal_are_owner_only() {
        use std::os::unix::fs::PermissionsExt;
        let home = tempfile::tempdir().unwrap();
        let mock = MockHttp::responding(accepted());
        execute_authenticated(
            home.path(),
            &input("attempt-permissions"),
            &token("g"),
            &mock,
        )
        .unwrap();
        let account = sha256_hex(b"stable-google-sub");
        let attempt = sha256_hex(b"attempt-permissions");
        let dir = home.path().join("outbox").join(INTEGRATION).join(account);
        assert_eq!(
            std::fs::metadata(&dir).unwrap().permissions().mode() & 0o777,
            0o700
        );
        assert_eq!(
            std::fs::metadata(dir.join(format!("{attempt}.jsonl")))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }
}
