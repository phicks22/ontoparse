use std::path::PathBuf;

use assert_cmd::Command;
use predicates::str::contains;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/uberon_ext.obo")
}

#[test]
fn positional_and_flag_forms_are_equivalent() {
    let fixture = fixture_path();

    let positional = Command::cargo_bin("ontoparse")
        .unwrap()
        .arg(&fixture)
        .arg("FMA")
        .output()
        .unwrap();
    let flagged = Command::cargo_bin("ontoparse")
        .unwrap()
        .arg(&fixture)
        .arg("--to")
        .arg("FMA")
        .output()
        .unwrap();

    assert!(positional.status.success());
    assert!(flagged.status.success());
    assert_eq!(positional.stdout, flagged.stdout);
}

#[test]
fn both_positional_and_flag_given_is_an_error() {
    Command::cargo_bin("ontoparse")
        .unwrap()
        .arg(fixture_path())
        .arg("FMA")
        .arg("--to")
        .arg("FMA")
        .assert()
        .failure();
}

#[test]
fn missing_target_is_an_error() {
    Command::cargo_bin("ontoparse")
        .unwrap()
        .arg(fixture_path())
        .assert()
        .failure();
}

#[test]
fn unmatched_target_prefix_prints_warning_and_empty_body() {
    Command::cargo_bin("ontoparse")
        .unwrap()
        .arg(fixture_path())
        .arg("--to")
        .arg("ZZZZZNOPE")
        .assert()
        .success()
        .stdout("anchor_id\tanchor_name\ttarget_xref\n")
        .stderr(contains("warning"));
}

#[test]
fn output_is_well_formed_tsv_with_known_row() {
    Command::cargo_bin("ontoparse")
        .unwrap()
        .arg(fixture_path())
        .arg("--to")
        .arg("FMA")
        .assert()
        .success()
        .stdout(contains("UBERON:0000002\tuterine cervix\tFMA:17740"));
}
