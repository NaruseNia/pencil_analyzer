use std::collections::{HashMap, HashSet};

use pencil_analyzer::extract::extract_document;
use pencil_analyzer::model::common::*;
use pencil_analyzer::model::document::*;
use pencil_analyzer::model::objects::*;

fn sample_doc() -> Document {
    Document {
        version: "2.9".into(),
        themes: Some(HashMap::from([
            ("mode".into(), vec!["light".into(), "dark".into()]),
        ])),
        imports: Some(HashMap::from([
            ("icons".into(), "./icons.pen".into()),
        ])),
        variables: Some(HashMap::from([
            (
                "color.bg".into(),
                VariableDef::Color {
                    value: VariableValue::Single(OrVariable::Value("#FFF".into())),
                },
            ),
        ])),
        children: vec![
            // A reusable component
            Child::Frame(Frame {
                entity: Entity {
                    id: "btn".into(),
                    name: Some("Button".into()),
                    reusable: Some(true),
                    context: None,
                    theme: None,
                    enabled: None,
                    opacity: None,
                    flip_x: None,
                    flip_y: None,
                    layout_position: None,
                    metadata: None,
                    x: None,
                    y: None,
                    rotation: None,
                },
                width: None,
                height: None,
                corner_radius: None,
                fill: None,
                stroke: None,
                effect: None,
                children: Some(vec![Child::Text(Text {
                    entity: Entity {
                        id: "label".into(),
                        name: Some("Label".into()),
                        reusable: None,
                        context: None,
                        theme: None,
                        enabled: None,
                        opacity: None,
                        flip_x: None,
                        flip_y: None,
                        layout_position: None,
                        metadata: None,
                        x: None,
                        y: None,
                        rotation: None,
                    },
                    width: None,
                    height: None,
                    content: Some(TextContent::Plain(OrVariable::Value("Click".into()))),
                    text_growth: None,
                    fill: None,
                    stroke: None,
                    effect: None,
                    style: TextStyle {
                        font_family: None,
                        font_size: None,
                        font_weight: None,
                        letter_spacing: None,
                        font_style: None,
                        underline: None,
                        line_height: None,
                        text_align: None,
                        text_align_vertical: None,
                        strikethrough: None,
                        href: None,
                    },
                })]),
                clip: None,
                placeholder: None,
                slot: None,
                layout: None,
                gap: None,
                layout_include_stroke: None,
                padding: None,
                justify_content: None,
                align_items: None,
            }),
            // A non-reusable frame containing a nested reusable
            Child::Frame(Frame {
                entity: Entity {
                    id: "page".into(),
                    name: Some("Page".into()),
                    reusable: None,
                    context: None,
                    theme: None,
                    enabled: None,
                    opacity: None,
                    flip_x: None,
                    flip_y: None,
                    layout_position: None,
                    metadata: None,
                    x: None,
                    y: None,
                    rotation: None,
                },
                width: None,
                height: None,
                corner_radius: None,
                fill: None,
                stroke: None,
                effect: None,
                children: Some(vec![Child::Frame(Frame {
                    entity: Entity {
                        id: "card".into(),
                        name: Some("Card".into()),
                        reusable: Some(true),
                        context: None,
                        theme: None,
                        enabled: None,
                        opacity: None,
                        flip_x: None,
                        flip_y: None,
                        layout_position: None,
                        metadata: None,
                        x: None,
                        y: None,
                        rotation: None,
                    },
                    width: None,
                    height: None,
                    corner_radius: None,
                    fill: None,
                    stroke: None,
                    effect: None,
                    children: None,
                    clip: None,
                    placeholder: None,
                    slot: None,
                    layout: None,
                    gap: None,
                    layout_include_stroke: None,
                    padding: None,
                    justify_content: None,
                    align_items: None,
                })]),
                clip: None,
                placeholder: None,
                slot: None,
                layout: None,
                gap: None,
                layout_include_stroke: None,
                padding: None,
                justify_content: None,
                align_items: None,
            }),
        ],
    }
}

// ============================================================
// --extract components
// ============================================================

#[test]
fn extract_components_collects_reusable_nodes() {
    let doc = sample_doc();
    let cats: HashSet<String> = ["components".into()].into();
    let result = extract_document(&doc, &cats);

    assert_eq!(result.children.len(), 2);
    assert_eq!(result.children[0].id(), "btn");
    assert_eq!(result.children[1].id(), "card");
    // Non-extract categories should be cleared
    assert!(result.variables.is_none());
    assert!(result.themes.is_none());
    assert!(result.imports.is_none());
}

#[test]
fn extract_components_preserves_subtree() {
    let doc = sample_doc();
    let cats: HashSet<String> = ["components".into()].into();
    let result = extract_document(&doc, &cats);

    // btn should still have its child text node
    let btn_children = result.children[0].children().unwrap();
    assert_eq!(btn_children.len(), 1);
    assert_eq!(btn_children[0].id(), "label");
}

// ============================================================
// --extract variables
// ============================================================

#[test]
fn extract_variables_only() {
    let doc = sample_doc();
    let cats: HashSet<String> = ["variables".into()].into();
    let result = extract_document(&doc, &cats);

    assert!(result.variables.is_some());
    assert_eq!(result.variables.unwrap().len(), 1);
    assert!(result.children.is_empty());
    assert!(result.themes.is_none());
    assert!(result.imports.is_none());
}

// ============================================================
// --extract themes
// ============================================================

#[test]
fn extract_themes_only() {
    let doc = sample_doc();
    let cats: HashSet<String> = ["themes".into()].into();
    let result = extract_document(&doc, &cats);

    assert!(result.themes.is_some());
    assert!(result.children.is_empty());
    assert!(result.variables.is_none());
    assert!(result.imports.is_none());
}

// ============================================================
// --extract imports
// ============================================================

#[test]
fn extract_imports_only() {
    let doc = sample_doc();
    let cats: HashSet<String> = ["imports".into()].into();
    let result = extract_document(&doc, &cats);

    assert!(result.imports.is_some());
    assert_eq!(result.imports.unwrap().len(), 1);
    assert!(result.children.is_empty());
    assert!(result.variables.is_none());
    assert!(result.themes.is_none());
}

// ============================================================
// Multiple categories
// ============================================================

#[test]
fn extract_multiple_categories() {
    let doc = sample_doc();
    let cats: HashSet<String> = ["components".into(), "variables".into(), "themes".into()].into();
    let result = extract_document(&doc, &cats);

    assert_eq!(result.children.len(), 2);
    assert!(result.variables.is_some());
    assert!(result.themes.is_some());
    assert!(result.imports.is_none());
}

// ============================================================
// No matching components
// ============================================================

#[test]
fn extract_components_empty_when_none_reusable() {
    let doc = Document {
        version: "1.0".into(),
        themes: None,
        imports: None,
        variables: None,
        children: vec![Child::Rectangle(Rectangle {
            entity: Entity {
                id: "rect".into(),
                name: None,
                reusable: None,
                context: None,
                theme: None,
                enabled: None,
                opacity: None,
                flip_x: None,
                flip_y: None,
                layout_position: None,
                metadata: None,
                x: None,
                y: None,
                rotation: None,
            },
            width: None,
            height: None,
            corner_radius: None,
            fill: None,
            stroke: None,
            effect: None,
        })],
    };
    let cats: HashSet<String> = ["components".into()].into();
    let result = extract_document(&doc, &cats);

    assert!(result.children.is_empty());
}
