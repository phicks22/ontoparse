// Declarations of core loaders.
//
// Author: Parker Hicks
// Date: 2026-09-18
// Copyright @2026

use std::path::Path;

use crate::error::{OntoparseError, Result};
use crate::file::has_known_binary_signature;

/// Number of leading bytes read from a file when sniffing for a binary signature.
const SNIFF_BYTES: usize = 1024;

/// A loader that parses a specific ontology file format into `Output`.
pub trait Loader {
    type Output;

    fn load(&self, path: &Path) -> Result<Self::Output>;
}

/// Read the first `SNIFF_BYTES` of `path` and check them against known binary signatures.
pub fn is_binary_file(path: &Path) -> Result<bool> {
    use std::io::Read;

    let mut file = std::fs::File::open(path).map_err(|source| OntoparseError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let mut buf = vec![0u8; SNIFF_BYTES];
    let n = file.read(&mut buf).map_err(|source| OntoparseError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    buf.truncate(n);
    Ok(has_known_binary_signature(&buf))
}

/// Return an error if `path` looks like a binary file rather than a text ontology file.
pub fn reject_if_binary(path: &Path) -> Result<()> {
    if is_binary_file(path)? {
        return Err(OntoparseError::BinaryFile(path.to_path_buf()));
    }
    Ok(())
}
