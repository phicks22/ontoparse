// OBO format parser.
//
// Author: Parker Hicks
// Date: 2026-09-18
// Copyright @2026

use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use crate::error::{OntoparseError, Result};
use crate::loader::reject_if_binary;
use crate::types::{Definition, Ontology, Relationship, Synonym, SynonymScope, Term, TypeDef};

/// Which stanza is currently being accumulated while parsing.
enum Stanza {
    Header,
    Term(Term),
    Typedef(TypeDef),
    /// A recognized-but-unhandled stanza type (e.g. `[Instance]`); its lines are skipped.
    Other,
}

/// Parse an OBO file at `path` into an [`Ontology`].
pub fn parse_obo(path: &Path) -> Result<Ontology> {
    reject_if_binary(path)?;
    let file = std::fs::File::open(path).map_err(|source| OntoparseError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    parse_obo_reader(BufReader::new(file)).map_err(|e| match e {
        OntoparseError::MalformedLine { line, message } => OntoparseError::MalformedLine {
            line,
            message: format!("{} (in {})", message, path.display()),
        },
        other => other,
    })
}

/// Parse OBO content from any reader, for testing without touching disk.
pub fn parse_obo_reader<R: Read>(reader: BufReader<R>) -> Result<Ontology> {
    let mut ontology = Ontology::default();
    let mut stanza = Stanza::Header;

    for (idx, line) in reader.lines().enumerate() {
        let line_no = idx + 1;
        let line = line.map_err(|source| OntoparseError::Io {
            path: std::path::PathBuf::new(),
            source,
        })?;
        let line = line.trim_end();

        if line.is_empty() {
            continue;
        }

        if let Some(header) = parse_stanza_header(line) {
            flush_stanza(&mut ontology, std::mem::replace(&mut stanza, new_stanza(header)));
            continue;
        }

        let Some((tag, rest)) = split_tag_value(line) else {
            continue;
        };
        let value = strip_trailing_comment(rest);

        match &mut stanza {
            Stanza::Header => apply_header_tag(&mut ontology, tag, value),
            Stanza::Term(term) => apply_term_tag(term, tag, value, line_no)?,
            Stanza::Typedef(typedef) => apply_typedef_tag(typedef, tag, value),
            Stanza::Other => {}
        }
    }

    flush_stanza(&mut ontology, stanza);
    Ok(ontology)
}

fn new_stanza(header: &str) -> Stanza {
    match header {
        "Term" => Stanza::Term(Term::new(String::new())),
        "Typedef" => Stanza::Typedef(TypeDef::default()),
        _ => Stanza::Other,
    }
}

fn flush_stanza(ontology: &mut Ontology, stanza: Stanza) {
    match stanza {
        Stanza::Term(term) if !term.id.is_empty() => {
            ontology.terms.insert(term.id.clone(), term);
        }
        Stanza::Typedef(typedef) if !typedef.id.is_empty() => {
            ontology.typedefs.insert(typedef.id.clone(), typedef);
        }
        _ => {}
    }
}

/// If `line` is a stanza header like `[Term]`, return the name inside the brackets.
fn parse_stanza_header(line: &str) -> Option<&str> {
    let line = line.trim();
    line.strip_prefix('[').and_then(|s| s.strip_suffix(']'))
}

/// Split a `tag: value` line into its tag and raw (unprocessed) value.
fn split_tag_value(line: &str) -> Option<(&str, &str)> {
    let idx = line.find(':')?;
    let tag = line[..idx].trim();
    let value = line[idx + 1..].trim();
    Some((tag, value))
}

/// Strip a trailing `! comment`, ignoring any `!` that appears inside a quoted string.
fn strip_trailing_comment(value: &str) -> &str {
    let mut in_quotes = false;
    let mut chars = value.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        match c {
            '"' => in_quotes = !in_quotes,
            '\\' if in_quotes => {
                // Skip an escaped character so `\"` doesn't toggle quote state.
                chars.next();
            }
            '!' if !in_quotes => return value[..i].trim_end(),
            _ => {}
        }
    }
    value.trim_end()
}

/// Strip a trailing `{...}` qualifier block (e.g. `{all_only="true"}`), if present.
fn strip_trailing_braces(value: &str) -> &str {
    let trimmed = value.trim_end();
    if trimmed.ends_with('}')
        && let Some(start) = trimmed.rfind('{')
    {
        return trimmed[..start].trim_end();
    }
    trimmed
}

/// Extract a leading quoted string's contents, returning (text, remainder-after-closing-quote).
fn take_quoted(value: &str) -> Option<(&str, &str)> {
    let value = value.trim_start();
    let rest = value.strip_prefix('"')?;
    let mut end = None;
    let mut escaped = false;
    for (i, c) in rest.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match c {
            '\\' => escaped = true,
            '"' => {
                end = Some(i);
                break;
            }
            _ => {}
        }
    }
    let end = end?;
    Some((&rest[..end], rest[end + 1..].trim_start()))
}

/// Extract xrefs from a trailing `[xref1, xref2]` list, if present.
fn take_xref_list(value: &str) -> Vec<String> {
    let value = value.trim();
    let Some(inner) = value.strip_prefix('[').and_then(|s| s.strip_suffix(']')) else {
        return Vec::new();
    };
    inner
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

fn apply_header_tag(ontology: &mut Ontology, tag: &str, value: &str) {
    match tag {
        "format-version" => ontology.format_version = Some(value.to_string()),
        "data-version" => ontology.data_version = Some(value.to_string()),
        "ontology" => ontology.ontology_id = Some(value.to_string()),
        _ => {}
    }
}

fn apply_term_tag(term: &mut Term, tag: &str, value: &str, line_no: usize) -> Result<()> {
    match tag {
        "id" => term.id = value.to_string(),
        "name" => term.name = Some(value.to_string()),
        "is_a" => term.is_a.push(value.to_string()),
        "disjoint_from" => term.disjoint_from.push(value.to_string()),
        "is_obsolete" => term.is_obsolete = value == "true",
        "subset" => term.subsets.push(value.to_string()),
        "xref" => term.xrefs.push(strip_trailing_braces(value).to_string()),
        "def" => {
            let Some((text, remainder)) = take_quoted(value) else {
                return Err(OntoparseError::MalformedLine {
                    line: line_no,
                    message: "def: value must start with a quoted string".to_string(),
                });
            };
            term.definition = Some(Definition {
                text: text.to_string(),
                xrefs: take_xref_list(remainder),
            });
        }
        "synonym" => {
            let Some((text, remainder)) = take_quoted(value) else {
                return Err(OntoparseError::MalformedLine {
                    line: line_no,
                    message: "synonym: value must start with a quoted string".to_string(),
                });
            };
            let mut parts = remainder.split_whitespace();
            let scope = match parts.next() {
                Some("BROAD") => SynonymScope::Broad,
                Some("NARROW") => SynonymScope::Narrow,
                Some("RELATED") => SynonymScope::Related,
                _ => SynonymScope::Exact,
            };
            term.synonyms.push(Synonym {
                text: text.to_string(),
                scope,
                xrefs: take_xref_list(remainder),
            });
        }
        "relationship" => {
            let value = strip_trailing_braces(value);
            let mut parts = value.splitn(2, char::is_whitespace);
            let predicate = parts.next().unwrap_or_default().trim();
            let target = parts.next().unwrap_or_default().trim();
            if predicate.is_empty() || target.is_empty() {
                return Err(OntoparseError::MalformedLine {
                    line: line_no,
                    message: "relationship: expected `<predicate> <target>`".to_string(),
                });
            }
            term.relationships.push(Relationship {
                predicate: predicate.to_string(),
                target: target.to_string(),
            });
        }
        _ => {}
    }
    Ok(())
}

fn apply_typedef_tag(typedef: &mut TypeDef, tag: &str, value: &str) {
    match tag {
        "id" => typedef.id = value.to_string(),
        "name" => typedef.name = Some(value.to_string()),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn parse(input: &str) -> Ontology {
        parse_obo_reader(BufReader::new(Cursor::new(input.to_string()))).unwrap()
    }

    #[test]
    fn parses_header() {
        let onto = parse("format-version: 1.2\ndata-version: releases/2026-04-01\nontology: uberon\n");
        assert_eq!(onto.format_version.as_deref(), Some("1.2"));
        assert_eq!(onto.data_version.as_deref(), Some("releases/2026-04-01"));
        assert_eq!(onto.ontology_id.as_deref(), Some("uberon"));
    }

    #[test]
    fn parses_simple_term() {
        let onto = parse(
            "[Term]\nid: BFO:0000001\nname: entity\n\n[Term]\nid: BFO:0000002\nname: continuant\nis_a: BFO:0000001 ! entity\n",
        );
        assert_eq!(onto.terms.len(), 2);
        let root = &onto.terms["BFO:0000001"];
        assert_eq!(root.name.as_deref(), Some("entity"));
        assert!(root.is_a.is_empty());

        let child = &onto.terms["BFO:0000002"];
        assert_eq!(child.is_a, vec!["BFO:0000001".to_string()]);
    }

    #[test]
    fn strips_bang_comment_outside_quotes() {
        assert_eq!(strip_trailing_comment("BFO:0000001 ! entity"), "BFO:0000001");
        assert_eq!(strip_trailing_comment("BFO:0000001"), "BFO:0000001");
    }

    #[test]
    fn does_not_strip_bang_inside_quotes() {
        let value = strip_trailing_comment(r#""a! b" [BSPO:cjm]"#);
        assert_eq!(value, r#""a! b" [BSPO:cjm]"#);
    }

    #[test]
    fn strips_trailing_qualifier_braces() {
        assert_eq!(
            strip_trailing_braces(r#"part_of BFO:0000002 {all_only="true"}"#),
            "part_of BFO:0000002"
        );
        assert_eq!(strip_trailing_braces("part_of BFO:0000002"), "part_of BFO:0000002");
    }

    #[test]
    fn parses_relationship_with_qualifier_and_comment() {
        let onto = parse(
            "[Term]\nid: BFO:0000002\nrelationship: part_of BFO:0000002 {all_only=\"true\"} ! continuant\n",
        );
        let term = &onto.terms["BFO:0000002"];
        assert_eq!(term.relationships.len(), 1);
        assert_eq!(term.relationships[0].predicate, "part_of");
        assert_eq!(term.relationships[0].target, "BFO:0000002");
    }

    #[test]
    fn parses_def_with_xrefs() {
        let onto = parse(
            "[Term]\nid: BSPO:0000000\ndef: \"The side of an organism that is left.\" [BSPO:cjm, BSPO:wd]\n",
        );
        let term = &onto.terms["BSPO:0000000"];
        let def = term.definition.as_ref().unwrap();
        assert_eq!(def.text, "The side of an organism that is left.");
        assert_eq!(def.xrefs, vec!["BSPO:cjm".to_string(), "BSPO:wd".to_string()]);
    }

    #[test]
    fn parses_synonym_scope_and_xrefs() {
        let onto = parse("[Term]\nid: X:1\nsynonym: \"left\" EXACT []\n");
        let term = &onto.terms["X:1"];
        assert_eq!(term.synonyms.len(), 1);
        assert_eq!(term.synonyms[0].text, "left");
        assert_eq!(term.synonyms[0].scope, SynonymScope::Exact);
    }

    #[test]
    fn parses_is_obsolete() {
        let onto = parse("[Term]\nid: X:1\nis_obsolete: true\n");
        assert!(onto.terms["X:1"].is_obsolete);
    }

    #[test]
    fn accumulates_multiple_values() {
        let onto = parse(
            "[Term]\nid: X:1\nxref: A:1\nxref: A:2\nsynonym: \"a\" EXACT []\nsynonym: \"b\" RELATED []\n",
        );
        let term = &onto.terms["X:1"];
        assert_eq!(term.xrefs, vec!["A:1".to_string(), "A:2".to_string()]);
        assert_eq!(term.synonyms.len(), 2);
    }

    #[test]
    fn strips_qualifier_from_xref() {
        let onto = parse(
            "[Term]\nid: UBERON:0000002\nxref: UMLS:C0007874 {source=\"ncithesaurus:Cervix\"}\n",
        );
        let term = &onto.terms["UBERON:0000002"];
        assert_eq!(term.xrefs, vec!["UMLS:C0007874".to_string()]);
    }

    #[test]
    fn skips_typedef_and_unknown_stanzas() {
        let onto = parse(
            "[Typedef]\nid: part_of\nname: part of\n\n[Term]\nid: X:1\nname: thing\n",
        );
        assert_eq!(onto.typedefs.len(), 1);
        assert_eq!(onto.typedefs["part_of"].name.as_deref(), Some("part of"));
        assert_eq!(onto.terms.len(), 1);
    }
}
