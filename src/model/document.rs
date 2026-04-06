use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::common::{BooleanOrVariable, ColorOrVariable, NumberOrVariable, StringOrVariable, Theme};
use super::objects::Child;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Document {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub themes: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imports: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, VariableDef>>,
    pub children: Vec<Child>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum VariableDef {
    Boolean { value: VariableValue<BooleanOrVariable> },
    Color { value: VariableValue<ColorOrVariable> },
    Number { value: VariableValue<NumberOrVariable> },
    String { value: VariableValue<StringOrVariable> },
}

/// A variable value: either a single value, or a list of themed values.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum VariableValue<T> {
    Single(T),
    Themed(Vec<ThemedValue<T>>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThemedValue<T> {
    pub value: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<Theme>,
}
