// Loaders for supported file types.
//
// Author: Parker Hicks
// Date: 2026-09-18
// Copyright @2026

use std::path::Path;

use ontoparse_core::loader::Loader;
use ontoparse_core::parser::parse_obo;
use ontoparse_core::{Ontology, Result};

/// Loads OBO-format ontology files.
pub struct OboLoader;

impl Loader for OboLoader {
    type Output = Ontology;

    fn load(&self, path: &Path) -> Result<Ontology> {
        parse_obo(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn loads_simple_obo_file() {
        let mut file = tempfile_with_content(
            "format-version: 1.2\n\n[Term]\nid: X:1\nname: thing\n",
        );
        let path = file.path().to_path_buf();
        file.flush().unwrap();

        let onto = OboLoader.load(&path).unwrap();
        assert_eq!(onto.format_version.as_deref(), Some("1.2"));
        assert_eq!(onto.terms["X:1"].name.as_deref(), Some("thing"));
    }

    fn tempfile_with_content(content: &str) -> tempfile::NamedTempFile {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }
}
