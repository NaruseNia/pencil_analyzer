use pencil_analyzer::model::common::*;
use pencil_analyzer::model::objects::*;
use pencil_analyzer::parse;
use pencil_analyzer::resolve::refs::resolve_refs;

#[test]
fn resolve_simple_ref() {
    let json = r##"{
        "version": "2.9",
        "children": [
            {
                "type": "rectangle",
                "id": "box",
                "reusable": true,
                "width": 100, "height": 100,
                "fill": "#FF0000"
            },
            {
                "type": "ref",
                "id": "box-copy",
                "ref": "box",
                "x": 200, "y": 0
            }
        ]
    }"##;
    let doc = pencil_analyzer::parse::parse_from_str(json).unwrap();
    let resolved = resolve_refs(&doc).unwrap();

    assert_eq!(resolved.children.len(), 2);
    // The ref should be resolved to a rectangle
    assert_eq!(resolved.children[1].type_name(), "rectangle");
    assert_eq!(resolved.children[1].id(), "box-copy");
    assert_eq!(resolved.children[1].entity().x, Some(200.0));
}

#[test]
fn resolve_ref_with_property_override() {
    let json = r##"{
        "version": "2.9",
        "children": [
            {
                "type": "rectangle",
                "id": "box",
                "reusable": true,
                "width": 100, "height": 100,
                "fill": "#FF0000"
            },
            {
                "type": "ref",
                "id": "blue-box",
                "ref": "box",
                "fill": "#0000FF"
            }
        ]
    }"##;
    let doc = parse::parse_from_str(json).unwrap();
    let resolved = resolve_refs(&doc).unwrap();

    match &resolved.children[1] {
        Child::Rectangle(r) => {
            assert_eq!(r.entity.id, "blue-box");
            // fill should be overridden to blue
            let fill_json = serde_json::to_value(&r.fill).unwrap();
            assert!(fill_json.to_string().contains("#0000FF"));
        }
        other => panic!("Expected rectangle, got {}", other.type_name()),
    }
}

#[test]
fn resolve_ref_with_descendants() {
    let json = r#"{
        "version": "2.9",
        "children": [
            {
                "type": "frame",
                "id": "button",
                "reusable": true,
                "width": 200, "height": 48,
                "children": [
                    {
                        "type": "text",
                        "id": "label",
                        "content": "Submit"
                    }
                ]
            },
            {
                "type": "ref",
                "id": "cancel-btn",
                "ref": "button",
                "descendants": {
                    "label": { "content": "Cancel" }
                }
            }
        ]
    }"#;
    let doc = parse::parse_from_str(json).unwrap();
    let resolved = resolve_refs(&doc).unwrap();

    // The ref should be resolved to a frame
    match &resolved.children[1] {
        Child::Frame(f) => {
            assert_eq!(f.entity.id, "cancel-btn");
            let children = f.children.as_ref().unwrap();
            assert_eq!(children.len(), 1);
            match &children[0] {
                Child::Text(t) => {
                    match &t.content {
                        Some(TextContent::Plain(OrVariable::Value(s))) => {
                            assert_eq!(s, "Cancel");
                        }
                        other => panic!("Expected plain text 'Cancel', got {other:?}"),
                    }
                }
                other => panic!("Expected text, got {}", other.type_name()),
            }
        }
        other => panic!("Expected frame, got {}", other.type_name()),
    }
}

#[test]
fn resolve_nested_ref() {
    let json = r#"{
        "version": "2.9",
        "children": [
            {
                "type": "frame",
                "id": "button",
                "reusable": true,
                "children": [
                    { "type": "text", "id": "label", "content": "Click" }
                ]
            },
            {
                "type": "frame",
                "id": "card",
                "reusable": true,
                "children": [
                    { "type": "text", "id": "title", "content": "Card Title" },
                    { "type": "ref", "id": "card-btn", "ref": "button" }
                ]
            },
            {
                "type": "ref",
                "id": "my-card",
                "ref": "card"
            }
        ]
    }"#;
    let doc = parse::parse_from_str(json).unwrap();
    let resolved = resolve_refs(&doc).unwrap();

    // my-card should be a frame now
    match &resolved.children[2] {
        Child::Frame(f) => {
            assert_eq!(f.entity.id, "my-card");
            let children = f.children.as_ref().unwrap();
            assert_eq!(children.len(), 2);
            // The inner ref (card-btn) should also be resolved
            assert_eq!(children[1].type_name(), "frame");
            assert_eq!(children[1].id(), "card-btn");
        }
        other => panic!("Expected frame, got {}", other.type_name()),
    }
}

#[test]
fn resolve_circular_ref_detected() {
    let json = r#"{
        "version": "2.9",
        "children": [
            {
                "type": "frame",
                "id": "a",
                "reusable": true,
                "children": [
                    { "type": "ref", "id": "inner", "ref": "a" }
                ]
            },
            {
                "type": "ref",
                "id": "start",
                "ref": "a"
            }
        ]
    }"#;
    let doc = parse::parse_from_str(json).unwrap();
    let result = resolve_refs(&doc);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Circular ref"));
}

#[test]
fn resolve_unresolved_ref() {
    let json = r#"{
        "version": "2.9",
        "children": [
            { "type": "ref", "id": "x", "ref": "nonexistent" }
        ]
    }"#;
    let doc = parse::parse_from_str(json).unwrap();
    let result = resolve_refs(&doc);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unresolved ref"));
}

#[test]
fn resolve_descendant_with_slash_path() {
    let json = r#"{
        "version": "2.9",
        "children": [
            {
                "type": "frame",
                "id": "button",
                "reusable": true,
                "children": [
                    { "type": "text", "id": "label", "content": "OK" }
                ]
            },
            {
                "type": "frame",
                "id": "dialog",
                "reusable": true,
                "children": [
                    { "type": "text", "id": "message", "content": "Are you sure?" },
                    { "type": "ref", "id": "ok-btn", "ref": "button" }
                ]
            },
            {
                "type": "ref",
                "id": "save-dialog",
                "ref": "dialog",
                "descendants": {
                    "message": { "content": "Save changes?" },
                    "ok-btn/label": { "content": "Save" }
                }
            }
        ]
    }"#;
    let doc = parse::parse_from_str(json).unwrap();
    let resolved = resolve_refs(&doc).unwrap();

    match &resolved.children[2] {
        Child::Frame(f) => {
            assert_eq!(f.entity.id, "save-dialog");
            let children = f.children.as_ref().unwrap();

            // message text should be overridden
            match &children[0] {
                Child::Text(t) => {
                    match &t.content {
                        Some(TextContent::Plain(OrVariable::Value(s))) => {
                            assert_eq!(s, "Save changes?");
                        }
                        other => panic!("Expected 'Save changes?', got {other:?}"),
                    }
                }
                other => panic!("Expected text, got {}", other.type_name()),
            }

            // ok-btn should be resolved to a frame, and its label should say "Save"
            match &children[1] {
                Child::Frame(btn) => {
                    assert_eq!(btn.entity.id, "ok-btn");
                    let btn_children = btn.children.as_ref().unwrap();
                    match &btn_children[0] {
                        Child::Text(t) => {
                            match &t.content {
                                Some(TextContent::Plain(OrVariable::Value(s))) => {
                                    assert_eq!(s, "Save");
                                }
                                other => panic!("Expected 'Save', got {other:?}"),
                            }
                        }
                        other => panic!("Expected text, got {}", other.type_name()),
                    }
                }
                other => panic!("Expected frame, got {}", other.type_name()),
            }
        }
        other => panic!("Expected frame, got {}", other.type_name()),
    }
}

#[test]
fn resolve_descendant_object_replacement() {
    let json = r#"{
        "version": "2.9",
        "children": [
            {
                "type": "frame",
                "id": "button",
                "reusable": true,
                "children": [
                    { "type": "text", "id": "label", "content": "OK" }
                ]
            },
            {
                "type": "ref",
                "id": "icon-btn",
                "ref": "button",
                "descendants": {
                    "label": {
                        "id": "icon",
                        "type": "icon_font",
                        "iconFontFamily": "lucide",
                        "iconFontName": "check"
                    }
                }
            }
        ]
    }"#;
    let doc = parse::parse_from_str(json).unwrap();
    let resolved = resolve_refs(&doc).unwrap();

    match &resolved.children[1] {
        Child::Frame(f) => {
            let children = f.children.as_ref().unwrap();
            assert_eq!(children.len(), 1);
            // label should be replaced with an icon_font node
            assert_eq!(children[0].type_name(), "icon_font");
            assert_eq!(children[0].id(), "icon");
        }
        other => panic!("Expected frame, got {}", other.type_name()),
    }
}

#[test]
fn resolve_sample_fixture() {
    let doc = parse::parse_document(std::path::Path::new("tests/fixtures/sample.pen")).unwrap();
    let resolved = resolve_refs(&doc).unwrap();

    assert_eq!(resolved.children.len(), 2);

    // cta-button (ref -> round-button) should be resolved to a frame
    let page = &resolved.children[1];
    let page_children = page.children().unwrap();
    assert_eq!(page_children[1].type_name(), "frame");
    assert_eq!(page_children[1].id(), "cta-button");

    // The label inside should have content "Get Started"
    match &page_children[1] {
        Child::Frame(f) => {
            let btn_children = f.children.as_ref().unwrap();
            match &btn_children[0] {
                Child::Text(t) => {
                    match &t.content {
                        Some(TextContent::Plain(OrVariable::Value(s))) => {
                            assert_eq!(s, "Get Started");
                        }
                        other => panic!("Expected 'Get Started', got {other:?}"),
                    }
                }
                other => panic!("Expected text, got {}", other.type_name()),
            }
        }
        other => panic!("Expected frame, got {}", other.type_name()),
    }
}
