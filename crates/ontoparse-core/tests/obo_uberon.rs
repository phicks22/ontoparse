use std::path::PathBuf;

use ontoparse_core::parser::parse_obo;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../data/uberon_ext.obo")
}

#[test]
fn parses_full_uberon_fixture() {
    let path = fixture_path();
    let ontology = parse_obo(&path).expect("uberon_ext.obo should parse without error");

    assert_eq!(ontology.format_version.as_deref(), Some("1.2"));

    // Real UBERON fixture has ~26,959 `[Term]` stanzas at time of writing;
    // allow some slack in case the fixture is refreshed.
    assert!(
        ontology.terms.len() > 26_000,
        "expected roughly 26,959 terms, got {}",
        ontology.terms.len()
    );

    let root = ontology
        .terms
        .get("BFO:0000001")
        .expect("BFO:0000001 (entity) should be present");
    assert_eq!(root.name.as_deref(), Some("entity"));
    assert!(root.is_a.is_empty(), "root term should have no is_a parents");

    let continuant = ontology
        .terms
        .get("BFO:0000002")
        .expect("BFO:0000002 (continuant) should be present");
    assert_eq!(continuant.name.as_deref(), Some("continuant"));
    assert!(continuant.is_a.contains(&"BFO:0000001".to_string()));
}
