// Error calls for ontoparse
//
// Author: Parker Hicks
// Date: 2026-09-18
// Copyright @2026

use loaders::OntoFile;

/// Shared errors for ontoparse packages
#[derive(Debug, thiserror::Error)]
pub enum OntoparseError {

    #[error("File not found: {0:?}")]
     FileNotFound(OntoFile)
}
