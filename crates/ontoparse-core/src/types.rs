// Type definitions
//
// Author: Parker Hicks
// Date: 2026-09-18
// Copyright @2026

use indexmap::IndexMap;

/// Ontology file type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OntoFileType {
    OBO,
    OWL,
}

/// A parsed ontology: header metadata plus its terms and typedefs.
#[derive(Debug, Clone, Default)]
pub struct Ontology {
    pub format_version: Option<String>,
    pub data_version: Option<String>,
    pub ontology_id: Option<String>,
    /// Terms keyed by their id (e.g. "BFO:0000001"), in file order.
    pub terms: IndexMap<String, Term>,
    /// Typedefs (relationship type declarations) keyed by their id.
    pub typedefs: IndexMap<String, TypeDef>,
}

/// A single `[Term]` stanza.
#[derive(Debug, Clone, PartialEq)]
pub struct Term {
    pub id: String,
    pub name: Option<String>,
    pub definition: Option<Definition>,
    pub synonyms: Vec<Synonym>,
    pub xrefs: Vec<String>,
    /// Parent term ids from `is_a` lines.
    pub is_a: Vec<String>,
    /// Typed relationship edges from `relationship` lines.
    pub relationships: Vec<Relationship>,
    pub disjoint_from: Vec<String>,
    pub is_obsolete: bool,
    pub subsets: Vec<String>,
}

impl Term {
    pub fn new(id: impl Into<String>) -> Self {
        Term {
            id: id.into(),
            name: None,
            definition: None,
            synonyms: Vec::new(),
            xrefs: Vec::new(),
            is_a: Vec::new(),
            relationships: Vec::new(),
            disjoint_from: Vec::new(),
            is_obsolete: false,
            subsets: Vec::new(),
        }
    }
}

/// A `def:` line: free-text definition plus its supporting xrefs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    pub text: String,
    pub xrefs: Vec<String>,
}

/// A `synonym:` line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Synonym {
    pub text: String,
    pub scope: SynonymScope,
    pub xrefs: Vec<String>,
}

/// The scope qualifier on a `synonym:` line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SynonymScope {
    Exact,
    Broad,
    Narrow,
    Related,
}

/// A typed edge from a `relationship:` line, e.g. `relationship: part_of UBERON:0001062`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relationship {
    pub predicate: String,
    pub target: String,
}

/// A `[Typedef]` stanza.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TypeDef {
    pub id: String,
    pub name: Option<String>,
}
