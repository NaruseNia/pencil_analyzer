use pencil_analyzer::model::common::*;
use pencil_analyzer::model::document::*;
use pencil_analyzer::model::graphics::*;
use pencil_analyzer::model::layout::*;
use pencil_analyzer::model::objects::*;
use pencil_analyzer::parse;

// ============================================================
// OrVariable<T> deserialization
// ============================================================

#[test]
fn or_variable_number_value() {
    let v: NumberOrVariable = serde_json::from_str("42.0").unwrap();
    assert_eq!(v, OrVariable::Value(42.0));
}

#[test]
fn or_variable_number_variable() {
    let v: NumberOrVariable = serde_json::from_str(r#""$spacing.gap""#).unwrap();
    assert_eq!(v, OrVariable::Variable("$spacing.gap".into()));
}

#[test]
fn or_variable_bool_value() {
    let v: BooleanOrVariable = serde_json::from_str("true").unwrap();
    assert_eq!(v, OrVariable::Value(true));
}

#[test]
fn or_variable_string_plain() {
    let v: StringOrVariable = serde_json::from_str(r#""hello""#).unwrap();
    assert_eq!(v, OrVariable::Value("hello".into()));
}

#[test]
fn or_variable_string_variable() {
    let v: StringOrVariable = serde_json::from_str(r#""$font.family""#).unwrap();
    assert_eq!(v, OrVariable::Variable("$font.family".into()));
}

#[test]
fn or_variable_color_hex() {
    let v: ColorOrVariable = serde_json::from_str(r##""#FF0000""##).unwrap();
    assert_eq!(v, OrVariable::Value("#FF0000".into()));
}

#[test]
fn or_variable_color_variable() {
    let v: ColorOrVariable = serde_json::from_str(r#""$color.primary""#).unwrap();
    assert_eq!(v, OrVariable::Variable("$color.primary".into()));
}

// ============================================================
// TextContent deserialization
// ============================================================

#[test]
fn text_content_plain_string() {
    let v: TextContent = serde_json::from_str(r#""Hello World""#).unwrap();
    match v {
        TextContent::Plain(OrVariable::Value(s)) => assert_eq!(s, "Hello World"),
        _ => panic!("Expected plain string"),
    }
}

#[test]
fn text_content_plain_variable() {
    let v: TextContent = serde_json::from_str(r#""$label.text""#).unwrap();
    match v {
        TextContent::Plain(OrVariable::Variable(s)) => assert_eq!(s, "$label.text"),
        _ => panic!("Expected variable reference"),
    }
}

// ============================================================
// Layout types deserialization
// ============================================================

#[test]
fn padding_single() {
    let v: Padding = serde_json::from_str("16.0").unwrap();
    assert!(matches!(v, Padding::Single(OrVariable::Value(n)) if n == 16.0));
}

#[test]
fn padding_hv() {
    let v: Padding = serde_json::from_str("[16.0, 32.0]").unwrap();
    match v {
        Padding::HV(h, v_val) => {
            assert_eq!(h, OrVariable::Value(16.0));
            assert_eq!(v_val, OrVariable::Value(32.0));
        }
        _ => panic!("Expected HV padding"),
    }
}

#[test]
fn padding_trbl() {
    let v: Padding = serde_json::from_str("[10.0, 20.0, 30.0, 40.0]").unwrap();
    match v {
        Padding::TRBL(t, r, b, l) => {
            assert_eq!(t, OrVariable::Value(10.0));
            assert_eq!(r, OrVariable::Value(20.0));
            assert_eq!(b, OrVariable::Value(30.0));
            assert_eq!(l, OrVariable::Value(40.0));
        }
        _ => panic!("Expected TRBL padding"),
    }
}

#[test]
fn padding_with_variable() {
    let v: Padding = serde_json::from_str(r#""$spacing.page""#).unwrap();
    assert!(matches!(v, Padding::Single(OrVariable::Variable(ref s)) if s == "$spacing.page"));
}

#[test]
fn dimension_number() {
    let v: Dimension = serde_json::from_str("100").unwrap();
    assert!(matches!(v, Dimension::Number(n) if n == 100.0));
}

#[test]
fn dimension_sizing_behavior() {
    let v: Dimension = serde_json::from_str(r#""fit_content(100)""#).unwrap();
    assert!(matches!(v, Dimension::Variable(ref s) if s == "fit_content(100)"));
}

// ============================================================
// Graphics deserialization
// ============================================================

#[test]
fn fill_bare_color() {
    let v: Fill = serde_json::from_str(r##""#FF0000""##).unwrap();
    assert!(matches!(v, Fill::Color(OrVariable::Value(ref s)) if s == "#FF0000"));
}

#[test]
fn fill_bare_variable() {
    let v: Fill = serde_json::from_str(r#""$color.bg""#).unwrap();
    assert!(matches!(v, Fill::Color(OrVariable::Variable(ref s)) if s == "$color.bg"));
}

#[test]
fn fill_typed_color() {
    let json = r##"{"type":"color","color":"#00FF00"}"##;
    let v: Fill = serde_json::from_str(json).unwrap();
    match v {
        Fill::Typed(TypedFill::Color(cf)) => {
            assert_eq!(cf.color, OrVariable::Value("#00FF00".into()));
        }
        _ => panic!("Expected typed color fill"),
    }
}

#[test]
fn fill_typed_gradient() {
    let json = r##"{"type":"gradient","gradientType":"linear","colors":[{"color":"#000","position":0},{"color":"#FFF","position":1}]}"##;
    let v: Fill = serde_json::from_str(json).unwrap();
    match v {
        Fill::Typed(TypedFill::Gradient(g)) => {
            assert_eq!(g.gradient_type, Some(GradientType::Linear));
            assert_eq!(g.colors.as_ref().unwrap().len(), 2);
        }
        _ => panic!("Expected gradient fill"),
    }
}

#[test]
fn fill_typed_image() {
    let json = r#"{"type":"image","url":"./hero.png","mode":"fill"}"#;
    let v: Fill = serde_json::from_str(json).unwrap();
    match v {
        Fill::Typed(TypedFill::Image(img)) => {
            assert_eq!(img.url, "./hero.png");
            assert_eq!(img.mode, Some(ImageMode::Fill));
        }
        _ => panic!("Expected image fill"),
    }
}

#[test]
fn fills_single() {
    let v: Fills = serde_json::from_str(r##""#FF0000""##).unwrap();
    assert!(matches!(v, Fills::Single(_)));
}

#[test]
fn fills_multiple() {
    let json = r##"["#FF0000", {"type":"color","color":"#00FF00"}]"##;
    let v: Fills = serde_json::from_str(json).unwrap();
    match v {
        Fills::Multiple(fs) => assert_eq!(fs.len(), 2),
        _ => panic!("Expected multiple fills"),
    }
}

#[test]
fn stroke_uniform_thickness() {
    let json = r#"{"align":"inside","thickness":2}"#;
    let v: Stroke = serde_json::from_str(json).unwrap();
    assert_eq!(v.align, Some(StrokeAlign::Inside));
    assert!(matches!(v.thickness, Some(StrokeThickness::Uniform(OrVariable::Value(n))) if n == 2.0));
}

#[test]
fn stroke_per_side_thickness() {
    let json = r#"{"thickness":{"top":1,"bottom":2}}"#;
    let v: Stroke = serde_json::from_str(json).unwrap();
    match v.thickness {
        Some(StrokeThickness::PerSide(sides)) => {
            assert_eq!(sides.top, Some(OrVariable::Value(1.0)));
            assert_eq!(sides.bottom, Some(OrVariable::Value(2.0)));
            assert_eq!(sides.left, None);
        }
        _ => panic!("Expected per-side thickness"),
    }
}

#[test]
fn effect_blur() {
    let json = r#"{"type":"blur","radius":10}"#;
    let v: Effect = serde_json::from_str(json).unwrap();
    match v {
        Effect::Blur(b) => assert_eq!(b.radius, Some(OrVariable::Value(10.0))),
        _ => panic!("Expected blur"),
    }
}

#[test]
fn effect_shadow() {
    let json = r##"{"type":"shadow","shadowType":"outer","offset":{"x":0,"y":4},"blur":8,"color":"#00000040"}"##;
    let v: Effect = serde_json::from_str(json).unwrap();
    match v {
        Effect::Shadow(s) => {
            assert_eq!(s.shadow_type, Some(ShadowType::Outer));
            assert_eq!(s.blur, Some(OrVariable::Value(8.0)));
        }
        _ => panic!("Expected shadow"),
    }
}

#[test]
fn effects_single() {
    let json = r#"{"type":"blur","radius":5}"#;
    let v: Effects = serde_json::from_str(json).unwrap();
    assert!(matches!(v, Effects::Single(_)));
}

#[test]
fn effects_multiple() {
    let json = r#"[{"type":"blur","radius":5},{"type":"shadow","blur":4}]"#;
    let v: Effects = serde_json::from_str(json).unwrap();
    match v {
        Effects::Multiple(es) => assert_eq!(es.len(), 2),
        _ => panic!("Expected multiple effects"),
    }
}

// ============================================================
// Object deserialization (Child enum)
// ============================================================

#[test]
fn child_rectangle() {
    let json = r##"{"type":"rectangle","id":"rect1","x":10,"y":20,"width":100,"height":50,"fill":"#FF0000"}"##;
    let v: Child = serde_json::from_str(json).unwrap();
    assert_eq!(v.type_name(), "rectangle");
    assert_eq!(v.id(), "rect1");
    assert_eq!(v.entity().x, Some(10.0));
}

#[test]
fn child_frame_with_children() {
    let json = r#"{
        "type": "frame",
        "id": "container",
        "width": 400, "height": 300,
        "layout": "vertical",
        "gap": 16,
        "children": [
            {"type": "rectangle", "id": "child1", "width": 100, "height": 100},
            {"type": "text", "id": "child2", "content": "Hello"}
        ]
    }"#;
    let v: Child = serde_json::from_str(json).unwrap();
    assert_eq!(v.type_name(), "frame");
    let children = v.children().unwrap();
    assert_eq!(children.len(), 2);
    assert_eq!(children[0].type_name(), "rectangle");
    assert_eq!(children[1].type_name(), "text");
}

#[test]
fn child_text_with_style() {
    let json = r##"{
        "type": "text",
        "id": "title",
        "content": "Hello World",
        "fontSize": 24,
        "fontWeight": "700",
        "textAlign": "center",
        "fill": "#333333"
    }"##;
    let v: Child = serde_json::from_str(json).unwrap();
    match v {
        Child::Text(t) => {
            assert_eq!(t.style.font_size, Some(OrVariable::Value(24.0)));
            assert_eq!(t.style.font_weight, Some(OrVariable::Value("700".into())));
            assert_eq!(t.style.text_align, Some(TextAlign::Center));
        }
        _ => panic!("Expected text"),
    }
}

#[test]
fn child_ellipse() {
    let json = r#"{"type":"ellipse","id":"circle","width":100,"height":100,"innerRadius":0.5}"#;
    let v: Child = serde_json::from_str(json).unwrap();
    match v {
        Child::Ellipse(e) => {
            assert_eq!(e.inner_radius, Some(OrVariable::Value(0.5)));
        }
        _ => panic!("Expected ellipse"),
    }
}

#[test]
fn child_line() {
    let json = r#"{"type":"line","id":"divider","width":400,"height":0}"#;
    let v: Child = serde_json::from_str(json).unwrap();
    assert_eq!(v.type_name(), "line");
}

#[test]
fn child_polygon() {
    let json = r#"{"type":"polygon","id":"hex","width":100,"height":100,"polygonCount":6}"#;
    let v: Child = serde_json::from_str(json).unwrap();
    match v {
        Child::Polygon(p) => {
            assert_eq!(p.polygon_count, Some(OrVariable::Value(6.0)));
        }
        _ => panic!("Expected polygon"),
    }
}

#[test]
fn child_path() {
    let json = r#"{"type":"path","id":"custom","geometry":"M 0 0 L 100 100","fillRule":"evenodd"}"#;
    let v: Child = serde_json::from_str(json).unwrap();
    match v {
        Child::Path(p) => {
            assert_eq!(p.geometry.as_deref(), Some("M 0 0 L 100 100"));
            assert_eq!(p.fill_rule, Some(FillRule::Evenodd));
        }
        _ => panic!("Expected path"),
    }
}

#[test]
fn child_group() {
    let json = r#"{
        "type": "group",
        "id": "grp",
        "children": [
            {"type": "rectangle", "id": "r1"}
        ]
    }"#;
    let v: Child = serde_json::from_str(json).unwrap();
    assert_eq!(v.type_name(), "group");
    assert_eq!(v.children().unwrap().len(), 1);
}

#[test]
fn child_icon_font() {
    let json = r#"{"type":"icon_font","id":"icon1","iconFontFamily":"lucide","iconFontName":"check","weight":400}"#;
    let v: Child = serde_json::from_str(json).unwrap();
    match v {
        Child::IconFont(i) => {
            assert_eq!(
                i.icon_font_family,
                Some(OrVariable::Value("lucide".into()))
            );
            assert_eq!(i.icon_font_name, Some(OrVariable::Value("check".into())));
        }
        _ => panic!("Expected icon_font"),
    }
}

#[test]
fn child_note() {
    let json = r#"{"type":"note","id":"n1","content":"Design note here","width":200,"height":100}"#;
    let v: Child = serde_json::from_str(json).unwrap();
    assert_eq!(v.type_name(), "note");
}

#[test]
fn child_prompt() {
    let json = r#"{"type":"prompt","id":"p1","content":"Generate a button","model":"claude"}"#;
    let v: Child = serde_json::from_str(json).unwrap();
    match v {
        Child::Prompt(p) => {
            assert_eq!(p.model, Some(OrVariable::Value("claude".into())));
        }
        _ => panic!("Expected prompt"),
    }
}

#[test]
fn child_context() {
    let json = r#"{"type":"context","id":"ctx1","content":"This is context info"}"#;
    let v: Child = serde_json::from_str(json).unwrap();
    assert_eq!(v.type_name(), "context");
}

#[test]
fn child_ref_simple() {
    let json = r#"{"type":"ref","id":"btn1","ref":"round-button","x":100,"y":0}"#;
    let v: Child = serde_json::from_str(json).unwrap();
    match v {
        Child::Ref(r) => {
            assert_eq!(r.ref_id, "round-button");
            assert!(r.descendants.is_none());
        }
        _ => panic!("Expected ref"),
    }
}

#[test]
fn child_ref_with_overrides_and_descendants() {
    let json = r##"{
        "type": "ref",
        "id": "red-btn",
        "ref": "round-button",
        "fill": "#FF0000",
        "descendants": {
            "label": { "content": "Cancel" },
            "ok-button/label": { "content": "Save" }
        }
    }"##;
    let v: Child = serde_json::from_str(json).unwrap();
    match v {
        Child::Ref(r) => {
            assert_eq!(r.ref_id, "round-button");
            let desc = r.descendants.as_ref().unwrap();
            assert_eq!(desc.len(), 2);
            assert!(desc.contains_key("label"));
            assert!(desc.contains_key("ok-button/label"));
            assert!(r.overrides.contains_key("fill"));
        }
        _ => panic!("Expected ref"),
    }
}

#[test]
fn child_frame_reusable_with_slot() {
    let json = r#"{
        "type": "frame",
        "id": "sidebar",
        "reusable": true,
        "children": [
            {
                "type": "frame",
                "id": "content",
                "slot": ["menu-item", "icon-button"]
            }
        ]
    }"#;
    let v: Child = serde_json::from_str(json).unwrap();
    match v {
        Child::Frame(f) => {
            assert_eq!(f.entity.reusable, Some(true));
            let content = &f.children.as_ref().unwrap()[0];
            match content {
                Child::Frame(inner) => {
                    assert_eq!(
                        inner.slot,
                        Some(vec!["menu-item".to_string(), "icon-button".to_string()])
                    );
                }
                _ => panic!("Expected inner frame"),
            }
        }
        _ => panic!("Expected frame"),
    }
}

#[test]
fn child_corner_radius_uniform() {
    let json = r#"{"type":"rectangle","id":"r1","cornerRadius":8}"#;
    let v: Child = serde_json::from_str(json).unwrap();
    match v {
        Child::Rectangle(r) => {
            assert!(matches!(
                r.corner_radius,
                Some(CornerRadius::Uniform(OrVariable::Value(n))) if n == 8.0
            ));
        }
        _ => panic!("Expected rectangle"),
    }
}

#[test]
fn child_corner_radius_per_corner() {
    let json = r#"{"type":"rectangle","id":"r2","cornerRadius":[4,8,12,16]}"#;
    let v: Child = serde_json::from_str(json).unwrap();
    match v {
        Child::Rectangle(r) => {
            assert!(matches!(r.corner_radius, Some(CornerRadius::PerCorner(..))));
        }
        _ => panic!("Expected rectangle"),
    }
}

// ============================================================
// Document deserialization
// ============================================================

#[test]
fn document_minimal() {
    let json = r#"{"version":"2.9","children":[]}"#;
    let doc: Document = serde_json::from_str(json).unwrap();
    assert_eq!(doc.version, "2.9");
    assert!(doc.children.is_empty());
    assert!(doc.themes.is_none());
    assert!(doc.variables.is_none());
}

#[test]
fn document_with_variables() {
    let json = r##"{
        "version": "2.9",
        "variables": {
            "color.bg": { "type": "color", "value": "#FFFFFF" },
            "spacing": { "type": "number", "value": 16 },
            "label": { "type": "string", "value": "Hello" },
            "visible": { "type": "boolean", "value": true }
        },
        "children": []
    }"##;
    let doc: Document = serde_json::from_str(json).unwrap();
    let vars = doc.variables.unwrap();
    assert_eq!(vars.len(), 4);
    assert!(matches!(vars["color.bg"], VariableDef::Color { .. }));
    assert!(matches!(vars["spacing"], VariableDef::Number { .. }));
    assert!(matches!(vars["label"], VariableDef::String { .. }));
    assert!(matches!(vars["visible"], VariableDef::Boolean { .. }));
}

#[test]
fn document_variable_themed() {
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
        "children": []
    }"##;
    let doc: Document = serde_json::from_str(json).unwrap();
    match &doc.variables.as_ref().unwrap()["color.bg"] {
        VariableDef::Color { value } => match value {
            VariableValue::Themed(entries) => {
                assert_eq!(entries.len(), 2);
                assert_eq!(
                    entries[0].theme.as_ref().unwrap().get("mode").unwrap(),
                    "light"
                );
            }
            _ => panic!("Expected themed value"),
        },
        _ => panic!("Expected color variable"),
    }
}

#[test]
fn document_with_imports() {
    let json = r#"{
        "version": "2.9",
        "imports": {
            "ds": "./design-system.pen",
            "icons": "../shared/icons.pen"
        },
        "children": []
    }"#;
    let doc: Document = serde_json::from_str(json).unwrap();
    let imports = doc.imports.unwrap();
    assert_eq!(imports.len(), 2);
    assert_eq!(imports["ds"], "./design-system.pen");
}

// ============================================================
// Integration: parse from file
// ============================================================

#[test]
fn parse_sample_fixture() {
    let doc = parse::parse_document(std::path::Path::new("tests/fixtures/sample.pen")).unwrap();
    assert_eq!(doc.version, "2.9");
    assert_eq!(doc.children.len(), 2);

    // First child: round-button frame
    assert_eq!(doc.children[0].id(), "round-button");
    assert_eq!(doc.children[0].type_name(), "frame");
    let btn_children = doc.children[0].children().unwrap();
    assert_eq!(btn_children.len(), 1);
    assert_eq!(btn_children[0].id(), "label");

    // Second child: landing-page frame
    assert_eq!(doc.children[1].id(), "landing-page");
    let page_children = doc.children[1].children().unwrap();
    assert_eq!(page_children.len(), 2);
    assert_eq!(page_children[0].type_name(), "text");
    assert_eq!(page_children[1].type_name(), "ref");

    // Check ref
    match &page_children[1] {
        Child::Ref(r) => {
            assert_eq!(r.ref_id, "round-button");
            assert!(r.descendants.is_some());
        }
        _ => panic!("Expected ref"),
    }

    // Variables
    let vars = doc.variables.unwrap();
    assert_eq!(vars.len(), 2);
    assert!(vars.contains_key("color.background"));
    assert!(vars.contains_key("color.text"));

    // Themes
    let themes = doc.themes.unwrap();
    assert_eq!(themes["mode"], vec!["light", "dark"]);
}

// ============================================================
// Round-trip: parse -> serialize -> parse
// ============================================================

#[test]
fn roundtrip_sample_fixture() {
    let doc = parse::parse_document(std::path::Path::new("tests/fixtures/sample.pen")).unwrap();
    let json = serde_json::to_string_pretty(&doc).unwrap();
    let doc2: Document = serde_json::from_str(&json).unwrap();
    assert_eq!(doc, doc2);
}
