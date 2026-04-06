use std::collections::HashSet;

use super::OutputOptions;
use crate::model::document::Document;

/// Structural keys that are always retained regardless of filter.
const STRUCTURAL_KEYS: &[&str] = &[
    "type", "id", "name", "children", "version", "ref",
];

pub fn format(doc: &Document, opts: &OutputOptions) -> anyhow::Result<String> {
    let value = serde_json::to_value(doc)?;
    let output = match &opts.filter {
        Some(fields) => filter_value(&value, fields),
        None => value,
    };
    Ok(serde_json::to_string_pretty(&output)?)
}

fn filter_value(value: &serde_json::Value, fields: &HashSet<String>) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let filtered: serde_json::Map<String, serde_json::Value> = map
                .iter()
                .filter_map(|(k, v)| {
                    if STRUCTURAL_KEYS.contains(&k.as_str()) || fields.contains(k) {
                        Some((k.clone(), filter_value(v, fields)))
                    } else {
                        None
                    }
                })
                .collect();
            serde_json::Value::Object(filtered)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(|v| filter_value(v, fields)).collect())
        }
        other => other.clone(),
    }
}
