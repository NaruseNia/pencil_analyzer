mod error;
mod model;
mod output;
mod parse;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "pencil_analyzer", about = "Parse and analyze Pencil .pen files")]
struct Cli {
    /// Path to .pen file
    file: PathBuf,

    /// Output format
    #[arg(long, default_value = "text", value_parser = ["json", "text"])]
    format: String,

    /// Resolve ref instances to their full definitions
    #[arg(long)]
    resolve_refs: bool,

    /// Resolve variable references (uses default theme unless --theme is specified)
    #[arg(long)]
    resolve_vars: bool,

    /// Theme selection (e.g., "mode=dark,spacing=condensed")
    #[arg(long)]
    theme: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let doc = parse::parse_document(&cli.file)?;

    match cli.format.as_str() {
        "json" => {
            let out = output::json::format(&doc)?;
            println!("{out}");
        }
        "text" => {
            let out = output::text::format(&doc);
            println!("{out}");
        }
        _ => unreachable!(),
    }

    Ok(())
}
