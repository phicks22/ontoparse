// Core module defining data types.

pub mod error;
pub mod file;
pub mod idmap;
pub mod loader;
pub mod parser;
pub mod types;

pub use error::{OntoparseError, Result};
pub use idmap::{build_id_map, XrefMapping};
pub use types::{Ontology, OntoFileType, Term};
