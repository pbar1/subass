#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod common;
mod event;
mod style;

use std::collections::HashMap;

use event::Events;
use style::Styles;

use crate::event::EventContext;
use crate::style::StyleContext;

#[derive(Default, Debug, Clone)]
pub struct AssScript {
    pub other_sections: HashMap<String, Vec<String>>,
    pub styles: Styles,
    pub events: Events,
}

impl AssScript {
    fn try_from_file(filename: &str) -> anyhow::Result<Self> {
        let mut script = AssScript::default();

        // initial state is before a section
        let mut section = "";

        let mut set_style_format = false;
        let mut set_event_format = false;

        for line in std::fs::read_to_string(filename)?.lines() {
            // ensure line doesn't have leading or trailing space
            let line = line.trim();

            // TODO: Collect comments in this struct as well
            // skip comments and newlines
            if line.starts_with(';') || line.is_empty() {
                continue;
            }

            // parse the sections
            if line.starts_with('[') && line.ends_with(']') {
                section = &line[1..line.len() - 1];
                continue;
            }

            match section {
                "V4+ Styles" => {
                    // first line must be format
                    if !set_style_format {
                        script.styles.context = StyleContext::from_format_line(line)?;
                        set_style_format = true;
                        continue;
                    }

                    let parsed = script.styles.context.style_strict_from_line(line)?;
                    script.styles.entries.push(parsed);
                }
                "Events" => {
                    // first line must be format
                    if !set_event_format {
                        script.events.context = EventContext::from_format_line(line)?;
                        set_event_format = true;
                        continue;
                    }

                    let parsed = script.events.context.event_strict_from_line(line)?;
                    script.events.entries.push(parsed);
                }
                _ => {
                    script
                        .other_sections
                        .entry(section.to_string())
                        .and_modify(|list| list.push(line.to_string()))
                        .or_insert_with(|| vec![line.to_string()]);
                }
            }
        }

        Ok(script)
    }
}

fn main() -> anyhow::Result<()> {
    let file_bot = "example.en.ass";
    // let file_top = "example.zh-TW.ass";

    let script_bot = AssScript::try_from_file(file_bot)?;
    // TODO: Print format line
    for x in script_bot.styles.entries {
        println!("{}", script_bot.styles.context.line_from_style_strict(&x)?);
    }
    // TODO: Print format line
    for x in script_bot.events.entries {
        println!("{}", script_bot.events.context.line_from_event_strict(&x)?);
    }

    // let script_top = AssScript::try_from_file(file_top)?;
    // dbg!(script_top);

    Ok(())
}
