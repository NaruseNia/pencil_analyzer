use pencil_analyzer::model::common::*;
use pencil_analyzer::model::objects::*;
use pencil_analyzer::parse;
use pencil_analyzer::resolve::variables::{default_theme, parse_theme_string, resolve_variables};

#[test]
fn parse_theme_string_basic() {
    let theme = parse_theme_string("mode=dark,spacing=condensed");
    assert_eq!(theme.get("mode").unwrap(), "dark");
    assert_eq!(theme.get("spacing").unwrap(), "condensed");
}

#[test]
fn parse_theme_string_with_spaces() {
    let theme = parse_theme_string("mode = dark , spacing = condensed");
    assert_eq!(theme.get("mode").unwrap(), "dark");
    assert_eq!(theme.get("spacing").unwrap(), "condensed");
}

#[test]
fn default_theme_uses_first_values() {
    let json = r#"{
        "version": "2.9",
        "themes": { "mode": ["light", "dark"], "size": ["regular", "compact"] },
        "children": []
    }"#;
    let doc = parse::parse_from_str(json).unwrap();
    let theme = default_theme(&doc);
    assert_eq!(theme.get("mode").unwrap(), "light");
    assert_eq!(theme.get("size").unwrap(), "regular");
}

#[test]
fn resolve_simple_color_variable() {
    let json = r##"{
        "version": "2.9",
        "variables": {
            "color.bg": { "type": "color", "value": "#FFFFFF" }
        },
        "children": [
            {
                "type": "rectangle",
                "id": "box",
                "fill": "$color.bg"
            }
        ]
    }"##;
    let doc = parse::parse_from_str(json).unwrap();
    let resolved = resolve_variables(&doc, &Theme::new()).unwrap();

    match &resolved.children[0] {
        Child::Rectangle(r) => {
            let fill_json = serde_json::to_value(&r.fill).unwrap();
            assert!(fill_json.to_string().contains("#FFFFFF"));
        }
        other => panic!("Expected rectangle, got {}", other.type_name()),
    }
}

#[test]
fn resolve_number_variable() {
    let json = r#"{
        "version": "2.9",
        "variables": {
            "size.title": { "type": "number", "value": 72 }
        },
        "children": [
            {
                "type": "text",
                "id": "title",
                "content": "Hello",
                "fontSize": "$size.title"
            }
        ]
    }"#;
    let doc = parse::parse_from_str(json).unwrap();
    let resolved = resolve_variables(&doc, &Theme::new()).unwrap();

    match &resolved.children[0] {
        Child::Text(t) => {
            assert_eq!(t.style.font_size, Some(OrVariable::Value(72.0)));
        }
        other => panic!("Expected text, got {}", other.type_name()),
    }
}

#[test]
fn resolve_themed_variable_light() {
    let json = r##"{
        "version": "2.9",
        "themes": { "mode": ["light", "dark"] },
        "variables": {
            "color.bg": {
                "type": "color",
                "value": [
                    { "value": "#FFFFFF", "theme": { "mode": "light" } },
                    { "value": "#000000", "theme": { "mode": "dark" } }
                ]
            }
        },
        "children": [
            { "type": "rectangle", "id": "box", "fill": "$color.bg" }
        ]
    }"##;
    let doc = parse::parse_from_str(json).unwrap();

    // Light theme (default)
    let theme = parse_theme_string("mode=light");
    let resolved = resolve_variables(&doc, &theme).unwrap();
    match &resolved.children[0] {
        Child::Rectangle(r) => {
            let fill_json = serde_json::to_value(&r.fill).unwrap();
            assert!(fill_json.to_string().contains("#FFFFFF"));
        }
        _ => panic!("Expected rectangle"),
    }
}

#[test]
fn resolve_themed_variable_dark() {
    let json = r##"{
        "version": "2.9",
        "themes": { "mode": ["light", "dark"] },
        "variables": {
            "color.bg": {
                "type": "color",
                "value": [
                    { "value": "#FFFFFF", "theme": { "mode": "light" } },
                    { "value": "#000000", "theme": { "mode": "dark" } }
                ]
            }
        },
        "children": [
            { "type": "rectangle", "id": "box", "fill": "$color.bg" }
        ]
    }"##;
    let doc = parse::parse_from_str(json).unwrap();

    let theme = parse_theme_string("mode=dark");
    let resolved = resolve_variables(&doc, &theme).unwrap();
    match &resolved.children[0] {
        Child::Rectangle(r) => {
            let fill_json = serde_json::to_value(&r.fill).unwrap();
            assert!(fill_json.to_string().contains("#000000"));
        }
        _ => panic!("Expected rectangle"),
    }
}

#[test]
fn resolve_multiple_variables() {
    let json = r##"{
        "version": "2.9",
        "variables": {
            "color.bg": { "type": "color", "value": "#FFFFFF" },
            "color.text": { "type": "color", "value": "#333333" },
            "size.gap": { "type": "number", "value": 16 }
        },
        "children": [
            {
                "type": "frame",
                "id": "page",
                "fill": "$color.bg",
                "gap": "$size.gap",
                "children": [
                    {
                        "type": "text",
                        "id": "title",
                        "content": "Hello",
                        "fill": "$color.text"
                    }
                ]
            }
        ]
    }"##;
    let doc = parse::parse_from_str(json).unwrap();
    let resolved = resolve_variables(&doc, &Theme::new()).unwrap();

    match &resolved.children[0] {
        Child::Frame(f) => {
            // gap should be resolved
            assert_eq!(f.gap, Some(OrVariable::Value(16.0)));
            // fill should be resolved
            let fill_json = serde_json::to_value(&f.fill).unwrap();
            assert!(fill_json.to_string().contains("#FFFFFF"));
            // child text fill should be resolved
            match &f.children.as_ref().unwrap()[0] {
                Child::Text(t) => {
                    let fill_json = serde_json::to_value(&t.fill).unwrap();
                    assert!(fill_json.to_string().contains("#333333"));
                }
                _ => panic!("Expected text"),
            }
        }
        _ => panic!("Expected frame"),
    }
}

#[test]
fn resolve_unresolved_variable_kept_as_is() {
    let json = r#"{
        "version": "2.9",
        "children": [
            {
                "type": "rectangle",
                "id": "box",
                "fill": "$nonexistent.var"
            }
        ]
    }"#;
    let doc = parse::parse_from_str(json).unwrap();
    let resolved = resolve_variables(&doc, &Theme::new()).unwrap();

    // Should not error — unresolved vars are kept as-is
    match &resolved.children[0] {
        Child::Rectangle(r) => {
            let fill_json = serde_json::to_value(&r.fill).unwrap();
            assert!(fill_json.to_string().contains("$nonexistent.var"));
        }
        _ => panic!("Expected rectangle"),
    }
}

#[test]
fn resolve_boolean_variable() {
    let json = r#"{
        "version": "2.9",
        "variables": {
            "is.visible": { "type": "boolean", "value": false }
        },
        "children": [
            {
                "type": "rectangle",
                "id": "box",
                "enabled": "$is.visible"
            }
        ]
    }"#;
    let doc = parse::parse_from_str(json).unwrap();
    let resolved = resolve_variables(&doc, &Theme::new()).unwrap();

    match &resolved.children[0] {
        Child::Rectangle(r) => {
            assert_eq!(r.entity.enabled, Some(OrVariable::Value(false)));
        }
        _ => panic!("Expected rectangle"),
    }
}

#[test]
fn resolve_sample_fixture_default_theme() {
    let doc = parse::parse_document(std::path::Path::new("tests/fixtures/sample.pen")).unwrap();
    let theme = default_theme(&doc);
    let resolved = resolve_variables(&doc, &theme).unwrap();

    // landing-page fill should be resolved to #FFFFFF (light mode default)
    match &resolved.children[1] {
        Child::Frame(f) => {
            let fill_json = serde_json::to_value(&f.fill).unwrap();
            assert!(
                fill_json.to_string().contains("#FFFFFF"),
                "Expected #FFFFFF for light theme, got: {fill_json}"
            );
        }
        _ => panic!("Expected frame"),
    }
}

#[test]
fn resolve_sample_fixture_dark_theme() {
    let doc = parse::parse_document(std::path::Path::new("tests/fixtures/sample.pen")).unwrap();
    let theme = parse_theme_string("mode=dark");
    let resolved = resolve_variables(&doc, &theme).unwrap();

    // landing-page fill should be resolved to #000000 (dark mode)
    match &resolved.children[1] {
        Child::Frame(f) => {
            let fill_json = serde_json::to_value(&f.fill).unwrap();
            assert!(
                fill_json.to_string().contains("#000000"),
                "Expected #000000 for dark theme, got: {fill_json}"
            );
        }
        _ => panic!("Expected frame"),
    }
}
