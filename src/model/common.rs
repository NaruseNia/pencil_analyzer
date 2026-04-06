use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;

/// A value that can be either a concrete type or a `$variable` reference.
#[derive(Debug, Clone, PartialEq)]
pub enum OrVariable<T> {
    Value(T),
    Variable(String),
}

impl<T: Serialize> Serialize for OrVariable<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            OrVariable::Value(v) => v.serialize(serializer),
            OrVariable::Variable(name) => serializer.serialize_str(name),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for OrVariable<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        if let serde_json::Value::String(ref s) = value {
            if s.starts_with('$') {
                return Ok(OrVariable::Variable(s.clone()));
            }
        }
        T::deserialize(value).map(OrVariable::Value).map_err(serde::de::Error::custom)
    }
}

impl<T: fmt::Display> fmt::Display for OrVariable<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrVariable::Value(v) => write!(f, "{v}"),
            OrVariable::Variable(name) => write!(f, "{name}"),
        }
    }
}

pub type NumberOrVariable = OrVariable<f64>;
pub type ColorOrVariable = OrVariable<String>;
pub type BooleanOrVariable = OrVariable<bool>;
pub type StringOrVariable = OrVariable<String>;

/// Theme axis selections, e.g. `{ "mode": "dark" }`.
pub type Theme = HashMap<String, String>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum BlendMode {
    Normal,
    Darken,
    Multiply,
    LinearBurn,
    ColorBurn,
    Light,
    Screen,
    LinearDodge,
    ColorDodge,
    Overlay,
    SoftLight,
    HardLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<StringOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<StringOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_style: Option<StringOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<BooleanOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_height: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_align: Option<TextAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_align_vertical: Option<TextAlignVertical>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<BooleanOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TextAlignVertical {
    Top,
    Middle,
    Bottom,
}

/// Text content can be a simple string/variable, or an array of styled segments.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TextContent {
    Plain(StringOrVariable),
    Styled(Vec<TextStyle>),
}

/// Metadata attached to an entity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    #[serde(rename = "type")]
    pub metadata_type: String,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
