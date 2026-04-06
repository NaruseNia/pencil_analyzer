use serde::{Deserialize, Serialize};

use super::common::{BlendMode, BooleanOrVariable, ColorOrVariable, NumberOrVariable};

/// A single fill. Can be a bare color string or a typed fill object.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Fill {
    Color(ColorOrVariable),
    Typed(TypedFill),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TypedFill {
    Color(ColorFill),
    Gradient(GradientFill),
    Image(ImageFill),
    MeshGradient(MeshGradientFill),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ColorFill {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<BooleanOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<BlendMode>,
    pub color: ColorOrVariable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GradientFill {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<BooleanOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<BlendMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_type: Option<GradientType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<GradientPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<GradientSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<GradientStop>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GradientType {
    Linear,
    Radial,
    Angular,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GradientPosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<NumberOrVariable>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GradientSize {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<NumberOrVariable>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GradientStop {
    pub color: ColorOrVariable,
    pub position: NumberOrVariable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImageFill {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<BooleanOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<BlendMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<NumberOrVariable>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<ImageMode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ImageMode {
    Stretch,
    Fill,
    Fit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MeshGradientFill {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<BooleanOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<BlendMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<ColorOrVariable>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<Vec<serde_json::Value>>,
}

/// Fills can be a single fill or an array of fills.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Fills {
    Single(Fill),
    Multiple(Vec<Fill>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Stroke {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<StrokeAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thickness: Option<StrokeThickness>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join: Option<StrokeJoin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miter_angle: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap: Option<StrokeCap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_pattern: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<Fills>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum StrokeAlign {
    Inside,
    Center,
    Outside,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum StrokeThickness {
    Uniform(NumberOrVariable),
    PerSide(StrokeThicknessSides),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StrokeThicknessSides {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<NumberOrVariable>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum StrokeJoin {
    Miter,
    Bevel,
    Round,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum StrokeCap {
    None,
    Round,
    Square,
}

/// A single effect.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Effect {
    Blur(BlurEffect),
    BackgroundBlur(BackgroundBlurEffect),
    Shadow(ShadowEffect),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlurEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<BooleanOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<NumberOrVariable>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundBlurEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<BooleanOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<NumberOrVariable>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShadowEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<BooleanOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_type: Option<ShadowType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<ShadowOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spread: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur: Option<NumberOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<ColorOrVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<BlendMode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ShadowType {
    Inner,
    Outer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShadowOffset {
    pub x: NumberOrVariable,
    pub y: NumberOrVariable,
}

/// Effects can be a single effect or an array.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Effects {
    Single(Effect),
    Multiple(Vec<Effect>),
}
