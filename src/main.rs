mod error;
mod model;
mod output;
mod parse;
mod resolve;

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
    let mut doc = parse::parse_document(&cli.file)?;

    if cli.resolve_refs {
        doc = resolve::refs::resolve_refs(&doc)?;
    }

    if cli.resolve_vars {
        let theme = match &cli.theme {
            Some(s) => resolve::variables::parse_theme_string(s),
            None => resolve::variables::default_theme(&doc),
        };
        doc = resolve::variables::resolve_variables(&doc, &theme)?;
    }

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
