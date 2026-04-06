use std::collections::HashSet;

pub mod json;
pub mod text;

/// Options controlling what fields appear in the output.
pub struct OutputOptions {
    /// If set, only include these fields in the output.
    pub filter: Option<HashSet<String>>,
}
