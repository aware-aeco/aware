//! Complete, run-scoped model report renderer. The legacy `render` stays unchanged.
//! Input and output are bounded records/artifacts, never one model-sized JSON value.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::error::AwareError;
use crate::runtime::artifact_stream::{
    ArtifactRef, RunArtifactScope, budget_from_env, require_reservation,
};

const MAX_RECORD: usize = 128 * 1024;
const MAX_HEADER: usize = 64 * 1024;
const TARGET_FRAGMENT: usize = 256 * 1024;
const MAX_FRAGMENT: usize = 1024 * 1024;
const CONTENT_TYPE: &str = "application/vnd.aware.html-report.bundle.v1";
const NONE: u64 = u64::MAX;

fn invalid(message: &str) -> AwareError {
    AwareError::Validation(format!("model report: {message}"))
}

fn number(value: &Value, key: &str) -> Result<u64, AwareError> {
    value
        .get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| invalid(&format!("missing or invalid {key}")))
}

fn string<'a>(value: &'a Value, key: &str) -> Result<&'a str, AwareError> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| invalid(&format!("missing or invalid {key}")))
}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

fn display(value: Option<&Value>) -> String {
    match value {
        None | Some(Value::Null) => String::new(),
        Some(Value::String(s)) => escape(s),
        Some(v) => escape(&v.to_string()),
    }
}

/// Read one bounded NDJSON line, including its newline for the terminal hash.
fn record_line<R: BufRead>(reader: &mut R) -> Result<Option<Vec<u8>>, AwareError> {
    let mut out = Vec::new();
    loop {
        let available = reader.fill_buf()?;
        if available.is_empty() {
            if out.is_empty() {
                return Ok(None);
            }
            return Err(invalid("record is missing its newline"));
        }
        let count = available
            .iter()
            .position(|b| *b == b'\n')
            .map_or(available.len(), |i| i + 1);
        if out.len().saturating_add(count) > MAX_RECORD {
            return Err(invalid("a model record is too large to read safely"));
        }
        let done = available[count - 1] == b'\n';
        out.extend_from_slice(&available[..count]);
        reader.consume(count);
        if done {
            return Ok(Some(out));
        }
    }
}

fn parse_record(line: &[u8]) -> Result<Value, AwareError> {
    serde_json::from_slice(line).map_err(|_| invalid("record is not valid JSON"))
}

struct SourceBits {
    file: tempfile::NamedTempFile,
    items: u64,
    seen: u64,
}

impl SourceBits {
    fn new(dir: &Path, items: u64) -> Result<Self, AwareError> {
        let file = tempfile::NamedTempFile::new_in(dir)?;
        file.as_file().set_len(items.div_ceil(8))?;
        Ok(Self {
            file,
            items,
            seen: 0,
        })
    }

    fn mark(&mut self, ordinal: u64) -> Result<(), AwareError> {
        if ordinal >= self.items {
            return Err(invalid("source row ordinal is outside its signed receipt"));
        }
        let file = self.file.as_file_mut();
        file.seek(SeekFrom::Start(ordinal / 8))?;
        let mut byte = [0u8; 1];
        file.read_exact(&mut byte)?;
        let bit = 1u8 << (ordinal % 8);
        if byte[0] & bit != 0 {
            return Err(invalid("source row appears more than once"));
        }
        byte[0] |= bit;
        file.seek(SeekFrom::Start(ordinal / 8))?;
        file.write_all(&byte)?;
        self.seen += 1;
        Ok(())
    }

    fn complete(&self) -> bool {
        self.seen == self.items
    }
}

/// Exact disk-backed ID set: hashes select slots, but stored original bytes
/// decide equality, including in the event of a hash collision.
struct IdentitySet {
    table: tempfile::NamedTempFile,
    ids: tempfile::NamedTempFile,
    slots: u64,
    log_bytes: u64,
}

impl IdentitySet {
    fn table_bytes(items: u64) -> Result<u64, AwareError> {
        let slots = items
            .checked_mul(2)
            .ok_or_else(|| invalid("identity table size overflow"))?
            .max(1)
            .checked_next_power_of_two()
            .ok_or_else(|| invalid("identity table size overflow"))?;
        slots
            .checked_mul(48)
            .ok_or_else(|| invalid("identity table size overflow"))
    }

    fn new(dir: &Path, items: u64) -> Result<Self, AwareError> {
        let slots = Self::table_bytes(items)? / 48;
        let table = tempfile::NamedTempFile::new_in(dir)?;
        table.as_file().set_len(slots * 48)?;
        Ok(Self {
            table,
            ids: tempfile::NamedTempFile::new_in(dir)?,
            slots,
            log_bytes: 0,
        })
    }

    fn insert(&mut self, id: &str, bundle: &mut Bundle) -> Result<(), AwareError> {
        if id.is_empty() || id.len() > MAX_RECORD {
            return Err(invalid("object or property id is invalid"));
        }
        let digest = Sha256::digest(id.as_bytes());
        let mut first = [0u8; 8];
        first.copy_from_slice(&digest[..8]);
        let mut slot = u64::from_le_bytes(first) & (self.slots - 1);
        for _ in 0..self.slots {
            let table = self.table.as_file_mut();
            table.seek(SeekFrom::Start(slot * 48))?;
            let mut entry = [0u8; 48];
            table.read_exact(&mut entry)?;
            if entry[44] == 0 {
                bundle.reserve_external(id.len() as u64)?;
                let offset = self.log_bytes;
                self.ids.write_all(id.as_bytes())?;
                self.log_bytes += id.len() as u64;
                entry[..32].copy_from_slice(&digest);
                entry[32..40].copy_from_slice(&offset.to_le_bytes());
                entry[40..44].copy_from_slice(&(id.len() as u32).to_le_bytes());
                entry[44] = 1;
                table.seek(SeekFrom::Start(slot * 48))?;
                table.write_all(&entry)?;
                return Ok(());
            }
            if entry[..32] == digest[..] {
                let offset = u64::from_le_bytes(
                    entry[32..40]
                        .try_into()
                        .map_err(|_| invalid("identity index corrupt"))?,
                );
                let len = u32::from_le_bytes(
                    entry[40..44]
                        .try_into()
                        .map_err(|_| invalid("identity index corrupt"))?,
                ) as usize;
                if len == id.len() {
                    let mut previous = vec![0u8; len];
                    self.ids.as_file_mut().seek(SeekFrom::Start(offset))?;
                    self.ids.as_file_mut().read_exact(&mut previous)?;
                    if previous == id.as_bytes() {
                        return Err(invalid(
                            "object or property identity appears more than once",
                        ));
                    }
                }
            }
            slot = (slot + 1) & (self.slots - 1);
        }
        Err(invalid("identity index is full"))
    }
}

struct BoundedWriter {
    file: tempfile::NamedTempFile,
    hash: Sha256,
    bytes: u64,
    budget: u64,
}

impl BoundedWriter {
    fn new(dir: &Path, budget: u64) -> Result<Self, AwareError> {
        Ok(Self {
            file: tempfile::NamedTempFile::new_in(dir)?,
            hash: Sha256::new(),
            bytes: 0,
            budget,
        })
    }
    fn write(&mut self, bytes: &[u8]) -> Result<(), AwareError> {
        let next = self
            .bytes
            .checked_add(bytes.len() as u64)
            .ok_or_else(|| invalid("bundle size overflow"))?;
        if next > self.budget {
            return Err(invalid("not enough space reserved to complete this report"));
        }
        self.file.write_all(bytes)?;
        self.hash.update(bytes);
        self.bytes = next;
        Ok(())
    }
    fn finish(self, scope: &RunArtifactScope) -> Result<ArtifactRef, AwareError> {
        scope.publish(
            self.file,
            self.bytes,
            format!("{:x}", self.hash.finalize()),
            CONTENT_TYPE,
        )
    }
}

#[derive(Clone, Copy)]
struct Range {
    first_object: u64,
    last_object: u64,
    first_property: u64,
    last_property: u64,
}

impl Default for Range {
    fn default() -> Self {
        Self {
            first_object: NONE,
            last_object: NONE,
            first_property: NONE,
            last_property: NONE,
        }
    }
}

impl Range {
    fn object(&mut self, n: u64) {
        if self.first_object == NONE {
            self.first_object = n;
        }
        self.last_object = n;
    }
    fn property(&mut self, n: u64) {
        if self.first_property == NONE {
            self.first_property = n;
        }
        self.last_property = n;
    }
}

struct Bundle {
    writer: BoundedWriter,
    index: tempfile::NamedTempFile,
    total_budget: u64,
    external_bytes: u64,
    chunk_count: u64,
    fragment: String,
    range: Range,
    table_open: bool,
    current_object: Option<u64>,
    rows_in_fragment: usize,
}

impl Bundle {
    fn new(
        scope: &RunArtifactScope,
        budget: u64,
        external_bytes: u64,
        header: &Value,
    ) -> Result<Self, AwareError> {
        let header = serde_json::to_vec(header)?;
        if header.len() > MAX_HEADER {
            return Err(invalid("bundle header is too large"));
        }
        let mut writer = BoundedWriter::new(scope.dir(), budget)?;
        writer.write(b"AWRPTB1\n")?;
        writer.write(&(header.len() as u32).to_le_bytes())?;
        writer.write(&header)?;
        Ok(Self {
            writer,
            index: tempfile::NamedTempFile::new_in(scope.dir())?,
            total_budget: budget,
            external_bytes,
            chunk_count: 0,
            fragment: String::new(),
            range: Range::default(),
            table_open: false,
            current_object: None,
            rows_in_fragment: 0,
        })
    }

    fn close_table(&mut self) {
        if self.table_open {
            self.fragment.push_str("</tbody></table></div>");
            self.table_open = false;
        }
    }

    fn reserve_external(&mut self, bytes: u64) -> Result<(), AwareError> {
        let next = self
            .external_bytes
            .checked_add(bytes)
            .ok_or_else(|| invalid("report workspace size overflow"))?;
        let index_bytes = self
            .chunk_count
            .checked_mul(76)
            .ok_or_else(|| invalid("bundle index size overflow"))?;
        let peak = self
            .writer
            .bytes
            .checked_add(self.fragment.len() as u64)
            .and_then(|n| n.checked_add(index_bytes.saturating_mul(2)))
            .and_then(|n| n.checked_add(next))
            .and_then(|n| n.checked_add(80))
            .ok_or_else(|| invalid("report workspace size overflow"))?;
        if peak > self.total_budget {
            return Err(invalid("not enough space reserved to complete this report"));
        }
        self.external_bytes = next;
        Ok(())
    }

    fn flush(&mut self) -> Result<(), AwareError> {
        self.close_table();
        if self.fragment.is_empty() {
            return Ok(());
        }
        if self.current_object.is_some() && !self.fragment.ends_with("</section>") {
            self.fragment.push_str("</section>");
        }
        if self.fragment.len() > MAX_FRAGMENT {
            return Err(invalid("HTML fragment exceeds its safe size"));
        }
        let bytes = self.fragment.as_bytes();
        // At peak the temporary index and its final copied footer coexist.
        // Reserve both copies before writing this fragment or index entry.
        let next_index = self
            .chunk_count
            .checked_add(1)
            .and_then(|n| n.checked_mul(76))
            .ok_or_else(|| invalid("bundle index size overflow"))?;
        let peak = self
            .writer
            .bytes
            .checked_add(4 + bytes.len() as u64)
            .and_then(|n| n.checked_add(next_index.saturating_mul(2)))
            .and_then(|n| n.checked_add(self.external_bytes))
            .and_then(|n| n.checked_add(80))
            .ok_or_else(|| invalid("bundle size overflow"))?;
        if peak > self.total_budget {
            return Err(invalid("not enough space reserved to complete this report"));
        }
        let offset = self.writer.bytes + 4;
        self.writer.write(&(bytes.len() as u32).to_le_bytes())?;
        self.writer.write(bytes)?;
        let mut h = Sha256::new();
        h.update(bytes);
        let mut entry = Vec::with_capacity(76);
        entry.extend_from_slice(&offset.to_le_bytes());
        entry.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        entry.extend_from_slice(&h.finalize());
        for n in [
            self.range.first_object,
            self.range.last_object,
            self.range.first_property,
            self.range.last_property,
        ] {
            entry.extend_from_slice(&n.to_le_bytes());
        }
        self.index.write_all(&entry)?;
        self.chunk_count += 1;
        self.fragment.clear();
        self.range = Range::default();
        self.rows_in_fragment = 0;
        Ok(())
    }

    fn heading(&mut self, ordinal: u64, heading: &str, continued: bool) {
        self.range.object(ordinal);
        self.fragment
            .push_str("<section class=\"model-object\"><h2>");
        self.fragment.push_str(heading);
        if continued {
            self.fragment.push_str(" <small>(continued)</small>");
        }
        self.fragment.push_str("</h2>");
    }

    fn start_object(&mut self, ordinal: u64, heading: String) -> Result<(), AwareError> {
        self.flush()?;
        self.current_object = Some(ordinal);
        self.heading(ordinal, &heading, false);
        Ok(())
    }

    fn property(&mut self, ordinal: u64, row: &str) -> Result<(), AwareError> {
        if self.fragment.len() + row.len() > TARGET_FRAGMENT || self.rows_in_fragment >= 32 {
            self.flush()?;
            if let Some(object) = self.current_object {
                self.heading(object, &format!("Object {}", object + 1), true);
            }
        }
        if !self.table_open {
            self.fragment.push_str("<div class=\"table-wrapper\"><table><thead><tr><th>Group</th><th>Property</th><th>Value</th><th>Unit</th><th>Provenance</th></tr></thead><tbody>");
            self.table_open = true;
        }
        self.fragment.push_str(row);
        self.range.property(ordinal);
        self.rows_in_fragment += 1;
        Ok(())
    }

    fn end_object(&mut self, properties: u64) -> Result<(), AwareError> {
        self.close_table();
        if properties == 0 {
            self.fragment
                .push_str("<p class=\"empty\">No readable properties.</p>");
        }
        self.fragment.push_str("</section>");
        self.current_object = None;
        self.flush()
    }

    fn provenance(&mut self, text: &str) -> Result<(), AwareError> {
        if self.fragment.len() + text.len() > TARGET_FRAGMENT {
            self.flush()?;
        }
        self.fragment.push_str(text);
        Ok(())
    }

    fn finish(
        mut self,
        scope: &RunArtifactScope,
        objects: u64,
        properties: u64,
    ) -> Result<ArtifactRef, AwareError> {
        if self.chunk_count == 0 && self.fragment.is_empty() {
            self.fragment
                .push_str("<p class=\"empty\">This model has no readable objects.</p>");
        }
        self.flush()?;
        let footer_offset = self.writer.bytes;
        let mut footer_hash = Sha256::new();
        let mut prefix = Vec::with_capacity(32);
        prefix.extend_from_slice(b"AWRPTIDX");
        for n in [self.chunk_count, objects, properties] {
            prefix.extend_from_slice(&n.to_le_bytes());
        }
        self.writer.write(&prefix)?;
        footer_hash.update(&prefix);
        let mut index = File::open(self.index.path())?;
        let mut block = [0u8; 64 * 1024];
        loop {
            let n = index.read(&mut block)?;
            if n == 0 {
                break;
            }
            self.writer.write(&block[..n])?;
            footer_hash.update(&block[..n]);
        }
        self.writer.write(b"AWRPTEND")?;
        self.writer.write(&footer_offset.to_le_bytes())?;
        self.writer.write(&footer_hash.finalize())?;
        self.writer.finish(scope)
    }
}

fn make_document(
    title: &str,
    provider: &str,
    revision: &str,
    entities: u64,
    properties: u64,
) -> (String, String) {
    let title = escape(title);
    let prefix = format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width\"><title>{title}</title><style>{}</style></head><body><main class=\"container\"><header class=\"report-header\"><h1>{title}</h1><p class=\"subtitle\">{} · revision {} · read-only · source unchanged</p><p>{entities} objects · {properties} properties · complete report</p></header>",
        super::html_report::STYLE,
        escape(provider),
        escape(revision)
    );
    (prefix, "<footer class=\"report-footer\">Complete read-only model-property report.</footer></main></body></html>".into())
}

fn field<'a>(value: &'a Value, names: &[&str]) -> Option<&'a Value> {
    names.iter().find_map(|name| value.get(*name))
}

fn property_row(property: &Value, source_ordinal: u64) -> String {
    format!(
        "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>Signed source row {source_ordinal}</td></tr>",
        display(field(property, &["groupName", "group", "category"])),
        display(field(property, &["name", "parameterName", "id"])),
        display(field(property, &["value", "displayValue"])),
        display(field(property, &["unit", "units"]))
    )
}

fn object_heading(entity: &Value, id: &str) -> String {
    let label = display(field(entity, &["name", "typeName", "family", "category"]));
    if label.is_empty() {
        escape(id)
    } else {
        format!("{} <small>({})</small>", label, escape(id))
    }
}

fn exact_role<'a>(receipts: &'a [Value], role: &str) -> Result<&'a Value, AwareError> {
    let matches: Vec<_> = receipts
        .iter()
        .filter(|r| r.get("role").and_then(Value::as_str) == Some(role))
        .collect();
    if matches.len() != 1 {
        return Err(invalid(
            "receipt list does not contain exactly one shard for each role",
        ));
    }
    let receipt = matches[0];
    if number(receipt, "ordinal")? != 0 || string(receipt, "digest")?.len() != 64 {
        return Err(invalid("receipt shard metadata is invalid"));
    }
    let _ = number(receipt, "bytes")?;
    Ok(receipt)
}

/// Terminal builtin output: a scoped bundle descriptor and exact counts only.
pub fn render_stream(
    args: &Value,
    artifact_dir: Option<&Path>,
    preview: bool,
) -> Result<Value, AwareError> {
    if let Some(columns) = args.get("columns")
        && *columns != json!(["Group", "Property", "Value", "Unit", "Provenance"])
    {
        return Err(invalid(
            "report columns must be Group, Property, Value, Unit, Provenance",
        ));
    }
    if preview {
        return Err(invalid("complete reports require a real run"));
    }
    require_reservation()?;
    let dir = artifact_dir.ok_or_else(|| invalid("run artifact directory is missing"))?;
    let scope = RunArtifactScope::from_dir(dir)?;
    let claim = scope.claim("render")?;
    let render_budget = budget_from_env("AWARE_REPORT_RENDER_BYTES")?;
    let result = render_stream_inner(args, &scope, render_budget)?;
    claim.commit();
    Ok(result)
}

fn render_stream_inner(
    args: &Value,
    scope: &RunArtifactScope,
    render_budget: u64,
) -> Result<Value, AwareError> {
    let descriptor = args
        .get("data")
        .and_then(|d| d.get("artifact"))
        .or_else(|| args.get("artifact"))
        .ok_or_else(|| invalid("stream artifact reference is missing"))?;
    let descriptor: ArtifactRef = serde_json::from_value(descriptor.clone())
        .map_err(|_| invalid("stream artifact reference is invalid"))?;
    let source = scope.open_verified(&descriptor, "application/x-ndjson")?;
    let mut reader = BufReader::with_capacity(64 * 1024, source);
    let first = record_line(&mut reader)?.ok_or_else(|| invalid("record stream is empty"))?;
    let mut prior_hash = Sha256::new();
    prior_hash.update(&first);
    let mut prior_bytes = first.len() as u64;
    let header = parse_record(&first)?;
    if string(&header, "type")? != "header"
        || string(&header, "schemaVersion")? != "floless.complete-model-report/v1"
    {
        return Err(invalid("record stream header has the wrong version"));
    }
    let expected = header
        .get("expected")
        .ok_or_else(|| invalid("expected totals are missing"))?;
    let entity_total = number(expected, "entities")?;
    let property_total = number(expected, "properties")?;
    let relationship_total = number(expected, "relationships")?;
    let receipts = header
        .get("receipts")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid("receipt list is missing"))?;
    if receipts.len() != 3 {
        return Err(invalid("receipt list is incomplete"));
    }
    for (role, count) in [
        ("entities", entity_total),
        ("properties", property_total),
        ("relationships", relationship_total),
    ] {
        if number(exact_role(receipts, role)?, "items")? != count {
            return Err(invalid("signed receipt and header totals disagree"));
        }
    }
    let bit_bytes = entity_total
        .div_ceil(8)
        .checked_add(property_total.div_ceil(8))
        .and_then(|n| n.checked_add(relationship_total.div_ceil(8)))
        .ok_or_else(|| invalid("source coverage size overflow"))?;
    let table_bytes = IdentitySet::table_bytes(entity_total)?
        .checked_add(IdentitySet::table_bytes(property_total)?)
        .ok_or_else(|| invalid("identity table size overflow"))?;
    let external_bytes = bit_bytes
        .checked_add(table_bytes)
        .ok_or_else(|| invalid("report workspace size overflow"))?;
    if external_bytes >= render_budget {
        return Err(invalid(
            "not enough space reserved to verify the complete report",
        ));
    }
    let mut bits = BTreeMap::new();
    for (role, count) in [
        ("entities", entity_total),
        ("properties", property_total),
        ("relationships", relationship_total),
    ] {
        bits.insert(role, SourceBits::new(scope.dir(), count)?);
    }
    let mut entity_ids = IdentitySet::new(scope.dir(), entity_total)?;
    let mut property_ids = IdentitySet::new(scope.dir(), property_total)?;
    let provider = string(&header, "provider")?;
    let revision = string(&header, "revisionId")?;
    if !matches!(provider, "revit" | "tekla") || revision.is_empty() {
        return Err(invalid("model provider or approved revision is invalid"));
    }
    let title = args
        .get("title")
        .and_then(Value::as_str)
        .unwrap_or("Model properties");
    let (document_prefix, document_suffix) =
        make_document(title, provider, revision, entity_total, property_total);
    let bundle_header = json!({"schemaVersion":"aware.html-report.bundle/v1", "documentPrefix":document_prefix,
        "documentSuffix":document_suffix,"provider":provider,"revisionId":revision,"objects":entity_total,"properties":property_total});
    let mut bundle = Bundle::new(scope, render_budget, external_bytes, &bundle_header)?;
    bundle.reserve_external(0)?;
    let mut entities = 0u64;
    let mut properties = 0u64;
    let mut relationships = 0u64;
    let mut current_id: Option<String> = None;
    let mut current_properties = 0u64;
    let mut terminal_seen = false;
    let mut model_provenance_seen = false;
    while let Some(line) = record_line(&mut reader)? {
        let record = parse_record(&line)?;
        let kind = string(&record, "type")?;
        if kind == "terminal" {
            let observed = record
                .get("observed")
                .ok_or_else(|| invalid("terminal totals are missing"))?;
            if !model_provenance_seen
                || current_id.is_some()
                || number(&record, "priorBytes")? != prior_bytes
                || string(&record, "priorSha256")?.to_ascii_lowercase()
                    != format!("{:x}", prior_hash.finalize())
                || number(observed, "entities")? != entities
                || number(observed, "properties")? != properties
                || number(observed, "relationships")? != relationships
                || entities != entity_total
                || properties != property_total
                || relationships != relationship_total
                || !bits.values().all(SourceBits::complete)
            {
                return Err(invalid(
                    "the complete report receipt does not match its records",
                ));
            }
            if record_line(&mut reader)?.is_some() {
                return Err(invalid("unexpected data after the terminal receipt"));
            }
            terminal_seen = true;
            break;
        }
        prior_hash.update(&line);
        prior_bytes = prior_bytes
            .checked_add(line.len() as u64)
            .ok_or_else(|| invalid("stream byte count overflow"))?;
        match kind {
            "model-provenance" => {
                if model_provenance_seen || entities != 0 || properties != 0 || relationships != 0 {
                    return Err(invalid("model provenance must appear once before objects"));
                }
                for key in ["sourceSha256", "manifestRoot", "projectUuid", "revisionId"] {
                    if string(&record, key)? != string(&header, key)? {
                        return Err(invalid(
                            "model provenance does not match the approved revision",
                        ));
                    }
                }
                let fingerprint = string(&record, "providerFingerprint")?;
                bundle.provenance(&format!(
                    "<section><h2>Model provenance</h2><p>Provider fingerprint: {}</p></section>",
                    escape(fingerprint)
                ))?;
                model_provenance_seen = true;
            }
            "entity-start" => {
                if !model_provenance_seen {
                    return Err(invalid("model provenance is missing"));
                }
                if current_id.is_some()
                    || number(&record, "entityOrdinal")? != entities
                    || number(&record, "recordOrdinal")? != entities
                    || number(&record, "shardOrdinal")? != 0
                {
                    return Err(invalid("object order is incomplete or duplicated"));
                }
                bits.get_mut("entities")
                    .ok_or_else(|| invalid("entity coverage unavailable"))?
                    .mark(number(&record, "sourceOrdinal")?)?;
                let id = string(&record, "id")?.to_owned();
                entity_ids.insert(&id, &mut bundle)?;
                let entity = record
                    .get("entity")
                    .ok_or_else(|| invalid("object data is missing"))?;
                if entity
                    .get("id")
                    .and_then(Value::as_str)
                    .is_some_and(|native| native != id)
                {
                    return Err(invalid("object identity disagrees with its source row"));
                }
                bundle.start_object(entities, object_heading(entity, &id))?;
                current_id = Some(id);
                current_properties = 0;
                entities += 1;
            }
            "property" => {
                let id = current_id
                    .as_deref()
                    .ok_or_else(|| invalid("property has no current object"))?;
                if string(&record, "ownerId")? != id
                    || number(&record, "entityOrdinal")? != entities - 1
                    || number(&record, "propertyOrdinal")? != current_properties
                    || number(&record, "recordOrdinal")? != properties
                    || number(&record, "shardOrdinal")? != 0
                {
                    return Err(invalid(
                        "property owner or order is incomplete or duplicated",
                    ));
                }
                let source_ordinal = number(&record, "sourceOrdinal")?;
                bits.get_mut("properties")
                    .ok_or_else(|| invalid("property coverage unavailable"))?
                    .mark(source_ordinal)?;
                let property = record
                    .get("property")
                    .ok_or_else(|| invalid("property data is missing"))?;
                let property_id = string(&record, "id")?;
                property_ids.insert(property_id, &mut bundle)?;
                if property
                    .get("entityId")
                    .and_then(Value::as_str)
                    .is_some_and(|owner| owner != id)
                    || property
                        .get("id")
                        .and_then(Value::as_str)
                        .is_some_and(|native| native != property_id)
                {
                    return Err(invalid("property identity disagrees with its source row"));
                }
                bundle.property(properties, &property_row(property, source_ordinal))?;
                properties += 1;
                current_properties += 1;
            }
            "entity-end" => {
                let id = current_id
                    .as_deref()
                    .ok_or_else(|| invalid("object end has no start"))?;
                if string(&record, "id")? != id
                    || number(&record, "entityOrdinal")? != entities - 1
                    || number(&record, "propertyCount")? != current_properties
                {
                    return Err(invalid("object property count is incomplete"));
                }
                bundle.end_object(current_properties)?;
                current_id = None;
            }
            "provenance" => {
                if current_id.is_some()
                    || number(&record, "recordOrdinal")? != relationships
                    || number(&record, "shardOrdinal")? != 0
                {
                    return Err(invalid("model provenance order is incomplete"));
                }
                bits.get_mut("relationships")
                    .ok_or_else(|| invalid("provenance coverage unavailable"))?
                    .mark(number(&record, "sourceOrdinal")?)?;
                let data = record
                    .get("relationship")
                    .ok_or_else(|| invalid("model provenance data is missing"))?;
                bundle.provenance(&format!(
                    "<section><h2>Model provenance</h2><p>{}</p></section>",
                    display(Some(data))
                ))?;
                relationships += 1;
            }
            _ => return Err(invalid("record type is unsupported")),
        }
    }
    if !terminal_seen {
        return Err(invalid("terminal receipt is missing"));
    }
    let reference = bundle.finish(scope, entities, properties)?;
    Ok(
        json!({"bundle":reference,"objectCount":entities,"propertyCount":properties,"complete":true}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scope(root: &Path, run: &str) -> RunArtifactScope {
        let dir = root
            .join("app")
            .join("instance")
            .join(format!("{run}.artifacts"));
        std::fs::create_dir_all(&dir).expect("artifact directory");
        RunArtifactScope::from_dir(&dir).expect("scope")
    }

    fn sample_stream() -> Vec<u8> {
        let header = json!({"type":"header","schemaVersion":"floless.complete-model-report/v1",
            "provider":"revit","projectUuid":"project-1","revisionId":"rev-1","manifestRoot":"root-1",
            "sourceSha256":"source-1","expected":{"entities":1,"properties":1,"relationships":0},
            "receipts":[
                {"role":"entities","digest":"a".repeat(64),"bytes":10,"items":1,"ordinal":0},
                {"role":"properties","digest":"b".repeat(64),"bytes":10,"items":1,"ordinal":0},
                {"role":"relationships","digest":"c".repeat(64),"bytes":0,"items":0,"ordinal":0}]});
        let rows = [
            header,
            json!({"type":"model-provenance","providerFingerprint":"fingerprint-1","sourceSha256":"source-1","manifestRoot":"root-1","projectUuid":"project-1","revisionId":"rev-1"}),
            json!({"type":"entity-start","shardOrdinal":0,"recordOrdinal":0,"sourceOrdinal":0,"entityOrdinal":0,"id":"e1","entity":{"name":"<Beam>"}}),
            json!({"type":"property","shardOrdinal":0,"recordOrdinal":0,"sourceOrdinal":0,"entityOrdinal":0,"propertyOrdinal":0,"id":"p1","ownerId":"e1","property":{"groupName":"Dimensions","name":"Length","value":"<script>alert(1)</script>","unit":"mm"}}),
            json!({"type":"entity-end","entityOrdinal":0,"id":"e1","propertyCount":1}),
        ];
        let mut bytes = Vec::new();
        for row in rows {
            bytes.extend_from_slice(row.to_string().as_bytes());
            bytes.push(b'\n');
        }
        let mut hash = Sha256::new();
        hash.update(&bytes);
        let terminal = json!({"type":"terminal","observed":{"entities":1,"properties":1,"relationships":0},"priorBytes":bytes.len(),"priorSha256":format!("{:x}",hash.finalize())});
        bytes.extend_from_slice(terminal.to_string().as_bytes());
        bytes.push(b'\n');
        bytes
    }

    fn many_property_stream(count: u64) -> Vec<u8> {
        let header = json!({"type":"header","schemaVersion":"floless.complete-model-report/v1",
            "provider":"tekla","projectUuid":"project-1","revisionId":"rev-1","manifestRoot":"root-1",
            "sourceSha256":"source-1","expected":{"entities":1,"properties":count,"relationships":0},
            "receipts":[
                {"role":"entities","digest":"a".repeat(64),"bytes":10,"items":1,"ordinal":0},
                {"role":"properties","digest":"b".repeat(64),"bytes":10,"items":count,"ordinal":0},
                {"role":"relationships","digest":"c".repeat(64),"bytes":0,"items":0,"ordinal":0}]});
        let mut bytes = Vec::new();
        for row in [
            header,
            json!({"type":"model-provenance","providerFingerprint":"fingerprint-1","sourceSha256":"source-1","manifestRoot":"root-1","projectUuid":"project-1","revisionId":"rev-1"}),
            json!({"type":"entity-start","shardOrdinal":0,"recordOrdinal":0,"sourceOrdinal":0,"entityOrdinal":0,"id":"e1","entity":{"id":"e1","name":"Beam"}}),
        ] {
            bytes.extend_from_slice(row.to_string().as_bytes());
            bytes.push(b'\n');
        }
        for ordinal in 0..count {
            let row = json!({"type":"property","shardOrdinal":0,"recordOrdinal":ordinal,"sourceOrdinal":ordinal,
                "entityOrdinal":0,"propertyOrdinal":ordinal,"id":format!("p{ordinal}"),"ownerId":"e1",
                "property":{"entityId":"e1","id":format!("p{ordinal}"),"groupName":"Dimensions","name":format!("Attribute {ordinal}"),"value":ordinal,"unit":"mm"}});
            bytes.extend_from_slice(row.to_string().as_bytes());
            bytes.push(b'\n');
        }
        let end = json!({"type":"entity-end","entityOrdinal":0,"id":"e1","propertyCount":count});
        bytes.extend_from_slice(end.to_string().as_bytes());
        bytes.push(b'\n');
        let mut hash = Sha256::new();
        hash.update(&bytes);
        let terminal = json!({"type":"terminal","observed":{"entities":1,"properties":count,"relationships":0},
            "priorBytes":bytes.len(),"priorSha256":format!("{:x}",hash.finalize())});
        bytes.extend_from_slice(terminal.to_string().as_bytes());
        bytes.push(b'\n');
        bytes
    }

    #[test]
    fn complete_report_bundle_is_indexed_escaped_and_pathless() {
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope(temp.path(), "run-1");
        let source = sample_stream();
        let descriptor = scope
            .write(&mut source.as_slice(), 1_000_000, "application/x-ndjson")
            .expect("spool");
        let output = render_stream_inner(
            &json!({"artifact":descriptor,"title":"Model report"}),
            &scope,
            2_000_000,
        )
        .expect("report");
        assert_eq!(output["complete"], true);
        assert_eq!(output["objectCount"], 1);
        assert_eq!(output["propertyCount"], 1);
        assert!(output.get("html").is_none());
        let bundle: ArtifactRef =
            serde_json::from_value(output["bundle"].clone()).expect("bundle ref");
        let mut file = scope.open_verified(&bundle, CONTENT_TYPE).expect("bundle");
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).expect("read bundle");
        assert!(bytes.starts_with(b"AWRPTB1\n"));
        assert!(bytes.windows(12).any(|w| w == b"&lt;Beam&gt;"));
        assert!(!bytes.windows(7).any(|w| w == b"<script"));
        assert!(bytes.windows(8).any(|w| w == b"AWRPTIDX"));
        assert!(bytes.windows(8).any(|w| w == b"AWRPTEND"));
        let read_u64 =
            |offset: usize| u64::from_le_bytes(bytes[offset..offset + 8].try_into().expect("u64"));
        let header_len =
            u32::from_le_bytes(bytes[8..12].try_into().expect("header length")) as usize;
        let header: Value =
            serde_json::from_slice(&bytes[12..12 + header_len]).expect("header JSON");
        assert_eq!(header["schemaVersion"], "aware.html-report.bundle/v1");
        let trailer = bytes.len() - 48;
        assert_eq!(&bytes[trailer..trailer + 8], b"AWRPTEND");
        let footer_offset = read_u64(trailer + 8) as usize;
        assert_eq!(&bytes[footer_offset..footer_offset + 8], b"AWRPTIDX");
        let mut digest = Sha256::new();
        digest.update(&bytes[footer_offset..trailer]);
        assert_eq!(&digest.finalize()[..], &bytes[trailer + 16..]);
        let chunks = read_u64(footer_offset + 8);
        assert!(chunks >= 2, "metadata and object fragments are indexed");
        assert_eq!(read_u64(footer_offset + 16), 1);
        assert_eq!(read_u64(footer_offset + 24), 1);
        assert_eq!(footer_offset + 32 + chunks as usize * 76, trailer);
        let mut full_html = header["documentPrefix"]
            .as_str()
            .expect("prefix")
            .to_owned();
        for index in 0..chunks as usize {
            let entry = footer_offset + 32 + index * 76;
            let offset = read_u64(entry) as usize;
            let len = u32::from_le_bytes(bytes[entry + 8..entry + 12].try_into().expect("length"))
                as usize;
            assert!(len <= MAX_FRAGMENT);
            assert_eq!(
                u32::from_le_bytes(bytes[offset - 4..offset].try_into().expect("frame")) as usize,
                len
            );
            let fragment = &bytes[offset..offset + len];
            let mut fragment_hash = Sha256::new();
            fragment_hash.update(fragment);
            assert_eq!(
                &fragment_hash.finalize()[..],
                &bytes[entry + 12..entry + 44]
            );
            full_html.push_str(std::str::from_utf8(fragment).expect("HTML fragment"));
        }
        full_html.push_str(header["documentSuffix"].as_str().expect("suffix"));
        assert!(full_html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
        assert!(full_html.contains("Signed source row 0"));
    }

    #[test]
    fn cross_run_reference_and_missing_terminal_refuse() {
        let temp = tempfile::tempdir().expect("temp");
        let first = scope(temp.path(), "run-1");
        let second = scope(temp.path(), "run-2");
        let source = sample_stream();
        let descriptor = first
            .write(&mut source.as_slice(), 1_000_000, "application/x-ndjson")
            .expect("spool");
        assert!(
            second
                .open_verified(&descriptor, "application/x-ndjson")
                .is_err()
        );
        let mut incomplete = source;
        let last_newline = incomplete
            .iter()
            .rposition(|b| *b == b'\n')
            .expect("last newline");
        let previous_newline = incomplete[..last_newline]
            .iter()
            .rposition(|b| *b == b'\n')
            .expect("previous newline");
        incomplete.truncate(previous_newline + 1);
        let partial = first
            .write(
                &mut incomplete.as_slice(),
                1_000_000,
                "application/x-ndjson",
            )
            .expect("partial spool");
        assert!(render_stream_inner(&json!({"artifact":partial}), &first, 2_000_000).is_err());
    }

    #[test]
    fn changed_property_or_insufficient_quota_publishes_no_bundle() {
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope(temp.path(), "run-1");
        let mut tampered = sample_stream();
        let where_property = tampered
            .windows(6)
            .position(|w| w == b"Length")
            .expect("property");
        tampered[where_property] = b'X';
        let reference = scope
            .write(&mut tampered.as_slice(), 1_000_000, "application/x-ndjson")
            .expect("spool");
        assert!(render_stream_inner(&json!({"artifact":reference}), &scope, 2_000_000).is_err());
        assert_eq!(
            std::fs::read_dir(scope.dir()).expect("entries").count(),
            1,
            "failed report left a completed artifact"
        );

        let valid = sample_stream();
        let reference = scope
            .write(&mut valid.as_slice(), 1_000_000, "application/x-ndjson")
            .expect("spool");
        assert!(render_stream_inner(&json!({"artifact":reference}), &scope, 100).is_err());
        assert_eq!(
            std::fs::read_dir(scope.dir()).expect("entries").count(),
            2,
            "quota failure left a completed artifact"
        );
    }

    #[test]
    fn source_coverage_bitset_refuses_duplicate_and_out_of_range_ordinals() {
        let temp = tempfile::tempdir().expect("temp");
        let mut bits = SourceBits::new(temp.path(), 2).expect("bitset");
        bits.mark(0).expect("first row");
        assert!(!bits.complete());
        assert!(bits.mark(0).is_err());
        assert!(bits.mark(2).is_err());
        bits.mark(1).expect("second row");
        assert!(bits.complete());
    }

    #[test]
    fn duplicate_property_identity_with_distinct_source_ordinals_refuses() {
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope(temp.path(), "run-1");
        let original = String::from_utf8(many_property_stream(2)).expect("utf8");
        let mut lines: Vec<String> = original.lines().map(str::to_owned).collect();
        let _terminal = lines.pop().expect("terminal");
        let second = lines
            .iter_mut()
            .find(|line| line.contains("\"propertyOrdinal\":1"))
            .expect("second property");
        *second = second.replace("\"p1\"", "\"p0\"");
        let mut bytes = lines.join("\n").into_bytes();
        bytes.push(b'\n');
        let mut hash = Sha256::new();
        hash.update(&bytes);
        let terminal = json!({"type":"terminal","observed":{"entities":1,"properties":2,"relationships":0},
            "priorBytes":bytes.len(),"priorSha256":format!("{:x}",hash.finalize())});
        bytes.extend_from_slice(terminal.to_string().as_bytes());
        bytes.push(b'\n');
        let descriptor = scope
            .write(&mut bytes.as_slice(), 1_000_000, "application/x-ndjson")
            .expect("spool");
        let error = render_stream_inner(&json!({"artifact":descriptor}), &scope, 2_000_000)
            .expect_err("duplicate must refuse");
        assert!(
            error
                .to_string()
                .contains("identity appears more than once")
        );
        assert_eq!(std::fs::read_dir(scope.dir()).expect("entries").count(), 1);
    }

    #[test]
    fn one_object_with_five_thousand_properties_reaches_every_chunk() {
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope(temp.path(), "run-1");
        let source = many_property_stream(5_000);
        let descriptor = scope
            .write(&mut source.as_slice(), 8_000_000, "application/x-ndjson")
            .expect("spool");
        let output = render_stream_inner(&json!({"artifact":descriptor}), &scope, 16_000_000)
            .expect("report");
        assert_eq!(output["objectCount"], 1);
        assert_eq!(output["propertyCount"], 5_000);
        let bundle: ArtifactRef =
            serde_json::from_value(output["bundle"].clone()).expect("bundle ref");
        let mut file = scope.open_verified(&bundle, CONTENT_TYPE).expect("bundle");
        file.seek(SeekFrom::End(-48)).expect("trailer");
        let mut trailer = [0u8; 48];
        file.read_exact(&mut trailer).expect("read trailer");
        let footer_offset = u64::from_le_bytes(trailer[8..16].try_into().expect("offset"));
        file.seek(SeekFrom::Start(footer_offset + 8))
            .expect("index");
        let mut count = [0u8; 8];
        file.read_exact(&mut count).expect("chunks");
        assert!(
            u64::from_le_bytes(count) > 100,
            "large object should span bounded chunks"
        );
    }

    /// A minimal stream that renders cleanly: header, model provenance, one
    /// object with one property, and one relationship row. Every refusal test
    /// below changes exactly one field of this, so the error it observes can
    /// only come from the guard it names.
    fn valid_rows() -> Vec<Value> {
        vec![
            json!({"type":"header","schemaVersion":"floless.complete-model-report/v1",
                "provider":"revit","projectUuid":"project-1","revisionId":"rev-1","manifestRoot":"root-1",
                "sourceSha256":"source-1","expected":{"entities":1,"properties":1,"relationships":1},
                "receipts":[
                    {"role":"entities","digest":"a".repeat(64),"bytes":10,"items":1,"ordinal":0},
                    {"role":"properties","digest":"b".repeat(64),"bytes":10,"items":1,"ordinal":0},
                    {"role":"relationships","digest":"c".repeat(64),"bytes":10,"items":1,"ordinal":0}]}),
            json!({"type":"model-provenance","providerFingerprint":"revit-2026 <build 3>",
                "sourceSha256":"source-1","manifestRoot":"root-1","projectUuid":"project-1","revisionId":"rev-1"}),
            json!({"type":"entity-start","shardOrdinal":0,"recordOrdinal":0,"sourceOrdinal":0,
                "entityOrdinal":0,"id":"e1","entity":{"id":"e1","name":"Beam"}}),
            json!({"type":"property","shardOrdinal":0,"recordOrdinal":0,"sourceOrdinal":0,
                "entityOrdinal":0,"propertyOrdinal":0,"id":"p1","ownerId":"e1",
                "property":{"entityId":"e1","id":"p1","groupName":"Dimensions","name":"Length","value":1200,"unit":"mm"}}),
            json!({"type":"entity-end","entityOrdinal":0,"id":"e1","propertyCount":1}),
            json!({"type":"provenance","shardOrdinal":0,"recordOrdinal":0,"sourceOrdinal":0,
                "relationship":"Hosted by <Level 1>"}),
        ]
    }

    /// Frame `rows` as NDJSON and seal them with a terminal receipt derived from
    /// the bytes actually written, then hand that receipt to `tamper`. Deriving
    /// it keeps every negative test honest: the stream fails the guard under
    /// test, not the digest.
    fn seal_with(rows: &[Value], tamper: impl FnOnce(&mut Value)) -> Vec<u8> {
        let mut bytes = Vec::new();
        for row in rows {
            bytes.extend_from_slice(row.to_string().as_bytes());
            bytes.push(b'\n');
        }
        let count = |kind: &str| rows.iter().filter(|row| row["type"] == kind).count() as u64;
        let mut hash = Sha256::new();
        hash.update(&bytes);
        let mut terminal = json!({"type":"terminal",
            "observed":{"entities":count("entity-start"),"properties":count("property"),
                "relationships":count("provenance")},
            "priorBytes":bytes.len(),"priorSha256":format!("{:x}",hash.finalize())});
        tamper(&mut terminal);
        bytes.extend_from_slice(terminal.to_string().as_bytes());
        bytes.push(b'\n');
        bytes
    }

    fn seal(rows: &[Value]) -> Vec<u8> {
        seal_with(rows, |_| {})
    }

    fn render_bytes(scope: &RunArtifactScope, bytes: &[u8]) -> Result<Value, AwareError> {
        let descriptor = scope
            .write(&mut &bytes[..], 4_000_000, "application/x-ndjson")
            .expect("spool");
        render_stream_inner(&json!({ "artifact": descriptor }), scope, 8_000_000)
    }

    /// `valid_rows()` with one field of the row of `kind` replaced.
    fn with_field(kind: &str, key: &str, value: Value) -> Vec<Value> {
        let mut rows = valid_rows();
        let row = rows
            .iter_mut()
            .find(|row| row["type"] == kind)
            .unwrap_or_else(|| panic!("no {kind} row"));
        row[key] = value;
        rows
    }

    /// Assert the stream is refused *for the stated reason*. Matching the
    /// message, not just `is_err`, is what stops a mutated guard from passing
    /// because some later guard happened to catch the same row.
    fn refuses(rows: &[Value], reason: &str) {
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope(temp.path(), "run-1");
        let error = render_bytes(&scope, &seal(rows)).expect_err(reason);
        assert!(
            error.to_string().contains(reason),
            "expected a refusal mentioning {reason:?}, got {error}"
        );
    }

    /// Reassemble the document the bundle describes, so tests can assert on
    /// what a reader sees rather than on frame offsets.
    fn bundle_html(scope: &RunArtifactScope, output: &Value) -> String {
        let reference: ArtifactRef =
            serde_json::from_value(output["bundle"].clone()).expect("bundle ref");
        let mut file = scope
            .open_verified(&reference, CONTENT_TYPE)
            .expect("bundle");
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).expect("read bundle");
        let read_u64 =
            |offset: usize| u64::from_le_bytes(bytes[offset..offset + 8].try_into().expect("u64"));
        let read_u32 = |offset: usize| {
            u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("u32")) as usize
        };
        let header: Value =
            serde_json::from_slice(&bytes[12..12 + read_u32(8)]).expect("header JSON");
        let trailer = bytes.len() - 48;
        let footer = read_u64(trailer + 8) as usize;
        let chunks = read_u64(footer + 8) as usize;
        let mut html = header["documentPrefix"]
            .as_str()
            .expect("prefix")
            .to_owned();
        for index in 0..chunks {
            let entry = footer + 32 + index * 76;
            let offset = read_u64(entry) as usize;
            let fragment = &bytes[offset..offset + read_u32(entry + 8)];
            html.push_str(std::str::from_utf8(fragment).expect("HTML fragment"));
        }
        html.push_str(header["documentSuffix"].as_str().expect("suffix"));
        html
    }

    #[test]
    fn a_relationship_row_is_counted_and_rendered_beside_the_provider_fingerprint() {
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope(temp.path(), "run-1");
        let output = render_bytes(&scope, &seal(&valid_rows())).expect("report");
        assert_eq!(output["complete"], true);
        assert_eq!(output["objectCount"], 1);
        assert_eq!(output["propertyCount"], 1);
        let html = bundle_html(&scope, &output);
        // The `provenance` arm is the only writer of a relationship section, and
        // it escapes its payload like every other value.
        assert!(html.contains("Hosted by &lt;Level 1&gt;"), "{html}");
        assert!(
            html.contains("Provider fingerprint: revit-2026 &lt;build 3&gt;"),
            "{html}"
        );
        // Column order in the row is the column order the header promises.
        assert!(
            html.contains(
                "<td>Dimensions</td><td>Length</td><td>1200</td><td>mm</td><td>Signed source row 0</td>"
            ),
            "{html}"
        );
    }

    #[test]
    fn the_stream_header_type_and_schema_version_are_pinned() {
        let reason = "record stream header has the wrong version";
        refuses(
            &with_field(
                "header",
                "schemaVersion",
                json!("floless.complete-model-report/v2"),
            ),
            reason,
        );
        refuses(&with_field("header", "type", json!("head")), reason);
    }

    #[test]
    fn the_receipt_list_must_carry_one_shard_for_each_of_the_three_roles() {
        let mut two = valid_rows();
        two[0]["receipts"] = json!([
            {"role":"entities","digest":"a".repeat(64),"bytes":10,"items":1,"ordinal":0},
            {"role":"properties","digest":"b".repeat(64),"bytes":10,"items":1,"ordinal":0}]);
        refuses(&two, "receipt list is incomplete");

        // At length three, one role twice necessarily means another has none, so
        // a whole-stream fixture cannot tell "duplicated" from "absent". Call
        // `exact_role` directly, past the length guard, where the two separate:
        // four shards, all three roles present, one of them twice.
        let shard = |role: &str| json!({"role":role,"digest":"a".repeat(64),"bytes":10,"items":1,"ordinal":0});
        let four = [
            shard("entities"),
            shard("entities"),
            shard("properties"),
            shard("relationships"),
        ];
        let duplicated = exact_role(&four, "entities").expect_err("duplicated role");
        assert!(
            duplicated
                .to_string()
                .contains("receipt list does not contain exactly one shard for each role"),
            "{duplicated}"
        );
        // The roles present exactly once still resolve, so the refusal above is
        // about the duplicate and not about the list as a whole.
        assert!(exact_role(&four, "properties").is_ok());
        assert!(exact_role(&four, "relationships").is_ok());
        // And a role with no shard at all is refused by the same rule.
        assert!(exact_role(&four, "nothing").is_err());

        // The direct call above proves `exact_role` rejects a duplicate; it does
        // not prove `render_stream_inner` looks shards up that way. A malformed
        // three-shard stream covers the integration: swap the receipt loop for a
        // positional or first-match lookup and this is what catches it.
        let mut duplicated_stream = valid_rows();
        duplicated_stream[0]["receipts"][2]["role"] = json!("entities");
        refuses(
            &duplicated_stream,
            "receipt list does not contain exactly one shard for each role",
        );

        let mut absent = valid_rows();
        absent[0]["receipts"] = json!(null);
        refuses(&absent, "receipt list is missing");
    }

    #[test]
    fn a_receipt_shard_must_be_ordinal_zero_with_a_full_length_digest_and_a_size() {
        let mut late = valid_rows();
        late[0]["receipts"][0]["ordinal"] = json!(1);
        refuses(&late, "receipt shard metadata is invalid");

        let mut short = valid_rows();
        short[0]["receipts"][1]["digest"] = json!("b".repeat(63));
        refuses(&short, "receipt shard metadata is invalid");

        let mut sizeless = valid_rows();
        sizeless[0]["receipts"][2]
            .as_object_mut()
            .expect("shard")
            .remove("bytes");
        refuses(&sizeless, "missing or invalid bytes");
    }

    #[test]
    fn receipt_item_counts_must_equal_the_header_totals() {
        for shard in 0..3 {
            let mut rows = valid_rows();
            let items = rows[0]["receipts"][shard]["items"].as_u64().expect("items");
            rows[0]["receipts"][shard]["items"] = json!(items + 1);
            refuses(&rows, "signed receipt and header totals disagree");
        }

        let mut headless = valid_rows();
        headless[0]
            .as_object_mut()
            .expect("header")
            .remove("expected");
        refuses(&headless, "expected totals are missing");
    }

    #[test]
    fn only_an_approved_provider_and_a_non_empty_revision_render() {
        let reason = "model provider or approved revision is invalid";
        refuses(&with_field("header", "provider", json!("rhino")), reason);

        // `revisionId` is cross-checked against the provenance row too, so blank
        // it in both — otherwise the mismatch guard would answer first.
        let mut blank = valid_rows();
        blank[0]["revisionId"] = json!("");
        blank[1]["revisionId"] = json!("");
        refuses(&blank, reason);

        // Tekla is the other approved reader, and it must still render.
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope(temp.path(), "run-1");
        let mut tekla = valid_rows();
        tekla[0]["provider"] = json!("tekla");
        let output = render_bytes(&scope, &seal(&tekla)).expect("tekla report");
        assert_eq!(output["objectCount"], 1);
    }

    #[test]
    fn model_provenance_must_repeat_every_approved_revision_field() {
        for key in ["sourceSha256", "manifestRoot", "projectUuid", "revisionId"] {
            let mut rows = valid_rows();
            rows[1][key] = json!("tampered");
            refuses(
                &rows,
                "model provenance does not match the approved revision",
            );
        }
    }

    #[test]
    fn model_provenance_appears_exactly_once_and_an_object_may_not_precede_it() {
        let mut twice = valid_rows();
        twice.insert(2, twice[1].clone());
        refuses(&twice, "model provenance must appear once before objects");

        let mut trailing = valid_rows();
        trailing.push(trailing[1].clone());
        refuses(
            &trailing,
            "model provenance must appear once before objects",
        );

        let mut missing = valid_rows();
        missing.remove(1);
        refuses(&missing, "model provenance is missing");
    }

    #[test]
    fn object_ordinals_must_be_exact_and_objects_may_not_nest() {
        let reason = "object order is incomplete or duplicated";
        for key in ["entityOrdinal", "recordOrdinal", "shardOrdinal"] {
            refuses(&with_field("entity-start", key, json!(1)), reason);
        }

        // A second start before the first object ends. Its ordinals are the ones
        // that WOULD be expected at that point, so nesting is the only rule it
        // breaks — a clone of the first start would trip the ordinal checks in
        // the same compound guard and prove nothing about nesting.
        let mut nested = valid_rows();
        let mut second = nested[2].clone();
        second["entityOrdinal"] = json!(1);
        second["recordOrdinal"] = json!(1);
        second["id"] = json!("e2");
        second["entity"] = json!({"id":"e2","name":"Column"});
        nested.insert(3, second);
        refuses(&nested, reason);

        // The source row an object claims has to exist in the signed receipt.
        refuses(
            &with_field("entity-start", "sourceOrdinal", json!(1)),
            "source row ordinal is outside its signed receipt",
        );
    }

    #[test]
    fn an_object_payload_may_contradict_nothing_about_its_source_row() {
        let mut disagrees = valid_rows();
        disagrees[2]["entity"]["id"] = json!("e2");
        refuses(&disagrees, "object identity disagrees with its source row");

        // A payload that simply omits its id is not a contradiction.
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope(temp.path(), "run-1");
        let mut silent = valid_rows();
        silent[2]["entity"]
            .as_object_mut()
            .expect("entity")
            .remove("id");
        assert_eq!(
            render_bytes(&scope, &seal(&silent)).expect("report")["objectCount"],
            1
        );
    }

    #[test]
    fn property_owner_and_ordinals_must_be_exact() {
        let reason = "property owner or order is incomplete or duplicated";
        refuses(&with_field("property", "ownerId", json!("e2")), reason);
        for key in [
            "entityOrdinal",
            "propertyOrdinal",
            "recordOrdinal",
            "shardOrdinal",
        ] {
            refuses(&with_field("property", key, json!(1)), reason);
        }

        // A property outside any object is refused before its ordinals matter.
        let mut orphan = valid_rows();
        let property = orphan.remove(3);
        orphan.insert(2, property);
        refuses(&orphan, "property has no current object");
    }

    #[test]
    fn a_property_payload_may_contradict_neither_its_owner_nor_its_own_id() {
        let mut wrong_owner = valid_rows();
        wrong_owner[3]["property"]["entityId"] = json!("e2");
        refuses(
            &wrong_owner,
            "property identity disagrees with its source row",
        );

        let mut wrong_id = valid_rows();
        wrong_id[3]["property"]["id"] = json!("p2");
        refuses(&wrong_id, "property identity disagrees with its source row");
    }

    #[test]
    fn an_object_end_must_match_the_start_it_closes() {
        let reason = "object property count is incomplete";
        refuses(&with_field("entity-end", "propertyCount", json!(2)), reason);
        refuses(&with_field("entity-end", "id", json!("e2")), reason);
        refuses(&with_field("entity-end", "entityOrdinal", json!(1)), reason);

        let mut orphan = valid_rows();
        let end = orphan.remove(4);
        orphan.insert(2, end);
        refuses(&orphan, "object end has no start");
    }

    #[test]
    fn a_relationship_row_may_not_sit_inside_an_object_or_out_of_order() {
        let reason = "model provenance order is incomplete";
        let mut inside = valid_rows();
        let relationship = inside.remove(5);
        inside.insert(3, relationship);
        refuses(&inside, reason);

        refuses(&with_field("provenance", "recordOrdinal", json!(1)), reason);
        refuses(&with_field("provenance", "shardOrdinal", json!(1)), reason);
        refuses(
            &with_field("provenance", "sourceOrdinal", json!(1)),
            "source row ordinal is outside its signed receipt",
        );
    }

    #[test]
    fn an_unsupported_record_type_refuses_rather_than_being_skipped() {
        let mut rows = valid_rows();
        rows.insert(2, json!({"type":"entity-delta","id":"e1"}));
        refuses(&rows, "record type is unsupported");
    }

    #[test]
    fn the_terminal_receipt_must_match_the_bytes_and_the_counts_it_seals() {
        let refuses_terminal = |tamper: &dyn Fn(&mut Value)| {
            let temp = tempfile::tempdir().expect("temp");
            let scope = scope(temp.path(), "run-1");
            let bytes = seal_with(&valid_rows(), tamper);
            let error = render_bytes(&scope, &bytes).expect_err("tampered terminal");
            assert!(
                error
                    .to_string()
                    .contains("the complete report receipt does not match its records"),
                "{error}"
            );
        };
        refuses_terminal(&|t| t["priorBytes"] = json!(1));
        refuses_terminal(&|t| t["priorSha256"] = json!("0".repeat(64)));
        refuses_terminal(&|t| t["observed"]["entities"] = json!(2));
        refuses_terminal(&|t| t["observed"]["properties"] = json!(0));
        refuses_terminal(&|t| t["observed"]["relationships"] = json!(0));

        // An object left open when the receipt arrives is a truncated stream.
        // The relationship row goes with the object end, so its expected total
        // and receipt drop to zero too — otherwise the count and coverage
        // checks answer for the open-object check and it proves nothing.
        let mut unclosed = valid_rows();
        unclosed.remove(4);
        unclosed.remove(4);
        unclosed[0]["expected"]["relationships"] = json!(0);
        unclosed[0]["receipts"][2]["items"] = json!(0);
        refuses(
            &unclosed,
            "the complete report receipt does not match its records",
        );
    }

    #[test]
    fn the_terminal_digest_is_compared_case_insensitively() {
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope(temp.path(), "run-1");
        let bytes = seal_with(&valid_rows(), |terminal| {
            let upper = terminal["priorSha256"]
                .as_str()
                .expect("digest")
                .to_ascii_uppercase();
            terminal["priorSha256"] = json!(upper);
        });
        assert_eq!(
            render_bytes(&scope, &bytes).expect("uppercase is the same digest")["objectCount"],
            1
        );
    }

    #[test]
    fn nothing_may_follow_the_terminal_receipt() {
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope(temp.path(), "run-1");
        let mut bytes = seal(&valid_rows());
        bytes.extend_from_slice(b"{\"type\":\"entity-end\"}\n");
        let error = render_bytes(&scope, &bytes).expect_err("trailing record");
        assert!(
            error
                .to_string()
                .contains("unexpected data after the terminal receipt"),
            "{error}"
        );
    }

    #[test]
    fn an_empty_unparseable_or_unterminated_stream_names_its_own_fault() {
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope(temp.path(), "run-1");
        let expect_refusal = |bytes: &[u8], reason: &str| {
            let error = render_bytes(&scope, bytes).expect_err(reason);
            assert!(error.to_string().contains(reason), "{error}");
        };
        expect_refusal(b"", "record stream is empty");
        expect_refusal(b"{ not json }\n", "record is not valid JSON");

        // Every record present and correct, but no terminal receipt to seal them.
        let rows = valid_rows();
        let mut unsealed = Vec::new();
        for row in &rows {
            unsealed.extend_from_slice(row.to_string().as_bytes());
            unsealed.push(b'\n');
        }
        expect_refusal(&unsealed, "terminal receipt is missing");
    }

    #[test]
    fn a_record_must_end_in_a_newline_and_stay_inside_its_bound() {
        // End of stream on a record boundary is not an error.
        assert_eq!(record_line(&mut &b""[..]).expect("empty"), None);

        // The newline is part of the record, because the terminal receipt
        // hashes the bytes including it.
        let mut reader = BufReader::new(&b"ab\ncd\n"[..]);
        assert_eq!(
            record_line(&mut reader).expect("first"),
            Some(b"ab\n".to_vec())
        );
        assert_eq!(
            record_line(&mut reader).expect("second"),
            Some(b"cd\n".to_vec())
        );
        assert_eq!(record_line(&mut reader).expect("end"), None);

        // Trailing bytes with no newline are a truncated record, not a final one.
        let truncated = record_line(&mut &b"{\"a\":1}"[..]).expect_err("truncated");
        assert!(
            truncated
                .to_string()
                .contains("record is missing its newline"),
            "{truncated}"
        );

        // A record that never ends is refused for being over its bound rather
        // than buffered to exhaustion and then blamed on the missing newline —
        // including when it arrives a few bytes at a time.
        let unbounded = vec![b'x'; MAX_RECORD + 1];
        let mut dribbled = BufReader::with_capacity(16, &unbounded[..]);
        let oversized = record_line(&mut dribbled).expect_err("unbounded record");
        assert!(
            oversized
                .to_string()
                .contains("a model record is too large to read safely"),
            "{oversized}"
        );
    }

    #[test]
    fn the_identity_table_is_a_power_of_two_of_slots_and_refuses_overflow() {
        // Two slots per item, rounded up to a power of two, 48 bytes a slot. The
        // power of two is what makes `slot & (slots - 1)` a correct wrap.
        for (items, bytes) in [(0u64, 48u64), (1, 96), (3, 384), (4, 384), (5, 768)] {
            assert_eq!(
                IdentitySet::table_bytes(items).expect("size"),
                bytes,
                "items={items}"
            );
        }
        assert!(IdentitySet::table_bytes(u64::MAX).is_err(), "items * 2");
        assert!(
            IdentitySet::table_bytes(u64::MAX / 2).is_err(),
            "next power of two"
        );
        assert!(IdentitySet::table_bytes(1 << 58).is_err(), "slots * 48");
    }

    #[test]
    fn an_object_with_no_readable_properties_says_so() {
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope(temp.path(), "run-1");
        let mut rows = valid_rows();
        rows.remove(3);
        rows[0]["expected"]["properties"] = json!(0);
        rows[0]["receipts"][1]["items"] = json!(0);
        rows[3]["propertyCount"] = json!(0);
        let output = render_bytes(&scope, &seal(&rows)).expect("report");
        assert_eq!(output["propertyCount"], 0);
        let html = bundle_html(&scope, &output);
        assert!(html.contains("No readable properties."), "{html}");
        assert!(!html.contains("<th>Property</th>"), "{html}");
    }

    #[test]
    fn a_model_with_no_objects_at_all_still_seals_a_complete_bundle() {
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope(temp.path(), "run-1");
        let mut rows = valid_rows();
        rows.truncate(2);
        rows[0]["expected"] = json!({"entities":0,"properties":0,"relationships":0});
        for shard in rows[0]["receipts"].as_array_mut().expect("receipts") {
            shard["items"] = json!(0);
        }
        // An empty role is covered the moment it is opened — `complete()` has to
        // read 0 of 0 as done, or a model with nothing to report never renders.
        let output = render_bytes(&scope, &seal(&rows)).expect("report");
        assert_eq!(output["objectCount"], 0);
        assert_eq!(output["propertyCount"], 0);
        assert_eq!(output["complete"], true);
        let html = bundle_html(&scope, &output);
        assert!(html.contains("0 objects · 0 properties"), "{html}");
        assert!(html.contains("Provider fingerprint:"), "{html}");
    }

    #[test]
    fn an_object_split_across_fragments_repeats_its_heading_as_continued() {
        let temp = tempfile::tempdir().expect("temp");
        // 32 rows is exactly one fragment's worth, so nothing is continued …
        let whole = scope(temp.path(), "run-1");
        let source = many_property_stream(32);
        let output = render_bytes(&whole, &source).expect("report");
        let html = bundle_html(&whole, &output);
        assert!(!html.contains("(continued)"), "{html}");

        // … and the 33rd row opens a fragment that has to say which object it
        // belongs to, or the reader sees a headless table.
        let split = scope(temp.path(), "run-2");
        let source = many_property_stream(33);
        let output = render_bytes(&split, &source).expect("report");
        let html = bundle_html(&split, &output);
        assert!(
            html.contains("<h2>Object 1 <small>(continued)</small></h2>"),
            "{html}"
        );
        assert_eq!(html.matches("Attribute 32<").count(), 1, "{html}");
    }

    #[test]
    fn a_property_row_accepts_each_documented_field_alias() {
        // Each alias list is tried in order, so a field is only exercised when
        // every earlier entry of its list is absent. `valid_rows()` covers the
        // first choice of each list (`groupName`, `name`, `value`, `unit`); the
        // two payloads here cover every remaining entry, last ones included.
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope(temp.path(), "run-1");
        let mut rows = valid_rows();
        rows[0]["expected"]["properties"] = json!(2);
        rows[0]["receipts"][1]["items"] = json!(2);
        rows[3]["property"] = json!({"entityId":"e1","group":"Identity",
            "parameterName":"Mark","displayValue":"B-01","units":"text"});
        let mut second = rows[3].clone();
        second["id"] = json!("p2");
        second["propertyOrdinal"] = json!(1);
        second["recordOrdinal"] = json!(1);
        second["sourceOrdinal"] = json!(1);
        // `category` and `id` end their lists, so nothing else may be present.
        second["property"] = json!({"entityId":"e1","category":"Constraints","id":"p2"});
        rows.insert(4, second);
        rows[5]["propertyCount"] = json!(2);
        let output = render_bytes(&scope, &seal(&rows)).expect("report");
        let html = bundle_html(&scope, &output);
        assert!(
            html.contains("<td>Identity</td><td>Mark</td><td>B-01</td><td>text</td>"),
            "{html}"
        );
        assert!(
            html.contains("<td>Constraints</td><td>p2</td><td></td><td></td>"),
            "{html}"
        );
    }

    #[test]
    fn an_object_heading_falls_back_to_its_escaped_id() {
        let temp = tempfile::tempdir().expect("temp");
        let labelled = scope(temp.path(), "run-1");
        let output = render_bytes(&labelled, &seal(&valid_rows())).expect("report");
        assert!(
            bundle_html(&labelled, &output).contains("<h2>Beam <small>(e1)</small></h2>"),
            "a named object keeps its id alongside the label"
        );

        let bare = scope(temp.path(), "run-2");
        let mut rows = valid_rows();
        rows[2]["id"] = json!("<e&1>");
        rows[2]["entity"] = json!({});
        rows[3]["ownerId"] = json!("<e&1>");
        rows[3]["property"]["entityId"] = json!("<e&1>");
        rows[4]["id"] = json!("<e&1>");
        let output = render_bytes(&bare, &seal(&rows)).expect("report");
        let html = bundle_html(&bare, &output);
        assert!(html.contains("<h2>&lt;e&amp;1&gt;</h2>"), "{html}");
        assert!(!html.contains("<e&1>"), "{html}");
    }

    #[test]
    fn every_html_special_character_is_escaped_and_non_strings_stringify_first() {
        assert_eq!(
            escape("<a href=\"x\">&'</a>"),
            "&lt;a href=&quot;x&quot;&gt;&amp;&#39;&lt;/a&gt;"
        );
        assert_eq!(display(None), "");
        assert_eq!(display(Some(&Value::Null)), "");
        // A string renders as itself, not as its JSON spelling with quotes.
        assert_eq!(display(Some(&json!("<b>"))), "&lt;b&gt;");
        assert_eq!(display(Some(&json!(12.5))), "12.5");
        assert_eq!(display(Some(&json!(false))), "false");
        assert_eq!(
            display(Some(&json!({"k":"<v>"}))),
            "{&quot;k&quot;:&quot;&lt;v&gt;&quot;}"
        );
    }

    #[test]
    fn the_report_column_set_is_fixed_and_a_preview_cannot_stand_in_for_a_run() {
        let narrowed = render_stream(
            &json!({"columns":["Group","Property","Value"]}),
            None,
            false,
        )
        .expect_err("the column set is fixed");
        assert!(
            narrowed
                .to_string()
                .contains("report columns must be Group, Property, Value, Unit, Provenance"),
            "{narrowed}"
        );

        // The documented set clears that guard and is then refused for its own
        // reason, which is what proves the guard is the one that fired above.
        let previewed = render_stream(
            &json!({"columns":["Group","Property","Value","Unit","Provenance"]}),
            None,
            true,
        )
        .expect_err("preview");
        assert!(
            previewed
                .to_string()
                .contains("complete reports require a real run"),
            "{previewed}"
        );
    }
}
