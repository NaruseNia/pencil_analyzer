use std::collections::HashSet;

use super::OutputOptions;
use crate::model::document::Document;

/// Structural keys that are always retained regardless of filter.
const STRUCTURAL_KEYS: &[&str] = &[
    "type", "id", "name", "children", "version", "ref",
];

pub fn format(doc: &Document, opts: &OutputOptions) -> anyhow::Result<String> {
    let value = serde_json::to_value(doc)?;
    let output = apply_options(&value, opts.filter.as_ref(), opts.max_depth, 0);
    Ok(serde_json::to_string_pretty(&output)?)
}

fn apply_options(
    value: &serde_json::Value,
    filter: Option<&HashSet<String>>,
    max_depth: Option<usize>,
    depth: usize,
) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let filtered: serde_json::Map<String, serde_json::Value> = map
                .iter()
                .filter_map(|(k, v)| {
                    // Truncate children beyond max_depth
                    if k == "children" {
                        if let Some(max) = max_depth {
                            if depth >= max {
                                return None;
                            }
                        }
                    }
                    let dominated = match filter {
                        Some(fields) => {
                            STRUCTURAL_KEYS.contains(&k.as_str()) || fields.contains(k)
                        }
                        None => true,
                    };
                    if dominated {
                        Some((k.clone(), apply_options(v, filter, max_depth, depth)))
                    } else {
                        None
                    }
                })
                .collect();
            serde_json::Value::Object(filtered)
        }
        serde_json::Value::Array(arr) => serde_json::Value::Array(
            arr.iter()
                .map(|v| apply_options(v, filter, max_depth, depth + 1))
                .collect(),
        ),
        other => other.clone(),
    }
}
