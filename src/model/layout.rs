use serde::{Deserialize, Serialize};

use super::common::NumberOrVariable;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Layout {
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
#[serde(rename_all = "snake_case")]
pub enum LayoutMode {
    None,
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum JustifyContent {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AlignItems {
    Start,
    Center,
    End,
}

/// Padding: single value, [horizontal, vertical], or [top, right, bottom, left].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Padding {
    Single(NumberOrVariable),
    HV(NumberOrVariable, NumberOrVariable),
    TRBL(
        NumberOrVariable,
        NumberOrVariable,
        NumberOrVariable,
        NumberOrVariable,
    ),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Position {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}

/// SizingBehavior is a string like "fit_content", "fill_container", "fit_content(100)".
pub type SizingBehavior = String;

/// Width/Height can be a number, variable, or sizing behavior string.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Dimension {
    Number(f64),
    Variable(String),
}
