use std::collections::HashSet;

use crate::model::common::TextContent;
use crate::model::document::{Document, VariableDef};
use crate::model::graphics::Fills;
use crate::model::layout::Dimension;
use crate::model::objects::Child;

pub fn format(doc: &Document, filter: Option<&HashSet<String>>) -> String {
    let mut out = String::new();
    out.push_str(&format!("Document (version {})\n", doc.version));

    if allowed(filter, "themes") {
        if let Some(themes) = &doc.themes {
            let entries: Vec<String> = themes
                .iter()
                .map(|(k, v)| format!("{k}[{}]", v.join(", ")))
                .collect();
            out.push_str(&format!("  Themes: {}\n", entries.join(", ")));
        }
    }

    if allowed(filter, "variables") {
        if let Some(vars) = &doc.variables {
            out.push_str(&format!("  Variables: {} defined\n", vars.len()));
            for (name, def) in vars {
                let type_str = match def {
                    VariableDef::Boolean { .. } => "boolean",
                    VariableDef::Color { .. } => "color",
                    VariableDef::Number { .. } => "number",
                    VariableDef::String { .. } => "string",
                };
                out.push_str(&format!("    ${name}: {type_str}\n"));
            }
        }
    }

    if allowed(filter, "imports") {
        if let Some(imports) = &doc.imports {
            out.push_str(&format!("  Imports: {} files\n", imports.len()));
            for (alias, path) in imports {
                out.push_str(&format!("    {alias} -> {path}\n"));
            }
        }
    }

    out.push('\n');

    for child in &doc.children {
        format_child(&mut out, child, 1, filter);
    }

    out
}

/// Returns true if the field should be included in output.
fn allowed(filter: Option<&HashSet<String>>, field: &str) -> bool {
    match filter {
        None => true,
        Some(fields) => fields.contains(field),
    }
}

fn format_child(out: &mut String, child: &Child, depth: usize, filter: Option<&HashSet<String>>) {
    let indent = "  ".repeat(depth);
    let id = child.id();
    let type_name = child.type_name();
    let entity = child.entity();

    let name_suffix = entity
        .name
        .as_deref()
        .map(|n| format!(" \"{n}\""))
        .unwrap_or_default();

    let size_str = if allowed(filter, "size") {
        format_size(child)
    } else {
        String::new()
    };
    let pos_str = if allowed(filter, "position") {
        match (entity.x, entity.y) {
            (Some(x), Some(y)) => format!(" @({x},{y})"),
            _ => String::new(),
        }
    } else {
        String::new()
    };

    match child {
        Child::Ref(r) => {
            out.push_str(&format!(
                "{indent}[{type_name}] {id}{name_suffix} -> {}{pos_str}\n",
                r.ref_id
            ));
            if allowed(filter, "descendants") {
                if let Some(desc) = &r.descendants {
                    for (path, _) in desc {
                        out.push_str(&format!("{indent}  override: {path}\n"));
                    }
                }
            }
        }
        _ => {
            out.push_str(&format!(
                "{indent}[{type_name}] {id}{name_suffix}{size_str}{pos_str}\n"
            ));
        }
    }

    if allowed(filter, "reusable") && entity.reusable == Some(true) {
        out.push_str(&format!("{indent}  reusable: true\n"));
    }

    if allowed(filter, "fill") {
        format_fill(out, child, &indent);
    }
    if allowed(filter, "content") {
        format_text_content(out, child, &indent);
    }
    if allowed(filter, "layout") {
        format_layout_info(out, child, &indent);
    }

    if let Some(children) = child.children() {
        for c in children {
            format_child(out, c, depth + 1, filter);
        }
    }
}

fn format_size(child: &Child) -> String {
    let (w, h) = match child {
        Child::Rectangle(n) => (&n.width, &n.height),
        Child::Frame(n) => (&n.width, &n.height),
        Child::Text(n) => (&n.width, &n.height),
        Child::Ellipse(n) => (&n.width, &n.height),
        Child::Line(n) => (&n.width, &n.height),
        Child::Polygon(n) => (&n.width, &n.height),
        Child::Path(n) => (&n.width, &n.height),
        Child::Group(n) => (&n.width, &n.height),
        Child::Note(n) => (&n.width, &n.height),
        Child::Prompt(n) => (&n.width, &n.height),
        Child::Context(n) => (&n.width, &n.height),
        Child::IconFont(n) => (&n.width, &n.height),
        Child::Ref(_) => return String::new(),
    };

    match (w, h) {
        (Some(w), Some(h)) => format!(" ({}x{})", dim_str(w), dim_str(h)),
        (Some(w), None) => format!(" (w:{})", dim_str(w)),
        (None, Some(h)) => format!(" (h:{})", dim_str(h)),
        (None, None) => String::new(),
    }
}

fn dim_str(d: &Dimension) -> String {
    match d {
        Dimension::Number(n) => format!("{n}"),
        Dimension::Variable(v) => v.clone(),
    }
}

fn format_fill(out: &mut String, child: &Child, indent: &str) {
    let fills = match child {
        Child::Rectangle(n) => &n.fill,
        Child::Frame(n) => &n.fill,
        Child::Text(n) => &n.fill,
        Child::Ellipse(n) => &n.fill,
        Child::Line(n) => &n.fill,
        Child::Polygon(n) => &n.fill,
        Child::Path(n) => &n.fill,
        Child::IconFont(n) => &n.fill,
        _ => return,
    };

    if let Some(fills) = fills {
        let desc = match fills {
            Fills::Single(f) => format!("{f:?}"),
            Fills::Multiple(fs) => format!("{} fills", fs.len()),
        };
        out.push_str(&format!("{indent}  fill: {desc}\n"));
    }
}

fn format_text_content(out: &mut String, child: &Child, indent: &str) {
    let content = match child {
        Child::Text(n) => &n.content,
        Child::Note(n) => &n.content,
        Child::Prompt(n) => &n.content,
        Child::Context(n) => &n.content,
        _ => return,
    };

    if let Some(content) = content {
        match content {
            TextContent::Plain(s) => {
                out.push_str(&format!("{indent}  content: {s}\n"));
            }
            TextContent::Styled(segments) => {
                out.push_str(&format!("{indent}  content: {} styled segments\n", segments.len()));
            }
        }
    }
}

fn format_layout_info(out: &mut String, child: &Child, indent: &str) {
    let layout = match child {
        Child::Frame(f) => &f.layout,
        Child::Group(g) => &g.layout,
        _ => return,
    };

    if let Some(layout) = layout {
        out.push_str(&format!("{indent}  layout: {layout:?}\n"));
    }
}
