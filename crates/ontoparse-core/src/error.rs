// Error types for ontoparse
//
// Author: Parker Hicks
// Date: 2026-09-18
// Copyright @2026

use std::path::PathBuf;

/// Shared errors for ontoparse packages.
#[derive(Debug, thiserror::Error)]
pub enum OntoparseError {
    #[error("I/O error reading {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("file appears to be binary, not a text ontology file: {0}")]
    BinaryFile(PathBuf),

    #[error("malformed OBO line {line}: {message}")]
    MalformedLine { line: usize, message: String },

    #[error("unsupported ontology file type: {0}")]
    UnsupportedFormat(String),

    #[error(
        "could not determine the anchoring ontology's id prefix (no `ontology:` header and no terms to infer from)"
    )]
    NoAnchorPrefix,
}

pub type Result<T> = std::result::Result<T, OntoparseError>;
