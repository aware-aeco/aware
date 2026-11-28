//! Version 1 of the IFC writer's per-object typed property sets.

use super::{Spf, is_meta_text, record_guid, s_lit};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
};

/// File-wide enumeration references and project SI units.
pub(super) struct PropertyWriter {
    length: i64,
    area: i64,
    volume: i64,
    // Kilograms are created lazily because older files omit the mass unit.
    mass: Option<i64>,
    // IFC4 requires enumeration names to be unique across the file.
    enumerations: BTreeMap<String, (Vec<String>, i64)>,
}

impl PropertyWriter {
    pub(super) fn new(length: i64, area: i64, volume: i64, mass: Option<i64>) -> Self {
        Self {
            length,
            area,
            volume,
            mass,
            enumerations: BTreeMap::new(),
        }
    }
}

// Preserve the existing Part 21 escaping for BMP characters. Supplementary scalars use
// the eight-digit directive so a reader doesn't have to interpret surrogate pairs.
pub(super) fn step_string(value: &str) -> String {
    let mut out = String::from("'");
    let mut start = 0;
    for (index, ch) in value
        .char_indices()
        .filter(|(_, ch)| u32::from(*ch) > 0xffff)
    {
        let part = s_lit(&value[start..index]);
        // `s_lit` wraps its output in one quote at each end; keep only the body.
        let body = part.strip_prefix('\'').unwrap_or(&part);
        out.push_str(body.strip_suffix('\'').unwrap_or(body));
        let _ = write!(out, "\\X4\\{:08X}\\X0\\", u32::from(ch));
        start = index + ch.len_utf8();
    }
    let part = s_lit(&value[start..]);
    // Keep the closing quote of the last part; the opening one is already in `out`.
    out.push_str(part.strip_prefix('\'').unwrap_or(&part));
    out
}

fn typed_real(value: f64) -> String {
    // The geometry formatter rounds to eleven decimal places. Typed values must also
    // preserve small nonzero numbers, and STEP requires a decimal point in the mantissa.
    let real = format!("{value:?}");
    match real.split_once('e') {
        Some((mantissa, exponent)) if !mantissa.contains('.') => {
            format!("{mantissa}.0E{exponent}")
        }
        Some((mantissa, exponent)) => format!("{mantissa}E{exponent}"),
        None => real,
    }
}

fn label_value(value: &Value) -> Result<String, String> {
    let text = value.as_str().ok_or("expected a string label")?;
    if text.len() > 255 {
        return Err("label exceeds 255 UTF-8 bytes; use type text for longer strings".into());
    }
    Ok(format!("IFCLABEL({})", step_string(text)))
}

struct Measure {
    entity: &'static str,
    project_symbol: &'static str,
    units: &'static [(&'static str, f64)],
}

fn measure(kind: &str) -> Option<Measure> {
    // Multipliers are expressed in the corresponding project unit.
    let (entity, project_symbol, units): (_, _, &[_]) = match kind {
        "length" => (
            "IFCLENGTHMEASURE",
            "mm",
            &[
                ("mm", 1.0),
                ("cm", 10.0),
                ("m", 1000.0),
                ("in", 25.4),
                ("ft", 304.8),
            ],
        ),
        "area" => (
            "IFCAREAMEASURE",
            "m2",
            &[
                ("mm2", 1e-6),
                ("cm2", 1e-4),
                ("m2", 1.0),
                ("in2", 0.00064516),
                ("ft2", 0.09290304),
            ],
        ),
        "volume" => (
            "IFCVOLUMEMEASURE",
            "m3",
            &[
                ("mm3", 1e-9),
                ("cm3", 1e-6),
                ("m3", 1.0),
                ("in3", 0.000016387064),
                ("ft3", 0.028316846592),
            ],
        ),
        "mass" => (
            "IFCMASSMEASURE",
            "kg",
            &[("g", 0.001), ("kg", 1.0), ("t", 1000.0), ("lb", 0.45359237)],
        ),
        _ => return None,
    };
    Some(Measure {
        entity,
        project_symbol,
        units,
    })
}

fn emit_measure(
    spf: &mut Spf,
    writer: &mut PropertyWriter,
    measure: Measure,
    name: &str,
    value: &Value,
    unit: Option<&Value>,
) -> Result<(i64, Option<String>), String> {
    let value = value.as_f64().ok_or("expected a finite JSON number")?;
    let source = match unit {
        Some(unit) => unit.as_str().ok_or("unit must be a string")?,
        None => measure.project_symbol,
    };
    let symbol = source.replace('²', "2").replace('³', "3");
    let factor = measure
        .units
        .iter()
        .find(|(name, _)| *name == symbol)
        .map(|(_, factor)| *factor)
        .ok_or_else(|| format!("unsupported unit '{source}' for {}", measure.entity))?;
    let converted = value * factor;
    if !converted.is_finite() || (value != 0.0 && converted == 0.0) {
        return Err("measure is out of range in the project unit".into());
    }
    let unit_ref = match measure.project_symbol {
        "mm" => writer.length,
        "m2" => writer.area,
        "m3" => writer.volume,
        _ => *writer
            .mass
            .get_or_insert_with(|| spf.emit("IFCSIUNIT(*,.MASSUNIT.,.KILO.,.GRAM.)")),
    };
    let property = spf.emit(&format!(
        "IFCPROPERTYSINGLEVALUE({name},$,{}({}),#{unit_ref})",
        measure.entity,
        typed_real(converted)
    ));
    Ok((
        property,
        (factor != 1.0).then(|| format!("{source} -> {}", measure.project_symbol)),
    ))
}

fn enumeration_labels(values: &[Value], role: &str) -> Result<Vec<String>, String> {
    if values.is_empty() {
        return Err(format!("enumerated {role} must be non-empty"));
    }
    let mut seen = BTreeSet::new();
    values
        .iter()
        .map(|value| {
            let label = label_value(value)?;
            if !seen.insert(label.clone()) {
                return Err(format!("enumerated {role} must be unique"));
            }
            Ok(label)
        })
        .collect()
}

fn emit_enumeration(
    spf: &mut Spf,
    writer: &mut PropertyWriter,
    name: &str,
    value: &Value,
    choices: Option<&Value>,
) -> Result<i64, String> {
    let values = value
        .as_array()
        .map_or(std::slice::from_ref(value), Vec::as_slice);
    let selected = enumeration_labels(values, "selections")?;
    let choices = match choices {
        Some(choices) => {
            let choices = choices
                .as_array()
                .ok_or("enumerated choices must be an array")?;
            let choices = enumeration_labels(choices, "choices")?;
            for (value, label) in values.iter().zip(&selected) {
                if !choices.contains(label) {
                    return Err(format!(
                        "selected value {value} is not in enumerated choices"
                    ));
                }
            }
            choices
        }
        None => selected.clone(),
    };

    let literal = step_string(name);
    let enumeration = match writer.enumerations.get(name) {
        Some((previous, reference)) if *previous == choices => *reference,
        Some(_) => {
            return Err(format!(
                "enumeration '{name}' has conflicting ordered choices"
            ));
        }
        None => {
            let reference = spf.emit(&format!(
                "IFCPROPERTYENUMERATION({literal},({}),$)",
                choices.join(",")
            ));
            writer
                .enumerations
                .insert(name.to_owned(), (choices, reference));
            reference
        }
    };
    Ok(spf.emit(&format!(
        "IFCPROPERTYENUMERATEDVALUE({literal},$,({}),#{enumeration})",
        selected.join(",")
    )))
}

fn emit_property(
    spf: &mut Spf,
    writer: &mut PropertyWriter,
    property: &Value,
    name: &str,
) -> Result<(i64, Option<String>), String> {
    let kind = property["type"].as_str().ok_or("type must be a string")?;
    let value = property.get("value").ok_or("value is required")?;
    let unit = property.get("unit");
    if let Some(measure) = measure(kind) {
        return emit_measure(spf, writer, measure, &step_string(name), value, unit);
    }
    if unit.is_some() {
        return Err(format!(
            "unit is only supported for measures, got type '{kind}'"
        ));
    }
    let nominal = match kind {
        "label" => label_value(value)?,
        "text" => format!(
            "IFCTEXT({})",
            step_string(value.as_str().ok_or("expected a text string")?)
        ),
        "integer" => {
            let number = value
                .as_i64()
                .ok_or("expected a signed 64-bit JSON integer")?;
            format!("IFCINTEGER({number})")
        }
        "real" => {
            let number = value
                .as_f64()
                .filter(|n| n.is_finite())
                .ok_or("expected a finite JSON number")?;
            format!("IFCREAL({})", typed_real(number))
        }
        "boolean" => {
            let boolean = value.as_bool().ok_or("expected a JSON boolean")?;
            format!("IFCBOOLEAN(.{}.)", if boolean { "T" } else { "F" })
        }
        "date" => {
            let date = value
                .as_str()
                .ok_or("expected a date string in YYYY-MM-DD form")?;
            let parsed = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").ok();
            if date.len() != 10 || parsed.is_none_or(|day| day.to_string() != date) {
                return Err("expected a valid calendar date in YYYY-MM-DD form".into());
            }
            format!("IFCDATE('{date}')")
        }
        "enumerated" => {
            let reference = emit_enumeration(spf, writer, name, value, property.get("choices"))?;
            return Ok((reference, None));
        }
        _ => return Err(format!("unknown property type '{kind}'")),
    };
    let property = spf.emit(&format!(
        "IFCPROPERTYSINGLEVALUE({},$,{nominal},$)",
        step_string(name)
    ));
    Ok((property, None))
}

pub(super) fn emit_property_sets(
    spf: &mut Spf,
    writer: &mut PropertyWriter,
    receipts: &mut Vec<Value>,
    record: &Value,
    id: &str,
    object: Result<i64, &str>,
    namespace: &str,
) {
    let Some(sets) = record.get("propertySets") else {
        return;
    };
    let Some(sets) = sets.as_array() else {
        let reason = object.err().unwrap_or("propertySets must be an array");
        receipts.push(json!({"id": id, "status": "not-written", "reason": reason}));
        return;
    };

    let mut set_names = BTreeSet::new();
    let kind = record["kind"].as_str().unwrap_or("member");
    if (kind == "extrusion"
        || (matches!(kind, "member" | "line" | "box") && record.get("positions").is_none()))
        && let Some(legacy) = record["meta"]["properties"]["name"].as_str()
    {
        set_names.insert(legacy);
    }
    for (set_index, set) in sets.iter().enumerate() {
        let name = set["name"].as_str().unwrap_or_default();
        let set_error = if let Err(reason) = object {
            Some(reason)
        } else if !is_meta_text(name, 255) || name.trim().is_empty() {
            Some(
                "property set name must be a non-blank label of at most 255 bytes without control characters",
            )
        } else if name
            .get(..5)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("pset_"))
            || name
                .get(..4)
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("qto_"))
        {
            Some("property set name uses a reserved Pset_ or Qto_ prefix")
        } else if !set_names.insert(name) {
            Some("duplicate property set name on this object")
        } else {
            None
        };
        let Some(properties) = set["properties"].as_array().filter(|list| !list.is_empty()) else {
            receipts.push(json!({
                "id": id, "set": set["name"].as_str(), "setIndex": set_index,
                "status": "not-written",
                "reason": object.err().unwrap_or("properties must be a non-empty array")
            }));
            continue;
        };
        let mut refs = Vec::new();
        let mut property_names = BTreeSet::new();
        for (property_index, property) in properties.iter().enumerate() {
            let property_name = property["name"].as_str().unwrap_or_default();
            let mut row = json!({
                "id": id, "set": set["name"].as_str(), "setIndex": set_index,
                "property": property["name"].as_str(), "propertyIndex": property_index,
                "type": property["type"].as_str()
            });
            let result = if let Some(error) = set_error {
                Err(error.into())
            } else if !is_meta_text(property_name, 255) || property_name.trim().is_empty() {
                Err("property name must be a non-blank identifier of at most 255 bytes without control characters".into())
            } else if property_names.contains(property_name) {
                Err("duplicate property name in this set".into())
            } else {
                emit_property(spf, writer, property, property_name)
            };
            match result {
                Ok((reference, conversion)) => {
                    property_names.insert(property_name);
                    refs.push(format!("#{reference}"));
                    if let Some(conversion) = conversion {
                        row["status"] = json!("converted");
                        row["conversion"] = json!(conversion);
                    } else {
                        row["status"] = json!("written");
                    }
                }
                Err(reason) => {
                    row["status"] = json!("not-written");
                    row["reason"] = json!(reason);
                }
            }
            receipts.push(row);
        }
        if let Ok(object) = object
            && !refs.is_empty()
        {
            let key = format!("{namespace}:{}:{id}{name}", id.len());
            let set_guid = record_guid("typed-property-set", &key);
            let relation_guid = record_guid("typed-property-set-relation", &key);
            let set = spf.emit(&format!(
                "IFCPROPERTYSET('{set_guid}',$,{},$,({}))",
                step_string(name),
                refs.join(",")
            ));
            spf.emit(&format!(
                "IFCRELDEFINESBYPROPERTIES('{relation_guid}',$,$,$,(#{object}),#{set})"
            ));
        }
    }
}
