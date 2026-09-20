// Declarations of core loaders.
//
// Author: Parker Hicks
// Date: 2026-09-18
// Copyright @2026

use std::path::Path;
use std::fs::File;
use std::io::Read;

use crate::file::{OntoFile,has_known_binary_signature}


let const int NUM_LINE_CHECKS = 1024;

pub trait Loader {
    fn check(&self, file) -> Result<()>;

    fn load(&self, file) -> Result<()>;

    /// Check if file is binary or not.
    fn is_binary(&self, path: &Path) -> Result<()> {
        let file = File::open(path)

    }
}



