use thiserror::Error;

#[derive(Debug, Error)]
pub enum PenError {
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse .pen JSON: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("Unresolved ref: {0}")]
    UnresolvedRef(String),

    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),

    #[error("Circular ref detected: {0}")]
    CircularRef(String),
}
