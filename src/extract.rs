use std::collections::HashSet;

use regex::Regex;

use crate::model::document::Document;
use crate::model::objects::Child;

/// Extract only the requested categories from a document.
///
/// Supported categories:
/// - `components`: reusable nodes (and their subtrees)
/// - `variables`: variable definitions
/// - `imports`: import declarations
/// - `themes`: theme definitions
pub fn extract_document(doc: &Document, categories: &HashSet<String>) -> Document {
    let children = if categories.contains("components") {
        collect_components(&doc.children)
    } else {
        vec![]
    };

    Document {
        version: doc.version.clone(),
        themes: if categories.contains("themes") {
            doc.themes.clone()
        } else {
            None
        },
        imports: if categories.contains("imports") {
            doc.imports.clone()
        } else {
            None
        },
        variables: if categories.contains("variables") {
            doc.variables.clone()
        } else {
            None
        },
        children,
    }
}

/// Recursively collect nodes that have `reusable == true`.
fn collect_components(children: &[Child]) -> Vec<Child> {
    let mut result = Vec::new();
    for child in children {
        if child.entity().reusable == Some(true) {
            result.push(child.clone());
        } else if let Some(nested) = child.children() {
            result.extend(collect_components(nested));
        }
    }
    result
}

/// Filter document children to only include nodes matching the given type names.
/// The filter applies recursively — children of matched nodes are also filtered.
pub fn filter_by_type(doc: &Document, types: &HashSet<String>) -> Document {
    Document {
        version: doc.version.clone(),
        themes: doc.themes.clone(),
        imports: doc.imports.clone(),
        variables: doc.variables.clone(),
        children: collect_by_type(&doc.children, types),
    }
}

fn collect_by_type(children: &[Child], types: &HashSet<String>) -> Vec<Child> {
    let mut result = Vec::new();
    for child in children {
        if types.contains(child.type_name()) {
            // Node matches — include it, but also filter its children recursively
            if let Some(nested) = child.children() {
                let filtered = collect_by_type(nested, types);
                result.push(child.with_children(filtered));
            } else {
                result.push(child.clone());
            }
        } else if let Some(nested) = child.children() {
            // Node doesn't match — skip it but search deeper
            result.extend(collect_by_type(nested, types));
        }
    }
    result
}

/// Filter nodes whose hierarchical path (built from names/IDs) matches the regex.
/// A matched node is included with its full subtree.
/// Path format: `"ParentName/ChildName/..."` (uses name if available, otherwise id).
pub fn filter_by_regex(doc: &Document, re: &Regex) -> Document {
    Document {
        version: doc.version.clone(),
        themes: doc.themes.clone(),
        imports: doc.imports.clone(),
        variables: doc.variables.clone(),
        children: collect_by_regex(&doc.children, re, ""),
    }
}

fn node_label(child: &Child) -> &str {
    child
        .entity()
        .name
        .as_deref()
        .unwrap_or_else(|| child.id())
}

fn collect_by_regex(children: &[Child], re: &Regex, parent_path: &str) -> Vec<Child> {
    let mut result = Vec::new();
    for child in children {
        let label = node_label(child);
        let path = if parent_path.is_empty() {
            label.to_string()
        } else {
            format!("{parent_path}/{label}")
        };

        if re.is_match(&path) {
            // Node matches — include with full subtree
            result.push(child.clone());
        } else if let Some(nested) = child.children() {
            // Not matched — recurse into children
            let matched = collect_by_regex(nested, re, &path);
            if !matched.is_empty() {
                result.push(child.with_children(matched));
            }
        }
    }
    result
}
