// Anchor-ontology-to-target-ontology ID mapping via xrefs.
//
// Author: Parker Hicks
// Date: 2026-09-20
// Copyright @2026

use std::collections::HashMap;

use crate::error::{OntoparseError, Result};
use crate::types::Ontology;

/// One row of an anchor-ontology-to-target-ontology xref mapping.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XrefMapping {
    pub anchor_id: String,
    pub anchor_name: Option<String>,
    pub target_xref: String,
}

/// Infer the anchoring ontology's term-id prefix for `ontology`.
///
/// Prefers the uppercased `ontology_id` header value; if absent, falls back to the
/// most common id prefix among `ontology.terms`. Returns `None` if neither is
/// available (e.g. an empty ontology with no header tag).
pub fn infer_anchor_prefix(ontology: &Ontology) -> Option<String> {
    if let Some(id) = &ontology.ontology_id {
        return Some(id.to_ascii_uppercase());
    }

    let mut counts: HashMap<&str, usize> = HashMap::new();
    for term_id in ontology.terms.keys() {
        if let Some((prefix, _)) = term_id.split_once(':') {
            *counts.entry(prefix).or_insert(0) += 1;
        }
    }
    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(prefix, _)| prefix.to_string())
}

/// Extract the CURIE-style ontology prefix from an xref value, if present.
///
/// Returns `None` for bare URLs (`http://...`, `https://...`) and any xref lacking a
/// `PREFIX:` form. Strips a trailing `{...}` qualifier block before parsing, since some
/// `xref:` lines carry OBO qualifier suffixes (e.g. `xref: pubmed:123 {source="pubmed"}`).
pub fn xref_prefix(xref: &str) -> Option<&str> {
    let xref = strip_trailing_qualifier(xref.trim());
    let (prefix, _rest) = xref.split_once(':')?;
    if prefix.is_empty()
        || prefix.eq_ignore_ascii_case("http")
        || prefix.eq_ignore_ascii_case("https")
    {
        return None;
    }
    Some(prefix)
}

fn strip_trailing_qualifier(value: &str) -> &str {
    let trimmed = value.trim_end();
    if trimmed.ends_with('}')
        && let Some(start) = trimmed.rfind('{')
    {
        return trimmed[..start].trim_end();
    }
    trimmed
}

/// Build an ID mapping from `ontology`'s anchoring terms to xrefs belonging to `to_prefix`.
/// The anchor prefix if from_prefix when provided. Otehrwise it is inferred as the most
/// prevalent prefix in the ontology.
///
/// Only terms whose own id prefix matches the ontology's inferred anchor prefix are
/// considered — imported/referenced terms from other ontologies (e.g. `GO`/`CL` terms
/// pulled into a UBERON file) are excluded. Matching against `to_prefix` is
/// case-insensitive. Returns one [`XrefMapping`] row per matching `(anchor term, xref)`
/// pair, in file order; an empty `Vec` (not an error) when nothing matches.
pub fn build_id_map(
    ontology: &Ontology,
    to_prefix: &str,
    from_prefix: Option<&str>,
) -> Result<Vec<XrefMapping>> {
    let anchor_prefix = from_prefix
        .map(str::to_owned)
        .or_else(|| infer_anchor_prefix(ontology))
        .ok_or(OntoparseError::NoAnchorPrefix)?;

    let mut rows = Vec::new();
    for term in ontology.terms.values() {
        let Some((term_prefix, _)) = term.id.split_once(':') else {
            continue;
        };
        if !term_prefix.eq_ignore_ascii_case(&anchor_prefix) {
            continue;
        }
        for xref in &term.xrefs {
            if let Some(prefix) = xref_prefix(xref)
                && prefix.eq_ignore_ascii_case(to_prefix)
            {
                rows.push(XrefMapping {
                    anchor_id: term.id.clone(),
                    anchor_name: term.name.clone(),
                    target_xref: strip_trailing_qualifier(xref.trim()).to_string(),
                });
            }
        }
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Term;

    fn ontology_with_terms(ontology_id: Option<&str>, terms: Vec<Term>) -> Ontology {
        let mut ontology = Ontology {
            ontology_id: ontology_id.map(String::from),
            ..Ontology::default()
        };
        for term in terms {
            ontology.terms.insert(term.id.clone(), term);
        }
        ontology
    }

    #[test]
    fn infers_anchor_prefix_from_header() {
        let ontology = ontology_with_terms(Some("uberon"), vec![]);
        assert_eq!(infer_anchor_prefix(&ontology), Some("UBERON".to_string()));
    }

    #[test]
    fn infers_anchor_prefix_by_plurality_when_header_absent() {
        let ontology = ontology_with_terms(
            None,
            vec![
                Term::new("UBERON:1"),
                Term::new("UBERON:2"),
                Term::new("UBERON:3"),
                Term::new("GO:1"),
            ],
        );
        assert_eq!(infer_anchor_prefix(&ontology), Some("UBERON".to_string()));
    }

    #[test]
    fn returns_none_when_no_header_and_no_terms() {
        let ontology = Ontology::default();
        assert_eq!(infer_anchor_prefix(&ontology), None);
    }

    #[test]
    fn xref_prefix_parses_simple_curie() {
        assert_eq!(xref_prefix("FMA:12247"), Some("FMA"));
    }

    #[test]
    fn xref_prefix_strips_trailing_qualifier_braces() {
        assert_eq!(
            xref_prefix(r#"pubmed:21614077 {source="pubmed"}"#),
            Some("pubmed")
        );
    }

    #[test]
    fn xref_prefix_rejects_bare_urls() {
        assert_eq!(
            xref_prefix("http://en.wikipedia.org/wiki/Goblet_cell"),
            None
        );
        assert_eq!(
            xref_prefix("https://en.wikipedia.org/wiki/Goblet_cell"),
            None
        );
    }

    #[test]
    fn xref_prefix_handles_escaped_colon_in_value() {
        assert_eq!(
            xref_prefix(r#"wikipedia.en:https\://en.wikipedia.org/wiki/Gas"#),
            Some("wikipedia.en")
        );
    }

    #[test]
    fn build_id_map_excludes_non_anchor_terms() {
        let mut uberon_term = Term::new("UBERON:0000001");
        uberon_term.xrefs.push("FMA:1".to_string());
        let mut go_term = Term::new("GO:0000001");
        go_term.xrefs.push("FMA:2".to_string());

        let ontology = ontology_with_terms(Some("uberon"), vec![uberon_term, go_term]);
        let rows = build_id_map(&ontology, "FMA").unwrap();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].anchor_id, "UBERON:0000001");
        assert_eq!(rows[0].target_xref, "FMA:1");
    }

    #[test]
    fn build_id_map_is_case_insensitive_on_to_prefix() {
        let mut term = Term::new("UBERON:0000001");
        term.xrefs.push("fma:1".to_string());
        let ontology = ontology_with_terms(Some("uberon"), vec![term]);

        let rows = build_id_map(&ontology, "FMA").unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].target_xref, "fma:1");
    }

    #[test]
    fn build_id_map_handles_one_to_many_and_many_to_one() {
        let mut term_a = Term::new("UBERON:0000001");
        term_a.xrefs.push("FMA:1".to_string());
        term_a.xrefs.push("FMA:2".to_string());
        let mut term_b = Term::new("UBERON:0000002");
        term_b.xrefs.push("FMA:9".to_string());
        let mut term_c = Term::new("UBERON:0000003");
        term_c.xrefs.push("FMA:9".to_string());

        let ontology = ontology_with_terms(Some("uberon"), vec![term_a, term_b, term_c]);
        let rows = build_id_map(&ontology, "FMA").unwrap();

        assert_eq!(rows.len(), 4);
        let fma9_anchors: Vec<&str> = rows
            .iter()
            .filter(|r| r.target_xref == "FMA:9")
            .map(|r| r.anchor_id.as_str())
            .collect();
        assert_eq!(fma9_anchors, vec!["UBERON:0000002", "UBERON:0000003"]);
    }

    #[test]
    fn build_id_map_returns_empty_vec_not_error_when_no_matches() {
        let term = Term::new("UBERON:0000001");
        let ontology = ontology_with_terms(Some("uberon"), vec![term]);
        let rows = build_id_map(&ontology, "FMA").unwrap();
        assert!(rows.is_empty());
    }

    #[test]
    fn build_id_map_errors_when_anchor_undeterminable() {
        let ontology = Ontology::default();
        let result = build_id_map(&ontology, "FMA");
        assert!(matches!(result, Err(OntoparseError::NoAnchorPrefix)));
    }
}
