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

#![allow(dead_code)]

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
    fn parens_and_or() {
        let code = "(e.x == 1) || (e.y == 2)";
        assert!(eval_predicate(code, &json!({"x":1})).unwrap());
        assert!(eval_predicate(code, &json!({"y":2})).unwrap());
        assert!(!eval_predicate(code, &json!({"x":3,"y":4})).unwrap());
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
}
