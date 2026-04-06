use std::path::Path;

use anyhow::Context;

use crate::model::document::Document;

pub fn parse_document(path: &Path) -> anyhow::Result<Document> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
    let doc: Document = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path.display()))?;
    Ok(doc)
}

pub fn parse_from_str(json: &str) -> anyhow::Result<Document> {
    let doc: Document = serde_json::from_str(json)?;
    Ok(doc)
}
