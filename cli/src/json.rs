//! Shared `serde_json` helpers.
//!
//! One home for the small JSON utilities that every value-shaped validation path
//! needs, so a change to how the substrate *words* a shape lands everywhere at
//! once instead of in whichever module the author happened to be editing.

use serde_json::Value;

/// The JSON type of `value`, spelled the way a validation message spells it —
/// `"must be an array (got string)"`.
///
/// Five modules had each grown a private, byte-identical copy of this match:
/// `render::file`, `render::ifc`, `render::viewer_3d`, `manifest::expose` and
/// `render::ui`. Nothing forced them to agree, so the words a user sees for a
/// rejected value depended on which verb rejected it.
pub fn type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn names_every_json_shape() {
        assert_eq!(type_name(&Value::Null), "null");
        assert_eq!(type_name(&json!(true)), "boolean");
        assert_eq!(type_name(&json!(1)), "number");
        assert_eq!(type_name(&json!(1.5)), "number");
        assert_eq!(type_name(&json!("s")), "string");
        assert_eq!(type_name(&json!([])), "array");
        assert_eq!(type_name(&json!({})), "object");
    }
}
