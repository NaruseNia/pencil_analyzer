use std::collections::HashSet;

use anyhow::{bail, Result};

use crate::model::document::Document;
use crate::model::objects::Child;

use super::index::NodeIndex;

/// Resolve all `ref` nodes in the document by expanding them into full subtrees.
pub fn resolve_refs(doc: &Document) -> Result<Document> {
    let index = NodeIndex::build(doc);
    let mut resolved = doc.clone();
    resolved.children = resolve_children(&doc.children, &index, &mut HashSet::new())?;
    Ok(resolved)
}

fn resolve_children(
    children: &[Child],
    index: &NodeIndex,
    visited: &mut HashSet<String>,
) -> Result<Vec<Child>> {
    children
        .iter()
        .map(|child| resolve_child(child, index, visited))
        .collect()
}

fn resolve_child(
    child: &Child,
    index: &NodeIndex,
    visited: &mut HashSet<String>,
) -> Result<Child> {
    match child {
        Child::Ref(ref_node) => {
            let ref_id = &ref_node.ref_id;

            if visited.contains(ref_id) {
                bail!("Circular ref detected: {ref_id}");
            }

            let target = index
                .get(ref_id)
                .ok_or_else(|| anyhow::anyhow!("Unresolved ref: {ref_id}"))?;

            // Clone the target and apply overrides
            let mut resolved = target.clone();

            // Apply property overrides from the ref node via JSON merge
            apply_overrides(&mut resolved, ref_node)?;

            // Recursively resolve any refs within the resolved subtree FIRST,
            // so that slash-path descendants (e.g. "ok-btn/label") can reach
            // into already-resolved nested refs.
            visited.insert(ref_id.clone());
            let mut result = resolve_child(&resolved, index, visited)?;
            visited.remove(ref_id);

            // Apply descendant overrides AFTER inner refs are resolved
            if let Some(descendants) = &ref_node.descendants {
                apply_descendants(&mut result, descendants)?;
            }

            Ok(result)
        }
        Child::Frame(f) => {
            let mut f = f.clone();
            if let Some(children) = &f.children {
                f.children = Some(resolve_children(children, index, visited)?);
            }
            Ok(Child::Frame(f))
        }
        Child::Group(g) => {
            let mut g = g.clone();
            if let Some(children) = &g.children {
                g.children = Some(resolve_children(children, index, visited)?);
            }
            Ok(Child::Group(g))
        }
        other => Ok(other.clone()),
    }
}

/// Apply property overrides from a Ref node onto the resolved target.
/// This merges the ref's override properties into the target's JSON representation.
fn apply_overrides(target: &mut Child, ref_node: &crate::model::objects::Ref) -> Result<()> {
    // Serialize target to JSON, merge overrides, deserialize back
    let mut target_json = serde_json::to_value(&*target)?;

    if let serde_json::Value::Object(ref mut map) = target_json {
        for (key, value) in &ref_node.overrides {
            map.insert(key.clone(), value.clone());
        }
        // Preserve the original id from the ref node, not the target
        map.insert(
            "id".to_string(),
            serde_json::Value::String(ref_node.entity.id.clone()),
        );
        // Preserve position from ref node if specified
        if let Some(x) = ref_node.entity.x {
            map.insert("x".to_string(), serde_json::Value::from(x));
        }
        if let Some(y) = ref_node.entity.y {
            map.insert("y".to_string(), serde_json::Value::from(y));
        }
    }

    *target = serde_json::from_value(target_json)?;
    Ok(())
}

/// Apply descendant overrides. Each key in the descendants map is an ID path
/// (e.g., "label" or "ok-button/label") pointing to a nested node.
fn apply_descendants(
    target: &mut Child,
    descendants: &std::collections::HashMap<String, serde_json::Value>,
) -> Result<()> {
    for (path, override_value) in descendants {
        let parts: Vec<&str> = path.split('/').collect();
        apply_descendant_at_path(target, &parts, override_value)?;
    }
    Ok(())
}

fn apply_descendant_at_path(
    node: &mut Child,
    path: &[&str],
    override_value: &serde_json::Value,
) -> Result<()> {
    if path.is_empty() {
        return Ok(());
    }

    let target_id = path[0];
    let remaining = &path[1..];

    let children = match node {
        Child::Frame(f) => f.children.as_mut(),
        Child::Group(g) => g.children.as_mut(),
        _ => return Ok(()),
    };

    let Some(children) = children else {
        return Ok(());
    };

    for child in children.iter_mut() {
        if child.id() == target_id {
            if remaining.is_empty() {
                // Check if this is a replacement (has "type" field) or property override
                if let serde_json::Value::Object(obj) = override_value {
                    if obj.contains_key("type") {
                        // Full object replacement
                        *child = serde_json::from_value(serde_json::Value::Object(obj.clone()))?;
                    } else if obj.contains_key("children") {
                        // Children replacement — keep the node, replace its children
                        let mut child_json = serde_json::to_value(&*child)?;
                        if let serde_json::Value::Object(ref mut map) = child_json {
                            map.insert("children".to_string(), obj["children"].clone());
                        }
                        *child = serde_json::from_value(child_json)?;
                    } else {
                        // Property override — merge into existing node
                        let mut child_json = serde_json::to_value(&*child)?;
                        if let serde_json::Value::Object(ref mut map) = child_json {
                            for (k, v) in obj {
                                map.insert(k.clone(), v.clone());
                            }
                        }
                        *child = serde_json::from_value(child_json)?;
                    }
                }
            } else {
                apply_descendant_at_path(child, remaining, override_value)?;
            }
            return Ok(());
        }
        // Also search into nested children (the target might be deeper)
        if remaining.is_empty() {
            // Try searching recursively if this child has children
            apply_descendant_at_path(child, path, override_value)?;
        }
    }

    Ok(())
}
