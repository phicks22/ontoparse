use std::path::PathBuf;

use ontoparse_core::idmap::build_id_map;
use ontoparse_core::parser::parse_obo;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/uberon_ext.obo")
}

#[test]
fn maps_uberon_to_fma_finds_known_pair() {
    let ontology = parse_obo(&fixture_path()).expect("fixture should parse");
    let rows = build_id_map(&ontology, "FMA").expect("anchor prefix should be inferable");

    assert!(!rows.is_empty(), "expected non-empty UBERON -> FMA mapping");

    let found = rows
        .iter()
        .find(|r| r.anchor_id == "UBERON:0000002" && r.target_xref == "FMA:17740")
        .expect("expected UBERON:0000002 -> FMA:17740 in the mapping");
    assert_eq!(found.anchor_name.as_deref(), Some("uterine cervix"));

    assert!(
        rows.iter().all(|r| r.anchor_id.starts_with("UBERON:")),
        "every mapped anchor id must belong to the UBERON anchor ontology"
    );
}
