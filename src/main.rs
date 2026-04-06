use std::collections::HashSet;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use pencil_analyzer::output::OutputOptions;
use pencil_analyzer::{extract, output, parse, resolve};

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

    /// Comma-separated list of fields to include in output (e.g., "content,fill,layout")
    #[arg(long, value_delimiter = ',')]
    filter: Option<Vec<String>>,

    /// Extract specific categories: components, variables, imports, themes
    #[arg(long, value_delimiter = ',')]
    extract: Option<Vec<String>>,
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

    let extract_set: Option<HashSet<String>> = cli
        .extract
        .map(|cats| cats.into_iter().collect());

    if let Some(ref cats) = extract_set {
        doc = extract::extract_document(&doc, cats);
    }

    let opts = OutputOptions {
        filter: cli.filter.map(|fields| fields.into_iter().collect()),
    };

    match cli.format.as_str() {
        "json" => {
            let out = output::json::format(&doc, &opts)?;
            println!("{out}");
        }
        "text" => {
            let out = output::text::format(&doc, &opts);
            println!("{out}");
        }
        _ => unreachable!(),
    }

    Ok(())
}
