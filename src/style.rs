use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Context;

use crate::common::Boolean;

/// Known fields in the [V4+ Styles] section
#[derive(Debug, Clone, PartialEq, strum::Display, strum::EnumString)]
#[strum(serialize_all = "PascalCase")]
pub enum StyleField {
    #[strum(default)]
    Unknown(String),
    Name,
    Fontname,
    Fontsize,
    PrimaryColour,
    SecondaryColour,
    OutlineColour,
    BackColour,
    Bold,
    Italic,
    Underline,
    StrikeOut,
    ScaleX,
    ScaleY,
    Spacing,
    Angle,
    BorderStyle,
    Outline,
    Shadow,
    Alignment,
    MarginL,
    MarginR,
    MarginV,
    Encoding,
}

#[derive(Debug, Clone)]
pub struct StyleContext {
    format: Vec<StyleField>,
}

impl StyleContext {
    pub fn from_format_line(line: &str) -> anyhow::Result<Self> {
        let (_, fields) = line.split_once(':').context("unable to split on ':'")?;
        let format = fields
            .split(',')
            .map(|x| StyleField::from_str(x.trim()))
            .collect::<Result<_, strum::ParseError>>()?;
        Ok(Self { format })
    }

    pub fn style_from_line(&self, line: &str) -> anyhow::Result<Style> {
        let (style_type, fields) = line.split_once(':').context("unable to split on ':'")?;

        let style_type = StyleType::from_str(style_type)?;

        let fields: Vec<String> = fields
            .splitn(self.format.len(), ',')
            .map(|x| x.trim().to_string())
            .collect();

        if fields.len() != self.format.len() {
            anyhow::bail!(
                "wrong number of fields, should: {}, actual: {}",
                fields.len(),
                self.format.len()
            );
        }

        let mut style = Style {
            style_type,
            ..Default::default()
        };

        for (i, field) in fields.into_iter().enumerate() {
            let field_type = self
                .format
                .get(i)
                .context("no item found at style format index: {i}")?;

            match field_type {
                StyleField::Unknown(x) => {
                    style.unknown_fields.insert(x.clone(), field);
                }
                StyleField::Name => style.name = Some(field),
                StyleField::Fontname => style.fontname = Some(field),
                StyleField::Fontsize => style.fontsize = Some(field),
                StyleField::PrimaryColour => style.primary_color = Some(field),
                StyleField::SecondaryColour => style.secondary_color = Some(field),
                StyleField::OutlineColour => style.outline_color = Some(field),
                StyleField::BackColour => style.back_color = Some(field),
                StyleField::Bold => style.bold = Some(Boolean::from_str(&field)?),
                StyleField::Italic => style.italic = Some(Boolean::from_str(&field)?),
                StyleField::Underline => style.underline = Some(Boolean::from_str(&field)?),
                StyleField::StrikeOut => style.strike_out = Some(Boolean::from_str(&field)?),
                StyleField::ScaleX => style.scale_x = Some(field),
                StyleField::ScaleY => style.scale_y = Some(field),
                StyleField::Spacing => style.spacing = Some(field),
                StyleField::Angle => style.angle = Some(field),
                StyleField::BorderStyle => {
                    style.border_style = Some(BorderStyle::from_str(&field)?);
                }
                StyleField::Outline => style.outline = Some(field),
                StyleField::Shadow => style.shadow = Some(field),
                StyleField::Alignment => style.alignment = Some(Alignment::from_str(&field)?),
                StyleField::MarginL => style.margin_l = Some(field),
                StyleField::MarginR => style.margin_r = Some(field),
                StyleField::MarginV => style.margin_v = Some(field),
                StyleField::Encoding => style.encoding = Some(field),
            }
        }

        Ok(style)
    }

    pub fn style_strict_from_line(&self, line: &str) -> anyhow::Result<StyleStrict> {
        let intermediate = self.style_from_line(line)?;
        StyleStrict::try_from(intermediate)
    }

    pub fn line_from_style_strict(&self, style: &StyleStrict) -> anyhow::Result<String> {
        let mut line = format!("{}: ", style.style_type);

        for field_type in &self.format {
            let s = match field_type {
                StyleField::Unknown(x) => format!(
                    "{}",
                    style
                        .unknown_fields
                        .get(x)
                        .context("unknown fields did not contain field: {field}")?
                ),
                StyleField::Name => format!("{}", style.name),
                StyleField::Fontname => format!("{}", style.fontname),
                StyleField::Fontsize => format!("{}", style.fontsize),
                StyleField::PrimaryColour => format!("{}", style.primary_color),
                StyleField::SecondaryColour => format!("{}", style.secondary_color),
                StyleField::OutlineColour => format!("{}", style.outline_color),
                StyleField::BackColour => format!("{}", style.back_color),
                StyleField::Bold => format!("{}", style.bold),
                StyleField::Italic => format!("{}", style.italic),
                StyleField::Underline => format!("{}", style.underline),
                StyleField::StrikeOut => format!("{}", style.strike_out),
                StyleField::ScaleX => format!("{}", style.scale_x),
                StyleField::ScaleY => format!("{}", style.scale_y),
                StyleField::Spacing => format!("{}", style.spacing),
                StyleField::Angle => format!("{}", style.angle),
                StyleField::BorderStyle => format!("{}", style.border_style),
                StyleField::Outline => format!("{}", style.outline),
                StyleField::Shadow => format!("{}", style.shadow),
                StyleField::Alignment => format!("{}", style.alignment),
                StyleField::MarginL => format!("{}", style.margin_l),
                StyleField::MarginR => format!("{}", style.margin_r),
                StyleField::MarginV => format!("{}", style.margin_v),
                StyleField::Encoding => format!("{}", style.encoding),
            };
            line.push_str(&s);
            line.push(',');
        }
        line.pop(); // remove trailing comma that we know we just added

        Ok(line)
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Style {
    unknown_fields: HashMap<String, String>,
    style_type: StyleType,
    name: Option<String>,
    fontname: Option<String>,
    fontsize: Option<String>,
    primary_color: Option<String>,
    secondary_color: Option<String>,
    outline_color: Option<String>,
    back_color: Option<String>,
    bold: Option<Boolean>,
    italic: Option<Boolean>,
    underline: Option<Boolean>,
    strike_out: Option<Boolean>,
    scale_x: Option<String>,
    scale_y: Option<String>,
    spacing: Option<String>,
    angle: Option<String>,
    border_style: Option<BorderStyle>,
    outline: Option<String>,
    shadow: Option<String>,
    alignment: Option<Alignment>,
    margin_l: Option<String>,
    margin_r: Option<String>,
    margin_v: Option<String>,
    encoding: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StyleStrict {
    unknown_fields: HashMap<String, String>,
    style_type: StyleType,
    name: String,
    fontname: String,
    fontsize: String,
    primary_color: String,
    secondary_color: String,
    outline_color: String,
    back_color: String,
    bold: Boolean,
    italic: Boolean,
    underline: Boolean,
    strike_out: Boolean,
    scale_x: String,
    scale_y: String,
    spacing: String,
    angle: String,
    border_style: BorderStyle,
    outline: String,
    shadow: String,
    alignment: Alignment,
    margin_l: String,
    margin_r: String,
    margin_v: String,
    encoding: String,
}

impl TryFrom<Style> for StyleStrict {
    type Error = anyhow::Error;

    fn try_from(value: Style) -> Result<Self, Self::Error> {
        Ok(Self {
            unknown_fields: value.unknown_fields,
            style_type: value.style_type,
            name: value.name.context("name not found")?,
            fontname: value.fontname.context("fontname not found")?,
            fontsize: value.fontsize.context("fontsize not found")?,
            primary_color: value.primary_color.context("PrimaryColor not found")?,
            secondary_color: value.secondary_color.context("SecondaryColor not found")?,
            outline_color: value.outline_color.context("OutlineColor not found")?,
            back_color: value.back_color.context("BackColor not found")?,
            bold: value.bold.context("Bold not found")?,
            italic: value.italic.context("Italic not found")?,
            underline: value.underline.context("Underline not found")?,
            strike_out: value.strike_out.context("StrikeOut not found")?,
            scale_x: value.scale_x.context("ScaleX not found")?,
            scale_y: value.scale_y.context("ScaleY not found")?,
            spacing: value.spacing.context("Spacing not found")?,
            angle: value.angle.context("Angle not found")?,
            border_style: value.border_style.context("BorderStyle not found")?,
            outline: value.outline.context("Outline not found")?,
            shadow: value.shadow.context("Shadow not found")?,
            alignment: value.alignment.context("Alignment not found")?,
            margin_l: value.margin_l.context("MarginL not found")?,
            margin_r: value.margin_r.context("MarginR not found")?,
            margin_v: value.margin_v.context("MarginV not found")?,
            encoding: value.encoding.context("Encoding not found")?,
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq, strum::Display, strum::EnumString)]
#[strum(serialize_all = "PascalCase")]
pub enum StyleType {
    #[strum(default)]
    Unknown(String),
    #[default]
    Style,
}

#[derive(Debug, Clone, PartialEq, strum::Display, strum::EnumString)]
pub enum BorderStyle {
    #[strum(default)]
    Unknown(String),
    #[strum(serialize = "1")]
    OutlineAndDropShadow,
    #[strum(serialize = "3")]
    OpaqueBox,
}

#[derive(Debug, Clone, PartialEq, strum::Display, strum::EnumString)]
pub enum Alignment {
    #[strum(default)]
    Unknown(String),
    #[strum(serialize = "1")]
    BottomLeft,
    #[strum(serialize = "2")]
    BottomCenter,
    #[strum(serialize = "3")]
    BottomRight,
    #[strum(serialize = "4")]
    MiddleLeft,
    #[strum(serialize = "5")]
    MiddleCenter,
    #[strum(serialize = "6")]
    MiddleRight,
    #[strum(serialize = "7")]
    TopLeft,
    #[strum(serialize = "8")]
    TopCenter,
    #[strum(serialize = "9")]
    TopRight,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::rstest;

    use super::*;

    const DEFAULT_STYLE_FORMAT: &str = r"Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding";

    #[rstest]
    #[case("Name", StyleField::Name)]
    #[case("MarginV", StyleField::MarginV)]
    #[case("FooBar", StyleField::Unknown("FooBar".to_string()))]
    fn test_style_field_from_str(#[case] got: &str, #[case] should: StyleField) {
        let result = StyleField::from_str(got).unwrap();
        assert_eq!(result, should);
    }

    #[rstest]
    #[case(StyleField::Name, "Name")]
    #[case(StyleField::MarginV, "MarginV")]
    #[case(StyleField::Unknown("FooBar".to_string()), "FooBar")]
    fn test_style_field_to_string(#[case] got: StyleField, #[case] should: &str) {
        let result = got.to_string();
        assert_eq!(result, should);
    }

    #[rstest]
    #[case(
        DEFAULT_STYLE_FORMAT,
        r"Style: Default,Roboto Medium,26,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,1.3,0,2,20,20,23,0",
    )]
    #[case(
        DEFAULT_STYLE_FORMAT,
        r"Style: zhu2,方正准圆_GBK,33,&H02FFFFFF,&H000000FF,&H00000000,&H00000000,-1,0,0,0,100,100,0,0,1,2,0.1,2,10,10,10,1",
    )]
    fn test_style_lossless(#[case] format: &str, #[case] line_before: &str) {
        let context = StyleContext::from_format_line(format).unwrap();
        let parsed = context.style_strict_from_line(line_before).unwrap();
        let line_after = context.line_from_style_strict(&parsed).unwrap();
        assert_eq!(line_after, line_before)
    }
}
