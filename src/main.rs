#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod common;
mod event;
mod style;

use std::collections::HashMap;

use crate::event::EventContext;
use crate::style::StyleContext;

fn main() -> anyhow::Result<()> {
    let file_bottom = "example.en.ass";
    let file_top = "example.zh-TW.ass";

    do_main(file_bottom)?;
    println!();
    println!("------------------------------------------------------");
    println!();
    do_main(file_top)?;

    Ok(())
}

fn do_main(filename: &str) -> anyhow::Result<()> {
    let mut map: HashMap<Option<String>, Vec<String>> = HashMap::new();
    let mut section: Option<String> = None;

    for line in std::fs::read_to_string(filename)?.lines() {
        if line.starts_with('[') && line.ends_with(']') {
            section = Some(line[1..line.len() - 1].to_string());
            continue;
        }

        map.entry(section.clone())
            .and_modify(|e| e.push(line.to_string()))
            .or_insert_with(|| vec![line.to_string()]);
    }

    dbg!(map.keys());

    let key = Some("V4+ Styles".to_string());
    if let Some(lines) = map.get(&key) {
        let mut context: Option<StyleContext> = None;
        for line in lines {
            if line.starts_with(';') || line == "" {
                continue;
            }

            if context.is_none() {
                context = Some(StyleContext::from_format_line(line)?);
                continue;
            }

            if let Some(ref context) = context {
                let style = context.style_strict_from_line(line)?;
                let line2 = context.line_from_style_strict(&style)?;
                dbg!(&line2);
                assert_eq!(&line2, line);
            }
        }
    }

    let key = Some("Events".to_string());
    if let Some(lines) = map.get(&key) {
        let mut context: Option<EventContext> = None;
        for line in lines {
            if line.starts_with(';') || line == "" {
                continue;
            }

            if context.is_none() {
                context = Some(EventContext::from_format_line(line)?);
                continue;
            }

            if let Some(ref context) = context {
                let event = context.event_strict_from_line(line)?;
                let line2 = context.line_from_event_strict(&event)?;
                dbg!(&line2);
                assert_eq!(&line2, line);
            }
        }
    }

    Ok(())
}
