use serde::{Deserialize, Serialize};

use super::common::NumberOrVariable;

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

/// Width/Height can be a number, variable, or sizing behavior string.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Dimension {
    Number(f64),
    Variable(String),
}
