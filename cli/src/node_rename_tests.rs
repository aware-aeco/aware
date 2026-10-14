use super::*;

/// Target `src-node` is referenced by every reference kind; each kind has a
/// same-spelled negative control that must NOT change.
const ALL_KINDS: &str = "# header comment
app: t
version: 0.0.1
description: x
requires: []
engineering:
  output-seal:
    artifact: '{{ src-node.path }}'
    operator: '{{ inputs.op }}'
nodes:
  - id: src-node
    agent: x
    command: emit
  - id: cfg
    agent: x
    command: use
    config:
      v: '{{ src-node.a }}'
      alias: 'pre {{ src_node.b }} post'
      mixed: '{{ other.c + src-node.d }}'
      cond: '{% if src-node.ok %}yes{% endif %}'
      field: '{{ other.src-node }}'
      note: src-node stays plain text
      braces_only: '{ src-node }'
      quoted: '{{ \"src-node\" }}'
  - id: inp
    agent: x
    command: use
    inputs:
      whole: '{{ src-node }}'
  - id: each
    agent: x
    command: use
    for-each: '{{ src-node.rows }}'
    do:
      - id: inner
        agent: x
        command: use
        inputs:
          v: '{{ src-node.q }}{{ item.z }}'
  - id: cmp
    compare:
      a: '{{ src-node.rows }}'
      b: '{{ other.rows }}'
      a-snapshot: '{{ src-node.snap }}'
      by: id
  - id: gate
    assert:
      expr: '{{ src-node.ok }}'
  - id: sw
    sweep:
      var: v
      values: ['{{ src-node.lo }}', 5]
    do:
      - id: swinner
        agent: x
        command: use
  - id: safe
    agent: x
    command: write
    safety:
      rollback-token: '{{ src-node.token }}'
  - id: pinned
    agent: x
    command: use
    frozen:
      x: '{{ src-node.keep }}'
  - id: lit
    inline: { kind: predicate, description: 'about src-node', code: '{{ src-node.keep }}' }
  - id: other
    agent: x
    command: emit
connections:
  - from: src-node
    to: cfg
  - { from: other, to: src-node }
  - from: other
    to: cmp
";

fn renamed(src: &str, path: &str, to: &str) -> (Plan, String) {
    let p = plan(src, path, to).unwrap_or_else(|e| panic!("{}: {}", e.code, e.message));
    let text = p
        .text_source
        .clone()
        .unwrap_or_else(|| panic!("text edit failed: {:?}", p.text_edit_note));
    (p, text)
}

#[test]
fn rewrites_every_reference_kind_and_only_those() {
    let (p, out) = renamed(ALL_KINDS, "src-node", "feed");
    for (needle, why) in [
        ("- id: feed\n", "the node id"),
        ("artifact: '{{ feed.path }}'", "engineering output-seal"),
        ("v: '{{ feed.a }}'", "config template"),
        ("'pre {{ feed.b }} post'", "underscore alias spelling"),
        ("{{ other.c + feed.d }}", "head later in an expression"),
        ("{% if feed.ok %}", "statement block"),
        ("whole: '{{ feed }}'", "bare whole-node ref in inputs"),
        ("for-each: '{{ feed.rows }}'", "for-each source"),
        ("v: '{{ feed.q }}{{ item.z }}'", "nested body ref"),
        ("a: '{{ feed.rows }}'", "compare a"),
        ("a-snapshot: '{{ feed.snap }}'", "compare a-snapshot"),
        ("expr: '{{ feed.ok }}'", "assert expr"),
        ("values: ['{{ feed.lo }}', 5]", "sweep values"),
        (
            "rollback-token: '{{ feed.token }}'",
            "safety rollback-token",
        ),
        ("- from: feed\n", "connection from"),
        ("{ from: other, to: feed }", "connection to (flow style)"),
    ] {
        assert!(out.contains(needle), "{why}: missing {needle:?} in\n{out}");
    }
    // Negative controls: same spelling, different meaning, or not a reference.
    for (needle, why) in [
        ("# header comment", "yaml comment"),
        (
            "{{ other.src-node }}",
            "a FIELD that happens to share the name",
        ),
        (
            "note: src-node stays plain text",
            "plain text outside braces",
        ),
        ("'{ src-node }'", "single braces are not a template"),
        (
            "{{ \"src-node\" }}",
            "a quoted literal is reported, not rewritten",
        ),
        ("x: '{{ src-node.keep }}'", "frozen pinned output"),
        ("code: '{{ src-node.keep }}'", "inline code"),
        ("description: 'about src-node'", "inline description prose"),
        ("b: '{{ other.rows }}'", "an unrelated reference"),
        ("operator: '{{ inputs.op }}'", "namespace head"),
        ("- from: other\n    to: cmp", "an unrelated connection"),
    ] {
        assert!(out.contains(needle), "{why}: {needle:?} changed in\n{out}");
    }
    assert!(
        p.unverified.iter().any(|u| u.contains("quoted string")),
        "{:?}",
        p.unverified
    );
    assert!(serde_yaml::from_str::<crate::manifest::app::App>(&out).is_ok());
}

#[test]
fn only_the_deliberate_controls_still_name_the_old_id() {
    // If any reference kind were missed, its line would still name the old id and this
    // count would be higher than the six deliberate control lines.
    let (_, out) = renamed(ALL_KINDS, "src-node", "feed");
    let left: Vec<&str> = out
        .lines()
        .filter(|l| l.contains("src-node") || l.contains("src_node"))
        .collect();
    assert_eq!(left.len(), 6, "unexpected leftovers: {left:#?}");
}

#[test]
fn a_shadowing_nested_node_keeps_its_own_references() {
    let src = "app: t
version: 0.0.1
description: x
requires: []
nodes:
  - id: think
    agent: x
    command: emit
  - id: outer
    for-each: '[1]'
    do:
      - id: think
        agent: x
        command: emit
      - id: user
        agent: x
        command: use
        inputs:
          v: '{{ think.x }}'
  - id: top-user
    agent: x
    command: use
    inputs:
      v: '{{ think.x }}'
";
    // Rename the TOP-LEVEL `think`: only the reference outside `outer` follows it.
    let p = plan(src, "think", "first").unwrap_or_else(|e| panic!("{}: {}", e.code, e.message));
    let out = p.text_source.clone().unwrap();
    assert!(out.contains("- id: first\n"));
    assert_eq!(out.matches("{{ think.x }}").count(), 1, "{out}");
    assert!(out.contains("v: '{{ first.x }}'"));
    assert!(!p.unverified.is_empty(), "shared bare id is reported");
}

#[test]
fn suggestions_are_unique_against_adversarial_existing_ids() {
    let src = "app: t
version: 0.0.1
description: x
requires: []
nodes:
  - id: outer
    for-each: '[1]'
    do:
      - id: think
        agent: x
        command: emit
  - id: outer.think
    agent: x
    command: emit
  - id: outer-think
    agent: x
    command: emit
  - id: outer-think-2
    agent: x
    command: emit
  - id: outer_think
    agent: x
    command: emit
";
    let app: App = serde_yaml::from_str(src).unwrap();
    let target = parse_path(&app, "outer.think").unwrap();
    let s = suggest_id(&app, &target);
    assert_eq!(s, "outer-think-3");
    assert!(hard_conflict(&app, &target, &s).is_none());
    assert!(soft_conflicts(&app, &target, &s).is_empty());
    let p = plan(src, "outer.think", &s).unwrap();
    let app2: App = serde_yaml::from_str(&p.text_source.unwrap()).unwrap();
    assert!(collisions(&app2).is_empty());
}

#[test]
fn duplicate_nested_siblings_are_addressable_by_position() {
    let src = "app: t
version: 0.0.1
description: x
requires: []
nodes:
  - id: p
    for-each: '[1]'
    do:
      - id: x
        agent: a
        command: c
      - id: x
        agent: a
        command: c
";
    let app: App = serde_yaml::from_str(src).unwrap();
    let c = collisions(&app);
    assert_eq!(c.len(), 1);
    let (d, _) = collision_details(&app, &c[0]);
    assert_eq!(d["nodes"][0]["path"], "p/x[0]");
    assert_eq!(d["nodes"][1]["path"], "p/x[1]");
    assert_eq!(
        d["repairs"][1]["default"], true,
        "the later duplicate is the default"
    );
    assert!(
        plan(src, "p/x", "y1").is_err(),
        "an ambiguous selector is refused"
    );
    let out = plan(src, "p/x[1]", "second").unwrap().text_source.unwrap();
    assert!(out.contains("- id: x\n") && out.contains("- id: second\n"));
    assert!(collisions(&serde_yaml::from_str::<App>(&out).unwrap()).is_empty());
}

#[test]
fn grammar_rejects_dots_reserved_words_and_leading_digits() {
    for bad in ["a.b", "item", "None", "1x", "", "a/b", "x y"] {
        assert!(check_new_id_grammar(bad).is_err(), "{bad}");
    }
    assert!(check_new_id_grammar("outer-think_2").is_ok());
}
