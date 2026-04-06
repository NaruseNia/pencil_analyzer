use std::collections::HashSet;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use pencil_analyzer::output::OutputOptions;
use pencil_analyzer::{extract, output, parse, resolve};

const TYPES: &[(&str, &str)] = &[
    ("rectangle", "Rectangle shape"),
    ("frame", "Frame container (supports layout)"),
    ("text", "Text node"),
    ("ellipse", "Ellipse / circle / arc"),
    ("line", "Line"),
    ("polygon", "Polygon"),
    ("path", "Vector path"),
    ("group", "Group container (supports layout)"),
    ("note", "Design note"),
    ("prompt", "AI prompt node"),
    ("context", "Context node"),
    ("icon_font", "Icon font glyph"),
    ("ref", "Component reference instance"),
];

const FILTERS: &[(&str, &str)] = &[
    ("content", "Text content"),
    ("fill", "Fill (color, gradient, image)"),
    ("layout", "Layout mode (horizontal/vertical)"),
    ("size", "Width and height"),
    ("position", "X/Y coordinates"),
    ("reusable", "Reusable component flag"),
    ("descendants", "Ref descendant overrides"),
    ("themes", "Document theme definitions"),
    ("variables", "Document variable definitions"),
    ("imports", "Import declarations"),
];

const EXTRACTS: &[(&str, &str)] = &[
    ("components", "Reusable nodes (reusable: true)"),
    ("variables", "Variable definitions"),
    ("imports", "Import declarations"),
    ("themes", "Theme definitions"),
];

#[derive(Parser)]
#[command(name = "pencil_analyzer", about = "Parse and analyze Pencil .pen files")]
struct Cli {
    /// Path to .pen file
    file: Option<PathBuf>,

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

    /// Filter nodes by type (e.g., "text,frame,rectangle")
    #[arg(long = "type", value_delimiter = ',')]
    node_type: Option<Vec<String>>,

    /// List available values: types, filters, extracts
    #[arg(long, value_parser = ["types", "filters", "extracts"])]
    list: Option<String>,

    /// Show only the node hierarchy (type, id, name) without any properties
    #[arg(long)]
    only_structure: bool,

    /// Limit tree depth (1 = top-level children only)
    #[arg(long)]
    depth: Option<usize>,

    /// Filter nodes by regex against their path (e.g., "Components/.*", ".*Button.*")
    #[arg(long = "regex")]
    regex_pattern: Option<String>,
}

fn print_list(title: &str, items: &[(&str, &str)]) {
    println!("{title}:");
    let max_name = items.iter().map(|(n, _)| n.len()).max().unwrap_or(0);
    for (name, desc) in items {
        println!("  {name:<max_name$}  {desc}");
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(ref category) = cli.list {
        match category.as_str() {
            "types" => print_list("Available --type values", TYPES),
            "filters" => print_list("Available --filter values", FILTERS),
            "extracts" => print_list("Available --extract values", EXTRACTS),
            _ => unreachable!(),
        }
        return Ok(());
    }

    let file = cli.file.as_ref().ok_or_else(|| {
        anyhow::anyhow!("A .pen file path is required (or use --list to show available values)")
    })?;

    let mut doc = parse::parse_document(file)?;

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

    if let Some(types) = cli.node_type {
        let type_set: HashSet<String> = types.into_iter().collect();
        doc = extract::filter_by_type(&doc, &type_set);
    }

    if let Some(ref pattern) = cli.regex_pattern {
        let re = regex::Regex::new(pattern)?;
        doc = extract::filter_by_regex(&doc, &re);
    }

    let opts = OutputOptions {
        filter: if cli.only_structure {
            Some(HashSet::new())
        } else {
            cli.filter.map(|fields| fields.into_iter().collect())
        },
        max_depth: cli.depth,
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
