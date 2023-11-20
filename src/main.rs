#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod common;
mod event;
mod style;

use std::collections::HashMap;

use style::StyleContext;

use crate::event::EventContext;

fn main() -> anyhow::Result<()> {
    let file_bottom = "example.en.ass";

    let mut map: HashMap<Option<String>, Vec<String>> = HashMap::new();
    let mut section: Option<String> = None;

    for line in std::fs::read_to_string(file_bottom)?.lines() {
        if line.starts_with('[') && line.ends_with(']') {
            section = Some(line[1..line.len() - 1].to_string());
            continue;
        }

        map.entry(section.clone())
            .and_modify(|e| e.push(line.to_string()))
            .or_insert_with(|| vec![line.to_string()]);
    }

    let key = Some("V4+ Styles".to_string());
    if map.contains_key(&key) {
        let format_line = map.get(&key).unwrap().get(0).unwrap();
        let style_line = map.get(&key).unwrap().get(1).unwrap();

        let style_context = StyleContext::from_format_line(format_line)?;
        dbg!(&style_context);

        let style = style_context.style_strict_from_line(style_line)?;
        dbg!(&style);

        let style_line_2 = style_context.line_from_style_strict(&style)?;
        dbg!(&style_line_2);
    }

    let key = Some("Events".to_string());
    if map.contains_key(&key) {
        let format_line = map.get(&key).unwrap().get(0).unwrap();
        let event_line = map.get(&key).unwrap().get(1).unwrap();

        let event_context = EventContext::from_format_line(format_line)?;
        dbg!(&event_context);

        let event = event_context.event_strict_from_line(event_line)?;
        dbg!(&event);
    }

    Ok(())
}
