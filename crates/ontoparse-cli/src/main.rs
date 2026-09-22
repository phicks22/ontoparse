use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};
use ontoparse_core::OntoparseError;
use ontoparse_core::idmap::build_id_map;
use ontoparse_core::loader::Loader;
use ontoparse_load::loader::OboLoader;

/// Extract an ID map from an OBO file's anchoring ontology to a target ontology via xrefs.
#[derive(Parser, Debug)]
#[command(
    name = "ontoparse",
    version,
    about = "Utils for parsing ontology files."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    // Extract a xref ID map from one ontology to another.
    Xref(XrefArgs),
}

#[derive(Args, Debug)]
struct XrefArgs {
    // Path to the OBO file to parse.
    file: PathBuf,

    // Target
    #[arg(long, value_name = "PREFIX")]
    to: String,

    // Source ontology prefix, (e.g., UBERON).
    #[arg(long, value_name = "PREFIX")]
    from: Option<String>,

    // Delimiter of output table
    #[arg(long, value_name = "SEPARATOR", default_value_t = "\t".to_string())]
    separator: String,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match &cli.command {
        Command::Xref(args) => run_xref(args),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run_xref(args: &XrefArgs) -> Result<(), OntoparseError> {
    let ontology = OboLoader.load(&args.file)?;
    let rows = build_id_map(&ontology, &args.to, args.from.as_deref())?;

    // write to console
    println!("anchor{}name{}target", &args.separator, &args.separator,);

    for row in &rows {
        println!(
            "{}{}{}{}{}",
            row.anchor_id,
            &args.separator,
            row.anchor_name.as_deref().unwrap_or(""),
            &args.separator,
            row.target_xref
        );
    }

    if rows.is_empty() {
        eprintln!("Warning: no xrefs found in {}", args.file.display());
    }

    Ok(())
}
