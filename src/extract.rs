use std::collections::HashSet;

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
/// Matching nodes are collected from the entire tree (recursively).
/// The subtree of each matched node is preserved as-is.
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
            result.push(child.clone());
        } else if let Some(nested) = child.children() {
            result.extend(collect_by_type(nested, types));
        }
    }
    result
}
