//! Minimal predicate evaluator for inline glue nodes.
//!
//! Grammar (Pratt-style precedence: `||` < `&&` < equality):
//!   expr := or_expr
//!   or_expr := and_expr ("||" and_expr)*
//!   and_expr := eq_expr ("&&" eq_expr)*
//!   eq_expr := atom (("==" | "!=") atom)?
//!   atom := "(" expr ")" | path | literal
//!   path := ident ("." ident)*
//!   literal := string | number | null | true | false
//!
//! `e => <expr>` is supported by stripping the prefix before parsing.

use serde_json::Value;

use crate::error::AwareError;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Ident(String),
    Str(String),
    Num(f64),
    Null,
    True,
    False,
    EqEq,
    NotEq,
    AndAnd,
    OrOr,
    LParen,
    RParen,
    Dot,
}

fn tokenize(src: &str) -> Result<Vec<Token>, AwareError> {
    let mut out = Vec::new();
    let mut chars = src.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            '(' => {
                chars.next();
                out.push(Token::LParen);
            }
            ')' => {
                chars.next();
                out.push(Token::RParen);
            }
            '.' => {
                chars.next();
                out.push(Token::Dot);
            }
            '=' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    out.push(Token::EqEq);
                } else {
                    return Err(AwareError::Validation(
                        "predicate: bare '=' not allowed".into(),
                    ));
                }
            }
            '!' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    out.push(Token::NotEq);
                } else {
                    return Err(AwareError::Validation(
                        "predicate: bare '!' not allowed".into(),
                    ));
                }
            }
            '&' => {
                chars.next();
                if chars.peek() == Some(&'&') {
                    chars.next();
                    out.push(Token::AndAnd);
                } else {
                    return Err(AwareError::Validation(
                        "predicate: bare '&' not allowed".into(),
                    ));
                }
            }
            '|' => {
                chars.next();
                if chars.peek() == Some(&'|') {
                    chars.next();
                    out.push(Token::OrOr);
                } else {
                    return Err(AwareError::Validation(
                        "predicate: bare '|' not allowed".into(),
                    ));
                }
            }
            '"' => {
                chars.next();
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        chars.next();
                        break;
                    }
                    s.push(c);
                    chars.next();
                }
                out.push(Token::Str(s));
            }
            c if c.is_ascii_digit() || c == '-' => {
                let mut s = String::new();
                if c == '-' {
                    s.push(c);
                    chars.next();
                }
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        s.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let n: f64 = s.parse().map_err(|e| {
                    AwareError::Validation(format!("predicate: bad number {s:?}: {e}"))
                })?;
                out.push(Token::Num(n));
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        s.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match s.as_str() {
                    "null" => out.push(Token::Null),
                    "true" => out.push(Token::True),
                    "false" => out.push(Token::False),
                    _ => out.push(Token::Ident(s)),
                }
            }
            c => {
                return Err(AwareError::Validation(format!(
                    "predicate: unexpected char {c:?}"
                )));
            }
        }
    }
    Ok(out)
}

#[derive(Debug, Clone)]
enum Node {
    Path(Vec<String>),
    Str(String),
    Num(f64),
    Null,
    Bool(bool),
    Eq(Box<Node>, Box<Node>),
    NotEq(Box<Node>, Box<Node>),
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
}

fn parse_or(tokens: &[Token]) -> Result<(Node, &[Token]), AwareError> {
    let (mut lhs, mut rest) = parse_and(tokens)?;
    while rest.first() == Some(&Token::OrOr) {
        let (rhs, r) = parse_and(&rest[1..])?;
        lhs = Node::Or(Box::new(lhs), Box::new(rhs));
        rest = r;
    }
    Ok((lhs, rest))
}

fn parse_and(tokens: &[Token]) -> Result<(Node, &[Token]), AwareError> {
    let (mut lhs, mut rest) = parse_eq(tokens)?;
    while rest.first() == Some(&Token::AndAnd) {
        let (rhs, r) = parse_eq(&rest[1..])?;
        lhs = Node::And(Box::new(lhs), Box::new(rhs));
        rest = r;
    }
    Ok((lhs, rest))
}

fn parse_eq(tokens: &[Token]) -> Result<(Node, &[Token]), AwareError> {
    let (lhs, rest) = parse_atom(tokens)?;
    match rest.first() {
        Some(Token::EqEq) => {
            let (rhs, r) = parse_atom(&rest[1..])?;
            Ok((Node::Eq(Box::new(lhs), Box::new(rhs)), r))
        }
        Some(Token::NotEq) => {
            let (rhs, r) = parse_atom(&rest[1..])?;
            Ok((Node::NotEq(Box::new(lhs), Box::new(rhs)), r))
        }
        _ => Ok((lhs, rest)),
    }
}

fn parse_atom(tokens: &[Token]) -> Result<(Node, &[Token]), AwareError> {
    match tokens.first() {
        Some(Token::LParen) => {
            let (node, rest) = parse_or(&tokens[1..])?;
            match rest.first() {
                Some(Token::RParen) => Ok((node, &rest[1..])),
                _ => Err(AwareError::Validation("predicate: missing ')'".into())),
            }
        }
        Some(Token::Str(s)) => Ok((Node::Str(s.clone()), &tokens[1..])),
        Some(Token::Num(n)) => Ok((Node::Num(*n), &tokens[1..])),
        Some(Token::Null) => Ok((Node::Null, &tokens[1..])),
        Some(Token::True) => Ok((Node::Bool(true), &tokens[1..])),
        Some(Token::False) => Ok((Node::Bool(false), &tokens[1..])),
        Some(Token::Ident(first)) => {
            let mut path = vec![first.clone()];
            let mut rest = &tokens[1..];
            while rest.first() == Some(&Token::Dot) {
                match rest.get(1) {
                    Some(Token::Ident(part)) => {
                        path.push(part.clone());
                        rest = &rest[2..];
                    }
                    _ => {
                        return Err(AwareError::Validation(
                            "predicate: '.' must be followed by identifier".into(),
                        ));
                    }
                }
            }
            Ok((Node::Path(path), rest))
        }
        Some(t) => Err(AwareError::Validation(format!(
            "predicate: unexpected token {t:?}"
        ))),
        None => Err(AwareError::Validation(
            "predicate: unexpected end of input".into(),
        )),
    }
}

fn resolve_path(path: &[String], event: &Value) -> Value {
    // The convention is `e.foo.bar` where `e` is the event itself.
    // First identifier is consumed as the binding name; subsequent identifiers are field hops.
    let mut current = event;
    let start = if path.first().map(|s| s == "e").unwrap_or(false) {
        1
    } else {
        0
    };
    for hop in &path[start..] {
        match current.get(hop.as_str()) {
            Some(v) => current = v,
            None => return Value::Null,
        }
    }
    current.clone()
}

fn eval_node(n: &Node, event: &Value) -> Result<Value, AwareError> {
    match n {
        Node::Path(p) => Ok(resolve_path(p, event)),
        Node::Str(s) => Ok(Value::String(s.clone())),
        Node::Num(n) => Ok(serde_json::json!(*n)),
        Node::Null => Ok(Value::Null),
        Node::Bool(b) => Ok(Value::Bool(*b)),
        Node::Eq(a, b) => {
            let va = eval_node(a, event)?;
            let vb = eval_node(b, event)?;
            Ok(Value::Bool(values_equal(&va, &vb)))
        }
        Node::NotEq(a, b) => {
            let va = eval_node(a, event)?;
            let vb = eval_node(b, event)?;
            Ok(Value::Bool(!values_equal(&va, &vb)))
        }
        Node::And(a, b) => {
            let va = eval_node(a, event)?;
            if !value_truthy(&va) {
                return Ok(Value::Bool(false));
            }
            let vb = eval_node(b, event)?;
            Ok(Value::Bool(value_truthy(&vb)))
        }
        Node::Or(a, b) => {
            let va = eval_node(a, event)?;
            if value_truthy(&va) {
                return Ok(Value::Bool(true));
            }
            let vb = eval_node(b, event)?;
            Ok(Value::Bool(value_truthy(&vb)))
        }
    }
}

fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x.as_f64() == y.as_f64(),
        _ => a == b,
    }
}

fn value_truthy(v: &Value) -> bool {
    match v {
        Value::Bool(b) => *b,
        Value::Null => false,
        _ => true,
    }
}

pub fn eval_predicate(code: &str, event: &Value) -> Result<bool, AwareError> {
    let expr_src = code
        .trim()
        .split_once("=>")
        .map(|(_, r)| r.trim())
        .unwrap_or(code.trim());
    let tokens = tokenize(expr_src)?;
    let (node, rest) = parse_or(&tokens)?;
    if !rest.is_empty() {
        return Err(AwareError::Validation(format!(
            "predicate: trailing tokens: {rest:?}"
        )));
    }
    let evaluated = eval_node(&node, event)?;
    match evaluated {
        Value::Bool(b) => Ok(b),
        other => Err(AwareError::Validation(format!(
            "predicate did not evaluate to bool: {other}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn simple_eq() {
        assert!(eval_predicate(r#"e.type == "Welded""#, &json!({"type":"Welded"})).unwrap());
        assert!(!eval_predicate(r#"e.type == "Welded""#, &json!({"type":"Bolted"})).unwrap());
    }

    #[test]
    fn null_check() {
        assert!(eval_predicate("e.mark != null", &json!({"mark":"A-104"})).unwrap());
        assert!(!eval_predicate("e.mark != null", &json!({"mark":null})).unwrap());
        assert!(!eval_predicate("e.mark != null", &json!({})).unwrap()); // missing = null
    }

    #[test]
    fn welded_to_tc_predicate() {
        let code = r#"e => e.type == "Welded" && e.mark != null"#;
        assert!(eval_predicate(code, &json!({"type":"Welded","mark":"A-104"})).unwrap());
        assert!(!eval_predicate(code, &json!({"type":"Welded","mark":null})).unwrap());
        assert!(!eval_predicate(code, &json!({"type":"Bolted","mark":"X"})).unwrap());
    }

    #[test]
    fn nested_path() {
        let code = r#"e.assembly.type == "Welded""#;
        assert!(eval_predicate(code, &json!({"assembly":{"type":"Welded"}})).unwrap());
    }

    #[test]
    fn numeric_eq() {
        assert!(eval_predicate("e.count == 5", &json!({"count":5})).unwrap());
        assert!(!eval_predicate("e.count == 5", &json!({"count":6})).unwrap());
    }

    #[test]
    fn bare_equals_is_error() {
        let err = eval_predicate("e.x = 1", &json!({"x":1}));
        assert!(err.is_err());
    }

    #[test]
    fn non_bool_result_is_error() {
        // Just a field path, no comparison
        let err = eval_predicate("e.x", &json!({"x":42}));
        assert!(err.is_err());
    }

    // ── precedence and grouping ─────────────────────────────────────────────
    //
    // The module header promises `||` < `&&` < equality, but nothing checked it.
    // Every prior test used one operator, or used `||` and `&&` in a shape that
    // parses the same under either precedence — so the ladder could have been
    // rebuilt in the wrong order and the suite would have stayed green.

    #[test]
    fn and_binds_tighter_than_or() {
        // Each event is chosen so that the two precedences disagree — an event
        // where they agree proves nothing here.
        //
        // `false && false` on the right of `||`: correct = true (the left arm
        // carries it); with the ladder swapped, `(a==1 || b==1) && c==1` = false.
        assert!(
            eval_predicate(
                "e.a == 1 || e.b == 1 && e.c == 1",
                &json!({"a":1,"b":9,"c":9})
            )
            .unwrap()
        );
        // The mirror image, so a swap is caught from either side. Correct =
        // `(false && false) || true` = true; swapped = `false && (…)` = false.
        assert!(
            eval_predicate(
                "e.a == 1 && e.b == 1 || e.c == 1",
                &json!({"a":9,"b":9,"c":1})
            )
            .unwrap()
        );
    }

    #[test]
    fn parentheses_regroup_what_precedence_would_otherwise_decide() {
        // The test this replaces wrapped each side of an `||` in parens that the
        // precedence ladder would have produced anyway, so it passed whether or
        // not `(` opened a fresh expression. Grouping is only observable when it
        // contradicts precedence: same event, same terms, opposite answers.
        let event = json!({"a":1,"b":9,"c":9});
        assert!(eval_predicate("e.a == 1 || e.b == 1 && e.c == 1", &event).unwrap());
        assert!(!eval_predicate("(e.a == 1 || e.b == 1) && e.c == 1", &event).unwrap());
        // And a group nests — the inner one must survive being reached through
        // the outer one rather than being flattened into it.
        assert!(!eval_predicate("((e.a == 1) || (e.b == 1)) && e.c == 1", &event).unwrap());
    }

    #[test]
    fn every_term_in_a_chain_is_evaluated() {
        // `parse_or` / `parse_and` loop; a single-shot `if` would parse the first
        // pair, leave the remainder unconsumed and fail with "trailing tokens" —
        // so a three-term chain is what distinguishes a loop from a branch.
        assert!(
            eval_predicate(
                "e.a == 1 && e.b == 2 && e.c == 3",
                &json!({"a":1,"b":2,"c":3})
            )
            .unwrap()
        );
        // …and the third term genuinely participates rather than being dropped.
        assert!(
            !eval_predicate(
                "e.a == 1 && e.b == 2 && e.c == 3",
                &json!({"a":1,"b":2,"c":9})
            )
            .unwrap()
        );
        assert!(
            eval_predicate(
                "e.a == 9 || e.b == 9 || e.c == 3",
                &json!({"a":1,"b":2,"c":3})
            )
            .unwrap()
        );
        assert!(
            !eval_predicate(
                "e.a == 9 || e.b == 9 || e.c == 9",
                &json!({"a":1,"b":2,"c":3})
            )
            .unwrap()
        );
    }

    // ── truthiness ──────────────────────────────────────────────────────────

    #[test]
    fn only_null_and_false_are_falsy() {
        // `value_truthy` is deliberately NOT JavaScript's rule, even though the
        // predicates are written in a JS-looking syntax: `0` and `""` are TRUE
        // here. That divergence is exactly the kind a maintainer would "fix"
        // toward the familiar semantics, silently flipping every gate in every
        // app that tests a numeric field. Nothing pinned it before.
        assert!(eval_predicate("e.a && e.b", &json!({"a":0,"b":""})).unwrap());
        assert!(eval_predicate("e.a || e.b", &json!({"a":0,"b":false})).unwrap());
        // The two that ARE falsy, plus the missing field that resolves to null.
        assert!(!eval_predicate("e.a || e.b", &json!({"a":false,"b":null})).unwrap());
        assert!(!eval_predicate("e.a || e.b", &json!({"a":false})).unwrap());
        assert!(!eval_predicate("e.a && e.b", &json!({"a":true,"b":null})).unwrap());
    }

    // ── path resolution ─────────────────────────────────────────────────────

    #[test]
    fn the_leading_e_is_a_binding_name_not_a_field() {
        // `e.type` means the event's `type`, not the `type` of a field called
        // `e` — and a path that does not start with `e` reads straight off the
        // event. One event carries both readings, so dropping the binding rule
        // or applying it unconditionally each produces the wrong answer.
        let event = json!({"type":"Welded", "e":{"type":"Bolted"}});
        assert!(eval_predicate(r#"e.type == "Welded""#, &event).unwrap());
        assert!(eval_predicate(r#"type == "Welded""#, &event).unwrap());
        // Reaching the field that is literally named `e` takes saying so twice.
        assert!(eval_predicate(r#"e.e.type == "Bolted""#, &event).unwrap());
    }

    #[test]
    fn a_hop_through_a_scalar_yields_null() {
        // A path that runs off the end of the data is null, not an error — that
        // is what lets `!= null` be the idiom for "present". `null_check` covers
        // the one-hop case; this covers hopping THROUGH a value that has no
        // fields at all, which is what a mis-typed path actually looks like.
        assert!(eval_predicate("e.mark.length == null", &json!({"mark":"A-104"})).unwrap());
        assert!(eval_predicate("e.a.b.c == null", &json!({"a":{"b":1}})).unwrap());
        assert!(!eval_predicate("e.a.b == null", &json!({"a":{"b":1}})).unwrap());
    }

    // ── what the parser refuses ─────────────────────────────────────────────
    //
    // A predicate gates whether work runs. Every input below must fail loudly:
    // a parser that quietly recovers from one of them returns an answer nobody
    // wrote, and for a filter that answer defaults to "let it through".

    #[test]
    fn a_chained_comparison_is_refused_rather_than_silently_truncated() {
        // Equality is non-associative here (`atom (op atom)?`). Without the
        // trailing-token check the extra `== e.c` would be dropped on the floor
        // and the predicate would answer about the first pair alone.
        let err = eval_predicate("e.a == e.b == e.c", &json!({"a":1,"b":1,"c":2}));
        assert!(err.is_err(), "chained comparison must not parse");
        assert!(format!("{}", err.unwrap_err()).contains("trailing tokens"));
    }

    #[test]
    fn an_unclosed_group_is_refused() {
        assert!(eval_predicate("(e.a == 1", &json!({"a":1})).is_err());
        assert!(eval_predicate("((e.a == 1)", &json!({"a":1})).is_err());
        assert!(eval_predicate("(e.a == 1 && e.b == 2", &json!({"a":1,"b":2})).is_err());
    }

    #[test]
    fn half_written_operators_are_refused() {
        // `bare_equals_is_error` covers `=`; the other three single characters
        // are each one keystroke away from a working operator, and each has its
        // own guard in the tokenizer.
        let event = json!({"a":true,"b":true});
        assert!(eval_predicate("e.a & e.b", &event).is_err(), "bare &");
        assert!(eval_predicate("e.a | e.b", &event).is_err(), "bare |");
        assert!(eval_predicate("e.a ! e.b", &event).is_err(), "bare !");
    }

    #[test]
    fn an_empty_predicate_is_refused_rather_than_passing_everything() {
        // The failure mode worth naming: an empty or arrow-only predicate that
        // parsed as "true" would gate nothing while looking like it gated.
        assert!(eval_predicate("", &json!({"a":1})).is_err());
        assert!(eval_predicate("   \n\t ", &json!({"a":1})).is_err());
        assert!(eval_predicate("e => ", &json!({"a":1})).is_err());
    }

    #[test]
    fn a_dangling_dot_is_refused() {
        assert!(eval_predicate("e. == 1", &json!({"a":1})).is_err());
        assert!(eval_predicate("e.a. == 1", &json!({"a":1})).is_err());
        assert!(eval_predicate("e..a == 1", &json!({"a":1})).is_err());
    }

    #[test]
    fn input_the_parser_cannot_account_for_is_refused() {
        // Left-over tokens, and a character the tokenizer has no rule for. A
        // tokenizer that skipped what it did not recognise would turn a typo
        // into a valid predicate with a different meaning.
        assert!(eval_predicate("e.a == 1 2", &json!({"a":1})).is_err());
        assert!(eval_predicate("e.a == 1 @", &json!({"a":1})).is_err());
        assert!(eval_predicate("e.a == 1 && ", &json!({"a":1})).is_err());
    }
}
