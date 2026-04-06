use std::collections::HashMap;

use pencil_analyzer::model::common::*;
use pencil_analyzer::model::document::*;
use pencil_analyzer::model::graphics::*;
use pencil_analyzer::model::layout::*;
use pencil_analyzer::model::objects::*;
use pencil_analyzer::output::json;
use pencil_analyzer::output::text;
use pencil_analyzer::output::OutputOptions;

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
        children: vec![Child::Frame(Frame {
            entity: Entity {
                id: "frame1".into(),
                name: Some("Main".into()),
                reusable: Some(true),
                context: None,
                theme: None,
                enabled: None,
                opacity: None,
                flip_x: None,
                flip_y: None,
                layout_position: None,
                metadata: None,
                x: Some(10.0),
                y: Some(20.0),
                rotation: None,
            },
            width: Some(Dimension::Number(100.0)),
            height: Some(Dimension::Number(50.0)),
            corner_radius: None,
            fill: Some(Fills::Single(Fill::Color(OrVariable::Value("#FF0000".into())))),
            stroke: None,
            effect: None,
            children: Some(vec![Child::Text(Text {
                entity: Entity {
                    id: "txt1".into(),
                    name: Some("Title".into()),
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
                content: Some(TextContent::Plain(OrVariable::Value("Hello".into()))),
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
            layout: Some(LayoutMode::Horizontal),
            gap: None,
            layout_include_stroke: None,
            padding: None,
            justify_content: None,
            align_items: None,
        })],
    }
}

// ============================================================
// Text format: --filter
// ============================================================

#[test]
fn text_no_filter_includes_everything() {
    let doc = sample_doc();
    let opts = OutputOptions { filter: None };
    let out = text::format(&doc, &opts);

    assert!(out.contains("Themes:"));
    assert!(out.contains("Variables:"));
    assert!(out.contains("Imports:"));
    assert!(out.contains("content: Hello"));
    assert!(out.contains("fill:"));
    assert!(out.contains("layout:"));
    assert!(out.contains("reusable: true"));
    assert!(out.contains("(100x50)"));
    assert!(out.contains("@(10,20)"));
}

#[test]
fn text_filter_content_only() {
    let doc = sample_doc();
    let opts = OutputOptions {
        filter: Some(["content".into()].into()),
    };
    let out = text::format(&doc, &opts);

    assert!(out.contains("content: Hello"));
    // Should NOT include other fields
    assert!(!out.contains("fill:"));
    assert!(!out.contains("layout:"));
    assert!(!out.contains("reusable: true"));
    assert!(!out.contains("Themes:"));
    assert!(!out.contains("Variables:"));
    assert!(!out.contains("Imports:"));
    // Structure (id/type/name) is always present
    assert!(out.contains("[frame]"));
    assert!(out.contains("[text]"));
}

#[test]
fn text_filter_fill_and_layout() {
    let doc = sample_doc();
    let opts = OutputOptions {
        filter: Some(["fill".into(), "layout".into()].into()),
    };
    let out = text::format(&doc, &opts);

    assert!(out.contains("fill:"));
    assert!(out.contains("layout:"));
    assert!(!out.contains("content:"));
    assert!(!out.contains("reusable:"));
}

#[test]
fn text_filter_size_and_position() {
    let doc = sample_doc();
    let opts = OutputOptions {
        filter: Some(["size".into(), "position".into()].into()),
    };
    let out = text::format(&doc, &opts);

    assert!(out.contains("(100x50)"));
    assert!(out.contains("@(10,20)"));
    assert!(!out.contains("fill:"));
    assert!(!out.contains("content:"));
}

#[test]
fn text_filter_excludes_size_and_position_when_not_requested() {
    let doc = sample_doc();
    let opts = OutputOptions {
        filter: Some(["content".into()].into()),
    };
    let out = text::format(&doc, &opts);

    assert!(!out.contains("(100x50)"));
    assert!(!out.contains("@(10,20)"));
}

// ============================================================
// JSON format: --filter
// ============================================================

#[test]
fn json_no_filter_includes_all_keys() {
    let doc = sample_doc();
    let opts = OutputOptions { filter: None };
    let out = json::format(&doc, &opts).unwrap();
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();

    let child = &v["children"][0];
    assert!(child.get("fill").is_some());
    assert!(child.get("layout").is_some());
    assert!(child.get("width").is_some());
    assert!(child.get("x").is_some());
}

#[test]
fn json_filter_keeps_structural_keys() {
    let doc = sample_doc();
    let opts = OutputOptions {
        filter: Some(["fill".into()].into()),
    };
    let out = json::format(&doc, &opts).unwrap();
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();

    // Structural keys always present
    assert!(v.get("version").is_some());
    assert!(v.get("children").is_some());
    let child = &v["children"][0];
    assert!(child.get("type").is_some());
    assert!(child.get("id").is_some());
    assert!(child.get("name").is_some());
}

#[test]
fn json_filter_removes_non_matching_keys() {
    let doc = sample_doc();
    let opts = OutputOptions {
        filter: Some(["fill".into()].into()),
    };
    let out = json::format(&doc, &opts).unwrap();
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();

    let child = &v["children"][0];
    assert!(child.get("fill").is_some());
    // Non-structural, non-filtered keys should be absent
    assert!(child.get("layout").is_none());
    assert!(child.get("width").is_none());
    assert!(child.get("x").is_none());
    assert!(child.get("reusable").is_none());
}

#[test]
fn json_filter_applies_recursively() {
    let doc = sample_doc();
    let opts = OutputOptions {
        filter: Some(["content".into()].into()),
    };
    let out = json::format(&doc, &opts).unwrap();
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();

    let text_node = &v["children"][0]["children"][0];
    assert!(text_node.get("content").is_some());
    assert!(text_node.get("type").is_some());
    // Non-matching keys removed from nested nodes too
    assert!(text_node.get("fill").is_none());
}
