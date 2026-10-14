//! Structured repair for ambiguous node ids (aware#701, builds on #694/#698).
//!
//! `E_APP_AMBIGUOUS_NODE_ID` stays a refusal to WRITE a lock (the lock, trace and
//! per-node consent markers are keyed by the scoped id), but it must not be a dead
//! end: the validator attaches structured `details` (this module builds them) and
//! `aware app rename-node` is the deterministic source repair.
//!
//! Node addressing. A node is addressed structurally by a *node path*: its ancestors'
//! ids and its own id joined by `/` (`outer/think` is `think` nested in `outer`;
//! `outer.think` is a single top-level node whose id contains a dot). A segment
//! carries `[n]` (its 0-based position among its siblings) when siblings share an id.
//! `%`, `/`, `[` and `]` inside an id are percent-escaped.
//!
//! Soundness. The rename is planned twice and the two must agree: once on the YAML
//! *value* tree (authoritative semantics), once as byte-preserving *text* edits (so
//! comments and layout survive). The text result is only used when re-parsing it
//! yields exactly the value-level result; otherwise `--reformat` is offered. Anything
//! the planner cannot prove (shared bare ids, quoted-literal mentions, a changed
//! scheduling edge set, ...) is reported as `unverified` and needs an explicit
//! `--accept-unverified` - an ask with a default, never a silent guess.

use std::collections::{BTreeSet, HashMap};

use serde_json::{Value as Json, json};
use serde_yaml::Value as Yaml;

use crate::manifest::app::{App, Node};

/// Names a template can mean something else by (namespaces, iteration bindings,
/// Jinja words and literals). A node may not be renamed TO one of these.
const RESERVED: &[&str] = &[
    "inputs",
    "config",
    "secrets",
    "run",
    "upstream",
    "item",
    "var",
    "now",
    "ctx",
    "env",
    "loop",
    "self",
    "true",
    "false",
    "none",
    "null",
    "and",
    "or",
    "not",
    "in",
    "is",
    "if",
    "else",
    "elif",
    "endif",
    "for",
    "endfor",
    "set",
    "raw",
    "endraw",
    "block",
    "macro",
    "import",
    "from",
    "with",
    "as",
    "include",
    "extends",
    "range",
    "dict",
    "namespace",
    "yes",
    "no",
    "on",
    "off",
];

fn alias(s: &str) -> String {
    s.replace('-', "_")
}

/// A new node id must be a plain, template-addressable name: it starts with a
/// letter, uses letters/digits/`-`/`_` only (no dot - dots are what make ids
/// ambiguous), and is not a reserved word.
pub fn check_new_id_grammar(id: &str) -> Result<(), String> {
    let mut chars = id.chars();
    let ok_first = chars.next().is_some_and(|c| c.is_ascii_alphabetic());
    if !ok_first
        || id.len() > 64
        || !id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(format!(
            "{id:?} is not a valid node id: use letters, digits, `-` and `_` (no dots or slashes), starting with a letter, at most 64 characters"
        ));
    }
    if RESERVED.contains(&id.to_ascii_lowercase().as_str()) {
        return Err(format!(
            "{id:?} is a reserved word a template would read differently; choose another name"
        ));
    }
    Ok(())
}

pub struct Entry<'a> {
    pub addr: Vec<usize>,
    pub node: &'a Node,
    /// Ids from the top-level ancestor down to this node.
    pub chain: Vec<&'a str>,
    pub scoped: String,
    pub iter_vars: Vec<&'static str>,
}

/// Depth-first, same order as `app_lock::flatten_nodes`.
pub fn walk(app: &App) -> Vec<Entry<'_>> {
    fn go<'a>(
        nodes: &'a [Node],
        addr: &[usize],
        chain: &[&'a str],
        iter_vars: &[&'static str],
        out: &mut Vec<Entry<'a>>,
    ) {
        for (i, n) in nodes.iter().enumerate() {
            let mut a = addr.to_vec();
            a.push(i);
            let mut c = chain.to_vec();
            c.push(n.id.as_str());
            out.push(Entry {
                addr: a.clone(),
                node: n,
                chain: c.clone(),
                scoped: c.join("."),
                iter_vars: iter_vars.to_vec(),
            });
            if let Some(body) = &n.do_ {
                let mut vars = iter_vars.to_vec();
                if n.for_each.is_some() {
                    vars.push("item");
                } else if n.sweep.is_some() {
                    vars.push("var");
                }
                go(body, &a, &c, &vars, out);
            }
        }
    }
    let mut out = Vec::new();
    go(&app.nodes, &[], &[], &[], &mut out);
    out
}

fn list_at<'a>(app: &'a App, prefix: &[usize]) -> &'a [Node] {
    let mut list: &[Node] = &app.nodes;
    for &i in prefix {
        match list.get(i).and_then(|n| n.do_.as_deref()) {
            Some(body) => list = body,
            None => return &[],
        }
    }
    list
}

fn encode(id: &str) -> String {
    id.replace('%', "%25")
        .replace('/', "%2F")
        .replace('[', "%5B")
        .replace(']', "%5D")
}

fn decode(seg: &str) -> String {
    seg.replace("%5B", "[")
        .replace("%5D", "]")
        .replace("%2F", "/")
        .replace("%25", "%")
}

/// The node path of the node at `addr`.
pub fn path_of(app: &App, addr: &[usize]) -> String {
    let mut segs = Vec::new();
    for depth in 0..addr.len() {
        let list = list_at(app, &addr[..depth]);
        let me = &list[addr[depth]];
        let dup = list.iter().filter(|n| n.id == me.id).count() > 1;
        let mut s = encode(&me.id);
        if dup {
            s.push_str(&format!("[{}]", addr[depth]));
        }
        segs.push(s);
    }
    segs.join("/")
}

/// Resolve a node path to a node address.
pub fn parse_path(app: &App, path: &str) -> Result<Vec<usize>, String> {
    let mut addr: Vec<usize> = Vec::new();
    for raw in path.split('/') {
        let list = list_at(app, &addr);
        let (id_part, idx) = match raw.rfind('[') {
            Some(p) if raw.ends_with(']') => {
                (&raw[..p], raw[p + 1..raw.len() - 1].parse::<usize>().ok())
            }
            _ => (raw, None),
        };
        let id = decode(id_part);
        let found: Vec<usize> = list
            .iter()
            .enumerate()
            .filter(|(_, n)| n.id == id)
            .map(|(i, _)| i)
            .collect();
        let pick = match (idx, found.as_slice()) {
            (Some(i), f) if f.contains(&i) => i,
            (None, [one]) => *one,
            (None, []) | (Some(_), _) if found.is_empty() => {
                let have: Vec<String> = (0..list.len())
                    .map(|i| {
                        let mut a = addr.clone();
                        a.push(i);
                        path_of(app, &a)
                    })
                    .collect();
                return Err(format!(
                    "no node {raw:?} at this level; the nodes here are: {}",
                    have.join(", ")
                ));
            }
            _ => {
                return Err(format!(
                    "several sibling nodes are named {id:?}; add their position, e.g. `{id}[{}]`",
                    found[0]
                ));
            }
        };
        addr.push(pick);
    }
    if addr.is_empty() {
        return Err("empty node path".into());
    }
    Ok(addr)
}

pub struct Collision {
    pub scoped: String,
    pub nodes: Vec<Vec<usize>>,
}

/// Groups of nodes sharing one scoped id, except plain top-level duplicates
/// (those are `E_APP_DUPLICATE_NODE_ID`).
pub fn collisions(app: &App) -> Vec<Collision> {
    let mut order: Vec<String> = Vec::new();
    let mut groups: HashMap<String, Vec<Vec<usize>>> = HashMap::new();
    for e in walk(app) {
        if !groups.contains_key(&e.scoped) {
            order.push(e.scoped.clone());
        }
        groups.entry(e.scoped.clone()).or_default().push(e.addr);
    }
    order
        .into_iter()
        .filter_map(|s| {
            let nodes = groups.remove(&s)?;
            (nodes.len() > 1 && !nodes.iter().all(|a| a.len() == 1))
                .then_some(Collision { scoped: s, nodes })
        })
        .collect()
}

fn prospective_scoped(
    app: &App,
    target: &[usize],
    new_id: &str,
) -> (Vec<String>, BTreeSet<String>) {
    let (mut inside, mut outside) = (Vec::new(), BTreeSet::new());
    for e in walk(app) {
        if e.addr.starts_with(target) {
            let mut c: Vec<&str> = e.chain.clone();
            c[target.len() - 1] = new_id;
            inside.push(c.join("."));
        } else {
            outside.insert(e.scoped);
        }
    }
    (inside, outside)
}

/// Why `new_id` cannot be used at all for the node at `target` (a real collision
/// or an invalid name), or `None`.
pub fn hard_conflict(app: &App, target: &[usize], new_id: &str) -> Option<String> {
    if let Err(e) = check_new_id_grammar(new_id) {
        return Some(e);
    }
    let siblings = list_at(app, &target[..target.len() - 1]);
    for (i, n) in siblings.iter().enumerate() {
        if i != *target.last().unwrap_or(&0) && (n.id == new_id || alias(&n.id) == alias(new_id)) {
            return Some(format!(
                "a sibling node is already called {:?} (ids that differ only by `-` vs `_` also clash)",
                n.id
            ));
        }
    }
    let (inside, outside) = prospective_scoped(app, target, new_id);
    if let Some(c) = inside.iter().find(|s| outside.contains(*s)) {
        return Some(format!(
            "the scoped id {c:?} would collide with another node"
        ));
    }
    None
}

/// Softer, "ask first" conditions on a new id: it is reused elsewhere (a template
/// head could mean either) or it is a connection input-slot name.
pub fn soft_conflicts(app: &App, target: &[usize], new_id: &str) -> Vec<String> {
    let mut out = Vec::new();
    for e in walk(app) {
        if e.addr != target && (e.node.id == new_id || alias(&e.node.id) == alias(new_id)) {
            out.push(format!(
                "another node at `{}` is also called {new_id:?}; a `{{{{ {new_id}.* }}}}` reference could then mean either",
                path_of(app, &e.addr)
            ));
        }
    }
    if app
        .connections
        .iter()
        .any(|c| c.input.as_deref() == Some(new_id))
    {
        out.push(format!(
            "{new_id:?} is also the name of a connection input slot, which templates resolve first"
        ));
    }
    out
}

fn is_fresh(app: &App, target: &[usize], cand: &str) -> bool {
    hard_conflict(app, target, cand).is_none() && soft_conflicts(app, target, cand).is_empty()
}

/// A suggested id for the node at `target`: its id with dots turned into dashes,
/// then `-2`, `-3`, ... until it is unique across the WHOLE app (so it can neither
/// collide with nor shadow anything). Bounded; falls back to a numbered name.
pub fn suggest_id(app: &App, target: &[usize]) -> String {
    let node = &list_at(app, &target[..target.len() - 1])[*target.last().unwrap_or(&0)];
    let mut base: String = node
        .id
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect();
    if !base.chars().next().is_some_and(|c| c.is_ascii_alphabetic()) {
        base = format!("node-{base}");
    }
    base.truncate(56);
    if base != node.id && is_fresh(app, target, &base) {
        return base;
    }
    for n in 2..=2000 {
        let cand = format!("{base}-{n}");
        if is_fresh(app, target, &cand) {
            return cand;
        }
    }
    format!("node-{}", uuid::Uuid::new_v4().simple())
}

fn shell_quote(s: &str) -> String {
    if s.chars()
        .all(|c| c.is_ascii_alphanumeric() || "-_./[]".contains(c))
    {
        s.to_string()
    } else {
        format!("'{}'", s.replace('\'', "'\\''"))
    }
}

/// Structured `details` for an `E_APP_AMBIGUOUS_NODE_ID` issue.
pub fn collision_details(app: &App, c: &Collision) -> (Json, String) {
    // Default repair: the shallowest node whose own id contains a dot (a dotted id is
    // what normally causes the clash); for plain duplicate siblings, the later one.
    let default_idx = c
        .nodes
        .iter()
        .enumerate()
        .filter(|(_, a)| {
            let list = list_at(app, &a[..a.len() - 1]);
            list[*a.last().unwrap_or(&0)].id.contains('.')
        })
        .min_by_key(|(_, a)| a.len())
        .map(|(i, _)| i)
        .unwrap_or(c.nodes.len() - 1);
    let mut nodes = Vec::new();
    let mut repairs = Vec::new();
    let mut default_cmd = String::new();
    for (i, a) in c.nodes.iter().enumerate() {
        let path = path_of(app, a);
        let list = list_at(app, &a[..a.len() - 1]);
        let id = list[*a.last().unwrap_or(&0)].id.clone();
        nodes.push(json!({
            "path": path,
            "id": id,
            "location": if a.len() == 1 { "top-level" } else { "nested" },
            "parent": if a.len() == 1 { Json::Null } else { json!(path_of(app, &a[..a.len() - 1])) },
        }));
        let new_id = suggest_id(app, a);
        let command = format!(
            "aware app rename-node <app-or-path> {} {}",
            shell_quote(&path),
            shell_quote(&new_id)
        );
        if i == default_idx {
            default_cmd = command.clone();
        }
        repairs.push(json!({
            "action": "rename-node",
            "node-path": path,
            "new-id": new_id,
            "default": i == default_idx,
            "command": command,
        }));
    }
    let details = json!({
        "scoped-id": c.scoped,
        "nodes": nodes,
        "repairs": repairs,
        "suggested-id": repairs[default_idx]["new-id"],
    });
    (details, default_cmd)
}

// ---------------------------------------------------------------------------
// Template rewriting
// ---------------------------------------------------------------------------

#[derive(Clone, Default, Debug)]
pub struct Pairs {
    /// `{{ head.* }}` spellings that resolve to the renamed node: (old, new).
    pub tokens: Vec<(String, String)>,
    /// Exact `upstream["<id>"]` literals (flat shared context, not lexically scoped).
    pub literals: Vec<(String, String)>,
}

impl Pairs {
    fn is_empty(&self) -> bool {
        self.tokens.is_empty() && self.literals.is_empty()
    }
}

fn is_id_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}

/// Rewrite one `{{ }}` / `{% %}` body.
fn rewrite_body(body: &str, pairs: &Pairs, mentions: &mut Vec<String>, whole: &str) -> String {
    let chars: Vec<char> = body.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '\'' || c == '"' {
            let mut j = i + 1;
            while j < chars.len() && chars[j] != c {
                if chars[j] == '\\' {
                    j += 1;
                }
                j += 1;
            }
            let end = j.min(chars.len());
            let content: String = chars[i + 1..end].iter().collect();
            let before: String = chars[..i].iter().collect();
            let after_upstream = before
                .trim_end()
                .strip_suffix("upstream[")
                .is_some_and(|pre| {
                    !pre.chars()
                        .next_back()
                        .is_some_and(|c| is_id_char(c) || c == '.')
                });
            let hit = pairs
                .literals
                .iter()
                .chain(pairs.tokens.iter())
                .find(|(o, _)| *o == content);
            match hit {
                Some((_, n)) if after_upstream => {
                    out.push(c);
                    out.push_str(n);
                    out.push(c);
                }
                Some(_) => {
                    mentions.push(format!(
                        "a quoted string equal to the old id inside a template: {whole}"
                    ));
                    out.extend(chars[i..(end + 1).min(chars.len())].iter());
                }
                None => out.extend(chars[i..(end + 1).min(chars.len())].iter()),
            }
            i = end + 1;
            continue;
        }
        if is_id_char(c) {
            let mut j = i;
            while j < chars.len() && is_id_char(chars[j]) {
                j += 1;
            }
            let run: String = chars[i..j].iter().collect();
            let prev = if i > 0 { Some(chars[i - 1]) } else { None };
            let prefix: String = chars[..i].iter().collect();
            let head_pos = match prev {
                Some('.') => {
                    let p = prefix.trim_end_matches('.');
                    p.ends_with("upstream")
                        && !p
                            .chars()
                            .rev()
                            .nth("upstream".len())
                            .is_some_and(|c| is_id_char(c) || c == '.')
                }
                // `| length` is a filter name, not a node reference.
                _ => !prefix.trim_end().ends_with('|'),
            };
            // `name=value` inside a call is a keyword argument, not a reference.
            let rest_after: String = chars[j..].iter().collect();
            let after_trim = rest_after.trim_start();
            let head_pos =
                head_pos && !(after_trim.starts_with('=') && !after_trim.starts_with("=="));
            match pairs.tokens.iter().find(|(o, _)| *o == run) {
                Some((_, n)) if head_pos => out.push_str(n),
                _ => out.push_str(&run),
            }
            i = j;
            continue;
        }
        out.push(c);
        i += 1;
    }
    out
}

/// Index of the first `close` delimiter that is outside a quoted string.
fn find_close(after: &str, close: &str) -> Option<usize> {
    let mut quote: Option<char> = None;
    let mut skip = false;
    for (i, c) in after.char_indices() {
        if skip {
            skip = false;
            continue;
        }
        match quote {
            Some(q) => {
                if c == '\\' {
                    skip = true;
                } else if c == q {
                    quote = None;
                }
            }
            None => {
                if after[i..].starts_with(close) {
                    return Some(i);
                }
                if c == '"' || c == '\'' {
                    quote = Some(c);
                }
            }
        }
    }
    None
}

/// Rewrite every template body in `s`; text outside `{{ }}` / `{% %}` is untouched.
pub fn rewrite_template(s: &str, pairs: &Pairs, mentions: &mut Vec<String>) -> String {
    if pairs.is_empty() || !s.contains('{') {
        return s.to_string();
    }
    let mut out = String::new();
    let mut rest = s;
    loop {
        let next = ["{{", "{%", "{#"]
            .iter()
            .filter_map(|d| rest.find(d).map(|p| (p, *d)))
            .min_by_key(|(p, _)| *p);
        let Some((p, open)) = next else {
            out.push_str(rest);
            break;
        };
        out.push_str(&rest[..p]);
        let close = match open {
            "{{" => "}}",
            "{%" => "%}",
            _ => "#}",
        };
        let after = &rest[p + 2..];
        let found = if open == "{#" {
            after.find(close)
        } else {
            find_close(after, close)
        };
        let Some(e) = found else {
            out.push_str(&rest[p..]);
            break;
        };
        let body = &after[..e];
        let is_raw = open == "{%"
            && body
                .trim_start_matches(['-', '+'])
                .trim_start()
                .starts_with("raw");
        let verbatim = open == "{#" || is_raw;
        if verbatim {
            // A raw block is literal through its matching `endraw`.
            let mut seg_end = p + 2 + e + 2;
            if is_raw {
                let tail = &rest[seg_end..];
                seg_end = match tail.find("endraw") {
                    Some(q) => match tail[q..].find("%}") {
                        Some(c) => seg_end + q + c + 2,
                        None => rest.len(),
                    },
                    None => rest.len(),
                };
            }
            // Comments and raw blocks are literal text: never rewritten, but a mention of the
            // old id in one is reported so a person can decide.
            let hit = pairs
                .tokens
                .iter()
                .chain(pairs.literals.iter())
                .any(|(o, _)| rest[p..seg_end].contains(o.as_str()));
            if hit {
                mentions.push(format!(
                    "the old id appears in a template comment/raw block: {}",
                    &rest[p..seg_end]
                ));
            }
            out.push_str(&rest[p..seg_end]);
            rest = &rest[seg_end..];
            continue;
        } else {
            out.push_str(open);
            out.push_str(&rewrite_body(
                body,
                pairs,
                mentions,
                &rest[p..p + 2 + e + 2],
            ));
            out.push_str(close);
        }
        rest = &after[e + 2..];
    }
    out
}

// ---------------------------------------------------------------------------
// Resolution
// ---------------------------------------------------------------------------

enum Resolved {
    Target,
    Other,
    Ambiguous,
}

/// Which node does head `h`, written inside the node at `addr`, mean? Mirrors
/// `app_lock::resolve_scope`: the innermost enclosing body first, outward to top level.
fn resolve_head(
    app: &App,
    addr: &[usize],
    iter_vars: &[&str],
    h: &str,
    target: &[usize],
) -> Resolved {
    if iter_vars.contains(&h) {
        return Resolved::Other;
    }
    for depth in (0..addr.len()).rev() {
        let list = list_at(app, &addr[..depth]);
        let m: Vec<usize> = list
            .iter()
            .enumerate()
            .filter(|(_, n)| n.id == h || alias(&n.id) == h)
            .map(|(i, _)| i)
            .collect();
        match m.as_slice() {
            [] => continue,
            [one] => {
                let mut a = addr[..depth].to_vec();
                a.push(*one);
                return if a == target {
                    Resolved::Target
                } else {
                    Resolved::Other
                };
            }
            _ => return Resolved::Ambiguous,
        }
    }
    Resolved::Other
}

// ---------------------------------------------------------------------------
// Value-level planning
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Edit {
    pub kind: &'static str,
    /// 1-based source line (text edits) or 0 (value-level edits).
    pub line: usize,
    pub location: String,
    pub before: String,
    pub after: String,
}

impl Edit {
    pub fn to_json(&self) -> Json {
        json!({"kind": self.kind, "line": if self.line == 0 { Json::Null } else { json!(self.line) },
               "location": self.location, "before": self.before, "after": self.after})
    }
}

#[derive(Debug)]
pub struct RenameError {
    pub code: &'static str,
    pub message: String,
    pub details: Json,
}

impl RenameError {
    fn new(code: &'static str, message: impl Into<String>, details: Json) -> Self {
        Self {
            code,
            message: message.into(),
            details,
        }
    }
}

pub struct Plan {
    pub old_id: String,
    pub new_id: String,
    pub node_path: String,
    /// New source bytes from the byte-preserving edit, when verified.
    pub text_source: Option<String>,
    /// Re-serialised source (comments and layout are lost), the `--reformat` route.
    pub reformatted: String,
    pub edits: Vec<Edit>,
    pub value_edits: Vec<Edit>,
    pub unverified: Vec<String>,
    pub text_edit_note: Option<String>,
}

fn rewrite_strings(
    v: &mut Yaml,
    pairs: &Pairs,
    loc: &str,
    mentions: &mut Vec<String>,
    edits: &mut Vec<Edit>,
) {
    match v {
        Yaml::String(s) => {
            let n = rewrite_template(s, pairs, mentions);
            if n != *s {
                edits.push(Edit {
                    kind: "template",
                    line: 0,
                    location: loc.to_string(),
                    before: s.clone(),
                    after: n.clone(),
                });
                *s = n;
            }
        }
        Yaml::Mapping(m) => {
            for (k, val) in m.iter_mut() {
                let name = k.as_str().unwrap_or("?").to_string();
                rewrite_strings(val, pairs, &format!("{loc}.{name}"), mentions, edits);
            }
        }
        Yaml::Sequence(seq) => {
            for (i, val) in seq.iter_mut().enumerate() {
                rewrite_strings(val, pairs, &format!("{loc}[{i}]"), mentions, edits);
            }
        }
        _ => {}
    }
}

struct Ctx<'a> {
    pairs: HashMap<Vec<usize>, Pairs>,
    target: &'a [usize],
    new_id: &'a str,
    mentions: Vec<String>,
    edits: Vec<Edit>,
    paths: HashMap<Vec<usize>, String>,
}

fn rewrite_nodes_value(seq: &mut Yaml, prefix: &[usize], cx: &mut Ctx) {
    let Some(items) = seq.as_sequence_mut() else {
        return;
    };
    for (i, item) in items.iter_mut().enumerate() {
        let mut addr = prefix.to_vec();
        addr.push(i);
        let pairs = cx.pairs.get(&addr).cloned().unwrap_or_default();
        let here = cx.paths.get(&addr).cloned().unwrap_or_default();
        let Some(map) = item.as_mapping_mut() else {
            continue;
        };
        for (k, val) in map.iter_mut() {
            let key = k.as_str().unwrap_or("").to_string();
            match key.as_str() {
                "id" => {
                    if addr == cx.target {
                        let before = val.as_str().unwrap_or("").to_string();
                        *val = Yaml::String(cx.new_id.to_string());
                        cx.edits.push(Edit {
                            kind: "node-id",
                            line: 0,
                            location: here.clone(),
                            before,
                            after: cx.new_id.to_string(),
                        });
                    }
                }
                "frozen" | "row" | "col" | "mode" => {}
                "do" => rewrite_nodes_value(val, &addr, cx),
                "inline" => {
                    if let Some(inputs) = val.as_mapping_mut().and_then(|m| m.get_mut("inputs")) {
                        let (mut mentions, mut edits) = (Vec::new(), Vec::new());
                        rewrite_strings(
                            inputs,
                            &pairs,
                            &format!("{here}.inline.inputs"),
                            &mut mentions,
                            &mut edits,
                        );
                        cx.mentions.extend(mentions);
                        cx.edits.extend(edits);
                    }
                }
                _ => {
                    let (mut mentions, mut edits) = (Vec::new(), Vec::new());
                    rewrite_strings(
                        val,
                        &pairs,
                        &format!("{here}.{key}"),
                        &mut mentions,
                        &mut edits,
                    );
                    cx.mentions.extend(mentions);
                    cx.edits.extend(edits);
                }
            }
        }
    }
}

fn edge_set(app: &App) -> BTreeSet<(String, String)> {
    let mut s: BTreeSet<(String, String)> = app
        .connections
        .iter()
        .map(|c| (c.from.clone(), c.to.clone()))
        .collect();
    for c in crate::app_lock::derive_connections(app) {
        s.insert((c.from, c.to));
    }
    s
}

fn error_codes(app: &App) -> Vec<&'static str> {
    crate::validate::validate_app(app)
        .into_iter()
        .filter(|i| i.severity == crate::validate::Severity::Error)
        .map(|i| i.code)
        .collect()
}

/// Plan renaming the node at `node_path` to `new_id` in `source`.
pub fn plan(source: &str, node_path: &str, new_id: &str) -> Result<Plan, RenameError> {
    let app: App = serde_yaml::from_str(source).map_err(|e| {
        RenameError::new(
            "E_APP_RENAME_NODE_PARSE",
            format!("the source does not parse: {e}"),
            Json::Null,
        )
    })?;
    let root: Yaml = serde_yaml::from_str(source).map_err(|e| {
        RenameError::new(
            "E_APP_RENAME_NODE_PARSE",
            format!("the source does not parse: {e}"),
            Json::Null,
        )
    })?;
    let target = parse_path(&app, node_path)
        .map_err(|m| RenameError::new("E_APP_RENAME_NODE_NOT_FOUND", m, Json::Null))?;
    let entries = walk(&app);
    let t_entry = entries.iter().find(|e| e.addr == target).ok_or_else(|| {
        RenameError::new("E_APP_RENAME_NODE_NOT_FOUND", "node not found", Json::Null)
    })?;
    let old_id = t_entry.node.id.clone();
    let node_path = path_of(&app, &target);

    let suggestion = || suggest_id(&app, &target);
    if new_id == old_id {
        return Err(RenameError::new(
            "E_APP_RENAME_NODE_SAME_ID",
            format!(
                "the node is already called {old_id:?}; give it a different id (suggested: {})",
                suggestion()
            ),
            json!({"suggested-id": suggestion()}),
        ));
    }
    if let Some(why) = hard_conflict(&app, &target, new_id) {
        let s = suggestion();
        return Err(RenameError::new(
            "E_APP_RENAME_NODE_ID_CONFLICT",
            format!("cannot rename `{node_path}` to {new_id:?}: {why}. Suggested free id: {s}"),
            json!({"reason": why, "suggested-id": s}),
        ));
    }

    let mut unverified: Vec<String> = soft_conflicts(&app, &target, new_id);
    let reserved_old = RESERVED.contains(&old_id.to_ascii_lowercase().as_str());
    let shared: Vec<String> = entries
        .iter()
        .filter(|e| {
            e.addr != target && (e.node.id == old_id || alias(&e.node.id) == alias(&old_id))
        })
        .map(|e| path_of(&app, &e.addr))
        .collect();
    if !shared.is_empty() {
        unverified.push(format!(
            "other nodes are also called {old_id:?} ({}); templates resolve by scope in the compiler but share one name at run time",
            shared.join(", ")
        ));
    }
    if app
        .connections
        .iter()
        .any(|c| c.input.as_deref() == Some(old_id.as_str()))
    {
        unverified.push(format!(
            "{old_id:?} is also a connection input-slot name; it was left as is"
        ));
    }

    // Which spellings, written inside which node, mean the target?
    let h_old: Vec<(String, String)> = {
        let mut v = vec![(old_id.clone(), new_id.to_string())];
        if alias(&old_id) != old_id {
            v.push((alias(&old_id), alias(new_id)));
        }
        v
    };
    let mut pairs: HashMap<Vec<usize>, Pairs> = HashMap::new();
    let mut paths: HashMap<Vec<usize>, String> = HashMap::new();
    let literals = h_old.clone();
    for e in &entries {
        paths.insert(e.addr.clone(), path_of(&app, &e.addr));
        let mut p = Pairs {
            tokens: Vec::new(),
            literals: literals.clone(),
        };
        if !reserved_old && !old_id.contains('.') {
            for (o, n) in &h_old {
                match resolve_head(&app, &e.addr, &e.iter_vars, o, &target) {
                    Resolved::Target => p.tokens.push((o.clone(), n.clone())),
                    Resolved::Ambiguous => {}
                    Resolved::Other => {}
                }
            }
        }
        pairs.insert(e.addr.clone(), p);
    }
    // Ambiguous resolution is only reported where a template actually uses the head.
    if !reserved_old && !old_id.contains('.') {
        for e in &entries {
            for (o, n) in &h_old {
                if matches!(
                    resolve_head(&app, &e.addr, &e.iter_vars, o, &target),
                    Resolved::Ambiguous
                ) {
                    let trial = Pairs {
                        tokens: vec![(o.clone(), n.clone())],
                        literals: vec![],
                    };
                    let mut probe = serde_yaml::to_value(&e.node.config).unwrap_or(Yaml::Null);
                    let (mut m, mut ed) = (Vec::new(), Vec::new());
                    rewrite_strings(&mut probe, &trial, "", &mut m, &mut ed);
                    let mut probe2 = serde_yaml::to_value(&e.node.inputs).unwrap_or(Yaml::Null);
                    rewrite_strings(&mut probe2, &trial, "", &mut m, &mut ed);
                    if !ed.is_empty() {
                        unverified.push(format!(
                            "`{}` refers to {o:?}, which matches several sibling nodes; it was left as is",
                            path_of(&app, &e.addr)
                        ));
                    }
                }
            }
        }
    }
    if reserved_old {
        unverified.push(format!(
            "{old_id:?} is a name templates treat specially, so no template reference was rewritten"
        ));
    }

    // Value-level result (authoritative).
    let mut expected = root.clone();
    let mut cx = Ctx {
        pairs: pairs.clone(),
        target: &target,
        new_id,
        mentions: Vec::new(),
        edits: Vec::new(),
        paths,
    };
    if let Some(nodes) = expected.as_mapping_mut().and_then(|m| m.get_mut("nodes")) {
        rewrite_nodes_value(nodes, &[], &mut cx);
    }
    // Connections name top-level ids exactly.
    if target.len() == 1
        && let Some(conns) = expected
            .as_mapping_mut()
            .and_then(|m| m.get_mut("connections"))
            .and_then(|c| c.as_sequence_mut())
    {
        for (i, c) in conns.iter_mut().enumerate() {
            if let Some(m) = c.as_mapping_mut() {
                for key in ["from", "to"] {
                    if let Some(v) = m.get_mut(key)
                        && v.as_str() == Some(old_id.as_str())
                    {
                        *v = Yaml::String(new_id.to_string());
                        cx.edits.push(Edit {
                            kind: if key == "from" {
                                "connection-from"
                            } else {
                                "connection-to"
                            },
                            line: 0,
                            location: format!("connections[{i}].{key}"),
                            before: old_id.clone(),
                            after: new_id.to_string(),
                        });
                    }
                }
            }
        }
    }
    // App-level strings that are templates at top-level scope.
    let top_pairs = if target.len() == 1 && !reserved_old && !old_id.contains('.') {
        let mut p = Pairs {
            tokens: Vec::new(),
            literals: literals.clone(),
        };
        for (o, n) in &h_old {
            if matches!(resolve_head(&app, &[0], &[], o, &target), Resolved::Target) {
                p.tokens.push((o.clone(), n.clone()));
            }
        }
        p
    } else {
        Pairs {
            tokens: Vec::new(),
            literals: literals.clone(),
        }
    };
    if let Some(eng) = expected
        .as_mapping_mut()
        .and_then(|m| m.get_mut("engineering"))
    {
        let (mut m, mut ed) = (Vec::new(), Vec::new());
        rewrite_strings(eng, &top_pairs, "engineering", &mut m, &mut ed);
        cx.mentions.extend(m);
        cx.edits.extend(ed);
    }
    for m in cx.mentions.drain(..) {
        if !unverified.contains(&m) {
            unverified.push(m);
        }
    }
    let value_edits = std::mem::take(&mut cx.edits);

    // The renamed app must be at least as valid, with the same scheduling edges.
    let planned_text = serde_yaml::to_string(&expected).map_err(|e| {
        RenameError::new(
            "E_APP_RENAME_NODE_INTERNAL",
            format!("cannot serialise: {e}"),
            Json::Null,
        )
    })?;
    let new_app: App = serde_yaml::from_str(&planned_text).map_err(|e| {
        RenameError::new(
            "E_APP_RENAME_NODE_INTERNAL",
            format!("the renamed source no longer parses: {e}"),
            Json::Null,
        )
    })?;
    let before_codes = error_codes(&app);
    let after_codes = error_codes(&new_app);
    if let Some(extra) = after_codes.iter().find(|c| !before_codes.contains(c)) {
        return Err(RenameError::new(
            "E_APP_RENAME_NODE_INTRODUCES_ERROR",
            format!("renaming would introduce {extra}; nothing was changed"),
            Json::Null,
        ));
    }
    let back = |s: &str| {
        if s == new_id {
            old_id.clone()
        } else {
            s.to_string()
        }
    };
    let after_edges: BTreeSet<(String, String)> = edge_set(&new_app)
        .into_iter()
        .map(|(f, t)| (back(&f), back(&t)))
        .collect();
    let before_edges = edge_set(&app);
    if after_edges != before_edges {
        let fmt = |s: &BTreeSet<(String, String)>| {
            s.iter()
                .map(|(f, t)| format!("{f}->{t}"))
                .collect::<Vec<_>>()
                .join(", ")
        };
        unverified.push(format!(
            "the scheduling edges between nodes change (before: [{}], after: [{}])",
            fmt(&before_edges),
            fmt(&after_edges)
        ));
    }

    let reformatted = serde_yaml::to_string(&expected).map_err(|e| {
        RenameError::new(
            "E_APP_RENAME_NODE_INTERNAL",
            format!("cannot serialise: {e}"),
            Json::Null,
        )
    })?;

    // Byte-preserving text edit, accepted only if it re-parses to exactly `expected`.
    let (text_source, edits, note) =
        match text_edit(source, &app, &target, &old_id, new_id, &pairs, &top_pairs) {
            Ok((new_src, edits)) => match serde_yaml::from_str::<Yaml>(&new_src) {
                Ok(v) if v == expected => (Some(new_src), edits, None),
                Ok(_) => (
                    None,
                    Vec::new(),
                    Some(
                        "the text edit did not reproduce the planned result (unusual layout)"
                            .to_string(),
                    ),
                ),
                Err(e) => (
                    None,
                    Vec::new(),
                    Some(format!("the text edit produced invalid YAML: {e}")),
                ),
            },
            Err(why) => (None, Vec::new(), Some(why)),
        };

    Ok(Plan {
        old_id,
        new_id: new_id.to_string(),
        node_path,
        text_source,
        reformatted,
        edits,
        value_edits,
        unverified,
        text_edit_note: note,
    })
}

// ---------------------------------------------------------------------------
// Text-level edit
// ---------------------------------------------------------------------------

fn indent(l: &str) -> usize {
    l.len() - l.trim_start().len()
}

/// The part of `line` before a trailing ` # comment` (quote-aware).
fn code_part(line: &str) -> &str {
    let b = line.as_bytes();
    let (mut sq, mut dq) = (false, false);
    for i in 0..b.len() {
        match b[i] {
            b'\'' if !dq => sq = !sq,
            b'"' if !sq => dq = !dq,
            b'#' if !sq && !dq && (i == 0 || b[i - 1] == b' ' || b[i - 1] == b'\t') => {
                return &line[..i];
            }
            _ => {}
        }
    }
    line
}

fn is_blank_or_comment(l: &str) -> bool {
    let t = l.trim();
    t.is_empty() || t.starts_with('#')
}

/// `true` for every line that is the body of a `|` / `>` block scalar.
fn block_scalar_mask(lines: &[&str]) -> Vec<bool> {
    let mut mask = vec![false; lines.len()];
    let mut key_indent: Option<usize> = None;
    for (i, l) in lines.iter().enumerate() {
        if let Some(k) = key_indent {
            if l.trim().is_empty() || indent(l) > k {
                mask[i] = true;
                continue;
            }
            key_indent = None;
        }
        let code = code_part(l).trim_end();
        if let Some(tok) = code.rsplit(char::is_whitespace).next()
            && tok.starts_with(['|', '>'])
            && tok
                .chars()
                .skip(1)
                .all(|c| c == '-' || c == '+' || c.is_ascii_digit())
        {
            let head = code[..code.len() - tok.len()].trim_end();
            if head.ends_with(':') || head.ends_with('-') {
                let t = l.trim_start();
                key_indent = Some(indent(l) + if t.starts_with("- ") { 2 } else { 0 });
            }
        }
    }
    mask
}

struct Region {
    start: usize,
    id_line: Option<usize>,
    own: Vec<(usize, usize)>,
    children: Vec<Region>,
    field_indent: usize,
}

fn key_of(trimmed: &str) -> &str {
    let t = trimmed.strip_prefix("- ").unwrap_or(trimmed);
    t.split(':').next().unwrap_or("").trim()
}

fn parse_list(
    lines: &[&str],
    mask: &[bool],
    from: usize,
    to: usize,
    min_indent: usize,
) -> Vec<Region> {
    let mut i = from;
    while i < to && (mask[i] || is_blank_or_comment(lines[i])) {
        i += 1;
    }
    if i >= to {
        return Vec::new();
    }
    let k = indent(lines[i]);
    let t = lines[i].trim_start();
    if k < min_indent || !(t.starts_with("- ") || t == "-") {
        return Vec::new();
    }
    let mut out = Vec::new();
    while i < to {
        let s = i;
        let mut j = i + 1;
        while j < to {
            let l = lines[j];
            if mask[j] || is_blank_or_comment(l) {
                j += 1;
                continue;
            }
            let ind = indent(l);
            if ind > k {
                j += 1;
                continue;
            }
            break;
        }
        out.push(build_region(lines, mask, s, j, k));
        i = j;
        if i >= to || indent(lines[i]) != k || !lines[i].trim_start().starts_with("- ") {
            break;
        }
    }
    out
}

fn build_region(lines: &[&str], mask: &[bool], s: usize, e: usize, k: usize) -> Region {
    let f = k + 2;
    let mut id_line = None;
    let mut do_line = None;
    for i in s..e {
        if mask[i] || is_blank_or_comment(lines[i]) {
            continue;
        }
        let at_field = if i == s { true } else { indent(lines[i]) == f };
        if !at_field {
            continue;
        }
        let key = key_of(lines[i].trim_start());
        if key == "id" && id_line.is_none() {
            id_line = Some(i);
        }
        if key == "do" && do_line.is_none() {
            let rest = code_part(lines[i]).trim_end();
            if rest.ends_with(':') {
                do_line = Some(i);
            }
        }
    }
    let (own, children) = match do_line {
        Some(d) => {
            let children = parse_list(lines, mask, d + 1, e, f);
            let child_end = children_end(&children, lines, mask, d + 1, e, f);
            (vec![(s, d + 1), (child_end, e)], children)
        }
        None => (vec![(s, e)], Vec::new()),
    };
    Region {
        start: s,
        id_line,
        own,
        children,
        field_indent: f,
    }
}

/// End (exclusive) of the child list that follows a `do:` line.
fn children_end(
    children: &[Region],
    lines: &[&str],
    mask: &[bool],
    from: usize,
    to: usize,
    f: usize,
) -> usize {
    if children.is_empty() {
        return from;
    }
    let mut i = from;
    while i < to && (mask[i] || is_blank_or_comment(lines[i])) {
        i += 1;
    }
    let k = indent(lines[i]);
    let mut j = i + 1;
    while j < to {
        let l = lines[j];
        if mask[j] || is_blank_or_comment(l) {
            j += 1;
            continue;
        }
        let ind = indent(l);
        if ind > k || (ind == k && l.trim_start().starts_with("- ")) {
            j += 1;
            continue;
        }
        break;
    }
    let _ = f;
    j
}

fn shape_matches(regions: &[Region], nodes: &[Node], lines: &[&str]) -> bool {
    if regions.len() != nodes.len() {
        return false;
    }
    regions.iter().zip(nodes).all(|(r, n)| {
        let id_ok = r.id_line.is_some_and(|l| {
            let t = lines[l].trim_start();
            let t = t.strip_prefix("- ").unwrap_or(t);
            t.strip_prefix("id:")
                .is_some_and(|v| scalar_text(code_part(v).trim()) == n.id)
        });
        id_ok && shape_matches(&r.children, n.do_.as_deref().unwrap_or(&[]), lines)
    })
}

fn scalar_text(v: &str) -> String {
    let v = v.trim();
    for q in ['"', '\''] {
        if v.len() >= 2
            && let Some(inner) = v.strip_prefix(q).and_then(|r| r.strip_suffix(q))
        {
            return inner.to_string();
        }
    }
    v.to_string()
}

/// Replace the scalar value of `key:` in `line` when it equals `old`.
fn replace_key_scalar(line: &str, keys: &[&str], old: &str, new: &str) -> Option<String> {
    let code = code_part(line);
    for key in keys {
        let pat = format!("{key}:");
        let mut from = 0;
        while let Some(p) = code[from..].find(&pat) {
            let at = from + p;
            let before_ok =
                at == 0 || matches!(code.as_bytes()[at - 1], b' ' | b'\t' | b'{' | b',' | b'-');
            if before_ok {
                let after = at + pat.len();
                let rest = &code[after..];
                let lead = rest.len() - rest.trim_start().len();
                let vstart = after + lead;
                let vrest = &code[vstart..];
                let (val_len, quoted) = match vrest.chars().next() {
                    Some(q @ ('"' | '\'')) => match vrest.strip_prefix(q).and_then(|r| r.find(q)) {
                        Some(e) => (e + 2, true),
                        None => (0, false),
                    },
                    _ => (vrest.find([',', '}']).unwrap_or(vrest.len()), false),
                };
                if val_len > 0 {
                    let raw = &vrest[..val_len];
                    let q = if quoted {
                        &raw[..raw.chars().next().map_or(0, char::len_utf8)]
                    } else {
                        ""
                    };
                    let text = if quoted {
                        raw.strip_prefix(q)
                            .and_then(|r| r.strip_suffix(q))
                            .unwrap_or(raw)
                    } else {
                        raw.trim_end()
                    };
                    if text == old {
                        let end = vstart
                            + if quoted {
                                val_len
                            } else {
                                raw.trim_end().len()
                            };
                        return Some(format!("{}{q}{new}{q}{}", &line[..vstart], &line[end..]));
                    }
                }
            }
            from = at + pat.len();
        }
    }
    None
}

/// Rewrite template references on one source line (code part only unless in a block
/// scalar body).
fn rewrite_line(line: &str, in_block: bool, pairs: &Pairs) -> String {
    if pairs.is_empty() {
        return line.to_string();
    }
    let (code, tail) = if in_block {
        (line, "")
    } else {
        let c = code_part(line);
        (c, &line[c.len()..])
    };
    let mut ignore = Vec::new();
    format!("{}{tail}", rewrite_template(code, pairs, &mut ignore))
}

#[allow(clippy::too_many_arguments)]
fn text_edit(
    source: &str,
    app: &App,
    target: &[usize],
    old_id: &str,
    new_id: &str,
    pairs: &HashMap<Vec<usize>, Pairs>,
    top_pairs: &Pairs,
) -> Result<(String, Vec<Edit>), String> {
    let crlf = source.contains("\r\n");
    let trailing_nl = source.ends_with('\n');
    let lines: Vec<&str> = source.lines().collect();
    let mask = block_scalar_mask(&lines);
    let nodes_line = lines
        .iter()
        .enumerate()
        .position(|(i, l)| !mask[i] && code_part(l).trim_end() == "nodes:")
        .ok_or("the `nodes:` list is not in block style")?;
    let regions = parse_list(&lines, &mask, nodes_line + 1, lines.len(), 0);
    if !shape_matches(&regions, &app.nodes, &lines) {
        return Err("the node list uses a layout the text editor cannot follow (flow style or unusual indentation)".into());
    }
    let mut new_lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
    let mut kinds: HashMap<usize, &'static str> = HashMap::new();

    fn visit(
        regions: &[Region],
        prefix: &[usize],
        lines: &[&str],
        mask: &[bool],
        pairs: &HashMap<Vec<usize>, Pairs>,
        target: &[usize],
        old_id: &str,
        new_id: &str,
        new_lines: &mut [String],
        kinds: &mut HashMap<usize, &'static str>,
    ) {
        for (i, r) in regions.iter().enumerate() {
            let mut addr = prefix.to_vec();
            addr.push(i);
            if addr == target
                && let Some(l) = r.id_line
                && let Some(n) = replace_key_scalar(&new_lines[l], &["id"], old_id, new_id)
            {
                new_lines[l] = n;
                kinds.insert(l, "node-id");
            }
            let p = pairs.get(&addr).cloned().unwrap_or_default();
            // `frozen:` and `inline:` (except its `inputs:`) are literal data.
            let mut skip_all_until: Option<usize> = None;
            let mut inline_mode = false;
            let mut inline_sub: Option<usize> = None;
            let mut in_inputs = false;
            for &(a, b) in &r.own {
                for l in a..b {
                    if Some(l) == r.id_line || (mask[l] && skip_all_until.is_some()) {
                        continue;
                    }
                    let line = lines[l];
                    if !mask[l] && is_blank_or_comment(line) {
                        continue;
                    }
                    let ind = indent(line);
                    let is_field = if l == r.start {
                        true
                    } else {
                        ind == r.field_indent
                    };
                    if !mask[l] {
                        if let Some(_u) = skip_all_until
                            && ind > r.field_indent
                        {
                            continue;
                        }
                        skip_all_until = None;
                        if inline_mode && ind <= r.field_indent && l != r.start {
                            inline_mode = false;
                            inline_sub = None;
                            in_inputs = false;
                        }
                        if is_field {
                            let key = key_of(line.trim_start());
                            if key == "frozen" {
                                skip_all_until = Some(l);
                                continue;
                            }
                            if key == "inline" {
                                if code_part(line).trim_end().ends_with(':') {
                                    inline_mode = true;
                                } else {
                                    // flow-style inline: literal as a whole
                                    continue;
                                }
                                continue;
                            }
                        }
                        if inline_mode {
                            let g = *inline_sub.get_or_insert(ind);
                            if ind == g {
                                in_inputs = key_of(line.trim_start()) == "inputs";
                            }
                            if !in_inputs {
                                continue;
                            }
                        }
                    } else if inline_mode && !in_inputs {
                        continue;
                    }
                    let nl = rewrite_line(&new_lines[l], mask[l], &p);
                    if nl != new_lines[l] {
                        new_lines[l] = nl;
                        kinds.entry(l).or_insert("template");
                    }
                }
            }
            visit(
                &r.children,
                &addr,
                lines,
                mask,
                pairs,
                target,
                old_id,
                new_id,
                new_lines,
                kinds,
            );
        }
    }
    visit(
        &regions,
        &[],
        &lines,
        &mask,
        pairs,
        target,
        old_id,
        new_id,
        &mut new_lines,
        &mut kinds,
    );

    // Connections (top-level targets only).
    let top_level_section = |name: &str| -> Option<(usize, usize)> {
        let start = lines
            .iter()
            .enumerate()
            .position(|(i, l)| !mask[i] && code_part(l).trim_end() == name)?;
        let mut end = lines.len();
        for (i, l) in lines.iter().enumerate().skip(start + 1) {
            if !mask[i]
                && !is_blank_or_comment(l)
                && indent(l) == 0
                && !l.trim_start().starts_with("- ")
            {
                end = i;
                break;
            }
        }
        Some((start + 1, end))
    };
    if target.len() == 1
        && let Some((a, b)) = top_level_section("connections:")
    {
        for l in a..b {
            if mask[l] || is_blank_or_comment(lines[l]) {
                continue;
            }
            if let Some(n) = replace_key_scalar(&new_lines[l], &["from", "to"], old_id, new_id) {
                let mut cur = n;
                // `from` and `to` may share a flow-style line.
                while let Some(m) = replace_key_scalar(&cur, &["from", "to"], old_id, new_id) {
                    if m == cur {
                        break;
                    }
                    cur = m;
                }
                new_lines[l] = cur;
                kinds.insert(l, "connection");
            }
        }
    }
    if let Some((a, b)) = top_level_section("engineering:") {
        for l in a..b {
            if is_blank_or_comment(lines[l]) && !mask[l] {
                continue;
            }
            let nl = rewrite_line(&new_lines[l], mask[l], top_pairs);
            if nl != new_lines[l] {
                new_lines[l] = nl;
                kinds.entry(l).or_insert("template");
            }
        }
    }

    let mut edits = Vec::new();
    for (i, nl) in new_lines.iter().enumerate() {
        if nl != lines[i] {
            edits.push(Edit {
                kind: kinds.get(&i).copied().unwrap_or("template"),
                line: i + 1,
                location: String::new(),
                before: lines[i].to_string(),
                after: nl.clone(),
            });
        }
    }
    let sep = if crlf { "\r\n" } else { "\n" };
    let mut out = new_lines.join(sep);
    if trailing_nl {
        out.push_str(sep);
    }
    Ok((out, edits))
}
#[cfg(test)]
#[path = "node_rename_tests.rs"]
mod tests;
