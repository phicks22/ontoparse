// Loaders for supported file types.
//
// Author: Parker Hicks
// Date: 2026-09-18
// Copyright @2026

use ontoparse_core::types::OntoFileType;
use ontoparse_core::loaders::Loader;


pub struct OboLoader {
    file: OboFile;
}

impl Loader for OboLoader {

    /// Check for proper file type and structure
    fn check(){

    }
}
