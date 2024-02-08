use chrono::{DateTime, Local, Weekday};
use serde_json::json;
use std::fs;
use anyhow::Result;
use handlebars::Handlebars;

pub fn daily(day :DateTime<Local>) -> Result<String> {
    let title = format!("# {}", day.format("%Y%W%a"));
    let register = Handlebars::new();

    let result = register.render_template(
        &fs::read_to_string("daily_template.md")?,
        &json!({
            "title" : title,
        })
        )?;
    Ok(result)
}
