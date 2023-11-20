use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Context;

/// Known fields in the [Events] section
#[derive(Debug, Clone, PartialEq, strum::Display, strum::EnumString)]
#[strum(serialize_all = "PascalCase")]
pub enum EventField {
    #[strum(default)]
    Unknown(String),
    Layer,
    Start,
    End,
    Style,
    Name,
    MarginL,
    MarginR,
    MarginV,
    Effect,
    Text,
}

#[derive(Debug, Clone)]
pub struct EventContext {
    format: Vec<EventField>,
}

impl EventContext {
    pub fn from_format_line(line: &str) -> anyhow::Result<Self> {
        let (_, fields) = line.split_once(':').context("unable to split on ':'")?;
        let format = fields
            .split(',')
            .map(|x| EventField::from_str(x.trim()))
            .collect::<Result<_, strum::ParseError>>()?;
        Ok(Self { format })
    }

    pub fn event_from_line(&self, line: &str) -> anyhow::Result<Event> {
        let (event_type, fields) = line.split_once(':').context("unable to split on ':'")?;

        let event_type = EventType::from_str(event_type)?;

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

        let mut event = Event {
            event_type,
            ..Default::default()
        };

        for (i, field) in fields.into_iter().enumerate() {
            let field_type = self
                .format
                .get(i)
                .context("no item found at event format index: {i}")?;

            match field_type {
                EventField::Unknown(x) => {
                    event.unknown_fields.insert(x.clone(), field);
                }
                EventField::Layer => event.layer = Some(field),
                EventField::Start => event.start = Some(field),
                EventField::End => event.end = Some(field),
                EventField::Style => event.style = Some(field),
                EventField::Name => event.name = Some(field),
                EventField::MarginL => event.margin_l = Some(field),
                EventField::MarginR => event.margin_r = Some(field),
                EventField::MarginV => event.margin_v = Some(field),
                EventField::Effect => event.effect = Some(field),
                EventField::Text => event.text = Some(field),
            }
        }

        Ok(event)
    }

    pub fn event_strict_from_line(&self, line: &str) -> anyhow::Result<EventStrict> {
        let intermediate = self.event_from_line(line)?;
        EventStrict::try_from(intermediate)
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Event {
    unknown_fields: HashMap<String, String>,
    event_type: EventType,
    layer: Option<String>,
    start: Option<String>,
    end: Option<String>,
    style: Option<String>,
    name: Option<String>,
    margin_l: Option<String>,
    margin_r: Option<String>,
    margin_v: Option<String>,
    effect: Option<String>,
    text: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventStrict {
    unknown_fields: HashMap<String, String>,
    event_type: EventType,
    layer: String,
    start: String,
    end: String,
    style: String,
    name: String,
    margin_l: String,
    margin_r: String,
    margin_v: String,
    effect: String,
    text: String,
}

impl TryFrom<Event> for EventStrict {
    type Error = anyhow::Error;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        Ok(Self {
            unknown_fields: value.unknown_fields,
            event_type: value.event_type,
            layer: value.layer.context("Layer not found")?,
            start: value.start.context("Start not found")?,
            end: value.end.context("End not found")?,
            style: value.style.context("Style not found")?,
            name: value.name.context("Name not found")?,
            margin_l: value.margin_l.context("MarginL not found")?,
            margin_r: value.margin_r.context("MarginR not found")?,
            margin_v: value.margin_v.context("MarginV not found")?,
            effect: value.effect.context("Effect not found")?,
            text: value.text.context("Text not found")?,
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq, strum::Display, strum::EnumString)]
#[strum(serialize_all = "PascalCase")]
pub enum EventType {
    #[strum(default)]
    Unknown(String),
    #[default]
    Dialogue,
    Comment,
    Picture,
    Movie,
    Sound,
    Command,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("Name", EventField::Name)]
    #[case("MarginV", EventField::MarginV)]
    #[case("FooBar", EventField::Unknown("FooBar".to_string()))]
    fn test_style_field_from_str(#[case] got: &str, #[case] should: EventField) {
        let result = EventField::from_str(got).unwrap();
        assert_eq!(result, should);
    }

    #[rstest]
    #[case(EventField::Name, "Name")]
    #[case(EventField::MarginV, "MarginV")]
    #[case(EventField::Unknown("FooBar".to_string()), "FooBar")]
    fn test_style_field_to_string(#[case] got: EventField, #[case] should: &str) {
        let result = got.to_string();
        assert_eq!(result, should);
    }
}
