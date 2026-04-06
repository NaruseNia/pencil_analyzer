use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::common::{
    BooleanOrVariable, Metadata, NumberOrVariable, StringOrVariable, TextContent,
    TextStyle, Theme,
};
use super::graphics::{Effects, Fills, Stroke};
use super::layout::{AlignItems, Dimension, JustifyContent, LayoutMode, Padding};

/// Common entity fields shared by all objects.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reusable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<Theme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<BooleanOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flip_x: Option<BooleanOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flip_y: Option<BooleanOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_position: Option<LayoutPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<NumberOrVariable>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum LayoutPosition {
    Auto,
    Absolute,
}

/// The main child node enum, discriminated by `type`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Child {
    Rectangle(Rectangle),
    Frame(Frame),
    Text(Text),
    Ellipse(Ellipse),
    Line(Line),
    Polygon(Polygon),
    Path(PathObj),
    Group(Group),
    Note(Note),
    Prompt(Prompt),
    Context(Context),
    IconFont(IconFont),
    Ref(Ref),
}

impl Child {
    pub fn id(&self) -> &str {
        match self {
            Child::Rectangle(n) => &n.entity.id,
            Child::Frame(n) => &n.entity.id,
            Child::Text(n) => &n.entity.id,
            Child::Ellipse(n) => &n.entity.id,
            Child::Line(n) => &n.entity.id,
            Child::Polygon(n) => &n.entity.id,
            Child::Path(n) => &n.entity.id,
            Child::Group(n) => &n.entity.id,
            Child::Note(n) => &n.entity.id,
            Child::Prompt(n) => &n.entity.id,
            Child::Context(n) => &n.entity.id,
            Child::IconFont(n) => &n.entity.id,
            Child::Ref(n) => &n.entity.id,
        }
    }

    pub fn type_name(&self) -> &str {
        match self {
            Child::Rectangle(_) => "rectangle",
            Child::Frame(_) => "frame",
            Child::Text(_) => "text",
            Child::Ellipse(_) => "ellipse",
            Child::Line(_) => "line",
            Child::Polygon(_) => "polygon",
            Child::Path(_) => "path",
            Child::Group(_) => "group",
            Child::Note(_) => "note",
            Child::Prompt(_) => "prompt",
            Child::Context(_) => "context",
            Child::IconFont(_) => "icon_font",
            Child::Ref(_) => "ref",
        }
    }

    pub fn children(&self) -> Option<&[Child]> {
        match self {
            Child::Frame(f) => f.children.as_deref(),
            Child::Group(g) => g.children.as_deref(),
            _ => None,
        }
    }

    pub fn entity(&self) -> &Entity {
        match self {
            Child::Rectangle(n) => &n.entity,
            Child::Frame(n) => &n.entity,
            Child::Text(n) => &n.entity,
            Child::Ellipse(n) => &n.entity,
            Child::Line(n) => &n.entity,
            Child::Polygon(n) => &n.entity,
            Child::Path(n) => &n.entity,
            Child::Group(n) => &n.entity,
            Child::Note(n) => &n.entity,
            Child::Prompt(n) => &n.entity,
            Child::Context(n) => &n.entity,
            Child::IconFont(n) => &n.entity,
            Child::Ref(n) => &n.entity,
        }
    }
}

// --- Concrete object types ---

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Rectangle {
    #[serde(flatten)]
    pub entity: Entity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<CornerRadius>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<Fills>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<Stroke>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<Effects>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CornerRadius {
    Uniform(NumberOrVariable),
    PerCorner(
        NumberOrVariable,
        NumberOrVariable,
        NumberOrVariable,
        NumberOrVariable,
    ),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    #[serde(flatten)]
    pub entity: Entity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<CornerRadius>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<Fills>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<Stroke>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<Effects>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Child>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<BooleanOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<Vec<String>>,
    // Layout properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<LayoutMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_include_stroke: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<Padding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justify_content: Option<JustifyContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align_items: Option<AlignItems>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    #[serde(flatten)]
    pub entity: Entity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<TextContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_growth: Option<TextGrowth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<Fills>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<Stroke>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<Effects>,
    #[serde(flatten)]
    pub style: TextStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum TextGrowth {
    Auto,
    FixedWidth,
    FixedWidthHeight,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ellipse {
    #[serde(flatten)]
    pub entity: Entity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_radius: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_angle: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sweep_angle: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<Fills>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<Stroke>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<Effects>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    #[serde(flatten)]
    pub entity: Entity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<Fills>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<Stroke>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<Effects>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Polygon {
    #[serde(flatten)]
    pub entity: Entity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon_count: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<Fills>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<Stroke>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<Effects>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PathObj {
    #[serde(flatten)]
    pub entity: Entity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_rule: Option<FillRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<Fills>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<Stroke>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<Effects>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum FillRule {
    Nonzero,
    Evenodd,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    #[serde(flatten)]
    pub entity: Entity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Child>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<Effects>,
    // Layout properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<LayoutMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_include_stroke: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<Padding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justify_content: Option<JustifyContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align_items: Option<AlignItems>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    #[serde(flatten)]
    pub entity: Entity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<TextContent>,
    #[serde(flatten)]
    pub style: TextStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Prompt {
    #[serde(flatten)]
    pub entity: Entity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<TextContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<StringOrVariable>,
    #[serde(flatten)]
    pub style: TextStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    #[serde(flatten)]
    pub entity: Entity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<TextContent>,
    #[serde(flatten)]
    pub style: TextStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IconFont {
    #[serde(flatten)]
    pub entity: Entity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_font_name: Option<StringOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_font_family: Option<StringOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<Fills>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<Effects>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ref {
    #[serde(flatten)]
    pub entity: Entity,
    #[serde(rename = "ref")]
    pub ref_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descendants: Option<HashMap<String, serde_json::Value>>,
    /// Catch-all for property overrides from the referenced component.
    #[serde(flatten)]
    pub overrides: HashMap<String, serde_json::Value>,
}
