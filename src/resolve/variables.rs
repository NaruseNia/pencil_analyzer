use std::collections::HashMap;

use anyhow::{bail, Result};

use crate::model::common::{OrVariable, Theme};
use crate::model::document::{Document, ThemedValue, VariableDef, VariableValue};

/// Resolved variable values: name -> JSON value.
type ResolvedVars = HashMap<String, serde_json::Value>;

/// Resolve all `$variable` references in the document.
/// Uses the given theme to select themed variable values.
/// If no theme is provided, defaults are used (first value of each axis).
pub fn resolve_variables(doc: &Document, theme: &Theme) -> Result<Document> {
    let resolved_vars = resolve_var_definitions(doc, theme)?;

    let mut result = doc.clone();
    result.children = result
        .children
        .into_iter()
        .map(|child| resolve_child_vars(child, &resolved_vars))
        .collect::<Result<_>>()?;
    Ok(result)
}

/// Parse a theme string like "mode=dark,spacing=condensed" into a Theme map.
pub fn parse_theme_string(s: &str) -> Theme {
    let mut theme = Theme::new();
    for part in s.split(',') {
        let part = part.trim();
        if let Some((key, value)) = part.split_once('=') {
            theme.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    theme
}

/// Build default theme from document's theme definitions (first value of each axis).
pub fn default_theme(doc: &Document) -> Theme {
    let mut theme = Theme::new();
    if let Some(themes) = &doc.themes {
        for (axis, values) in themes {
            if let Some(first) = values.first() {
                theme.insert(axis.clone(), first.clone());
            }
        }
    }
    theme
}

/// Resolve all variable definitions to concrete JSON values.
fn resolve_var_definitions(doc: &Document, theme: &Theme) -> Result<ResolvedVars> {
    let mut resolved = ResolvedVars::new();

    let Some(variables) = &doc.variables else {
        return Ok(resolved);
    };

    for (name, def) in variables {
        let value = resolve_var_def(def, theme)?;
        resolved.insert(name.clone(), value);
    }

    // Second pass: resolve variable references within variable values
    let snapshot = resolved.clone();
    for (name, value) in &mut resolved {
        *value = substitute_vars_in_json(value.clone(), &snapshot, name)?;
    }

    Ok(resolved)
}

fn resolve_var_def(def: &VariableDef, theme: &Theme) -> Result<serde_json::Value> {
    match def {
        VariableDef::Boolean { value } => resolve_typed_value(value, theme),
        VariableDef::Color { value } => resolve_typed_value(value, theme),
        VariableDef::Number { value } => resolve_typed_value(value, theme),
        VariableDef::String { value } => resolve_typed_value(value, theme),
    }
}

fn resolve_typed_value<T: serde::Serialize + Clone>(
    value: &VariableValue<OrVariable<T>>,
    theme: &Theme,
) -> Result<serde_json::Value> {
    match value {
        VariableValue::Single(v) => match v {
            OrVariable::Value(val) => Ok(serde_json::to_value(val)?),
            OrVariable::Variable(var_ref) => {
                // Variable referencing another variable — return as string for second pass
                Ok(serde_json::Value::String(var_ref.clone()))
            }
        },
        VariableValue::Themed(entries) => resolve_themed(entries, theme),
    }
}

/// Evaluate themed values: the last entry whose theme is satisfied wins.
fn resolve_themed<T: serde::Serialize + Clone>(
    entries: &[ThemedValue<OrVariable<T>>],
    theme: &Theme,
) -> Result<serde_json::Value> {
    let mut result = None;
    for entry in entries {
        if theme_matches(&entry.theme, theme) {
            result = Some(&entry.value);
        }
    }
    match result {
        Some(OrVariable::Value(val)) => Ok(serde_json::to_value(val)?),
        Some(OrVariable::Variable(var_ref)) => Ok(serde_json::Value::String(var_ref.clone())),
        None => bail!("No themed value matched for the given theme"),
    }
}

fn theme_matches(entry_theme: &Option<Theme>, active_theme: &Theme) -> bool {
    let Some(entry_theme) = entry_theme else {
        return true; // No theme constraint = always matches
    };
    entry_theme
        .iter()
        .all(|(axis, value)| active_theme.get(axis).map_or(false, |v| v == value))
}

/// Substitute $variable references in a JSON value tree.
fn substitute_vars_in_json(
    value: serde_json::Value,
    vars: &ResolvedVars,
    _context: &str,
) -> Result<serde_json::Value> {
    match value {
        serde_json::Value::String(ref s) if s.starts_with('$') => {
            let var_name = &s[1..];
            match vars.get(var_name) {
                Some(resolved) => Ok(resolved.clone()),
                None => Ok(value), // Leave unresolved references as-is
            }
        }
        serde_json::Value::Array(arr) => {
            let resolved: Result<Vec<_>> = arr
                .into_iter()
                .map(|v| substitute_vars_in_json(v, vars, _context))
                .collect();
            Ok(serde_json::Value::Array(resolved?))
        }
        serde_json::Value::Object(map) => {
            let resolved: Result<serde_json::Map<String, serde_json::Value>> = map
                .into_iter()
                .map(|(k, v)| Ok((k.clone(), substitute_vars_in_json(v, vars, &k)?)))
                .collect();
            Ok(serde_json::Value::Object(resolved?))
        }
        other => Ok(other),
    }
}

/// Resolve variable references within a Child node by going through JSON.
fn resolve_child_vars(
    child: crate::model::objects::Child,
    vars: &ResolvedVars,
) -> Result<crate::model::objects::Child> {
    let json = serde_json::to_value(&child)?;
    let resolved = substitute_vars_in_json(json, vars, "root")?;
    Ok(serde_json::from_value(resolved)?)
}
