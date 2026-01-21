use std::collections::HashMap;
use anyhow::Result;
use scraper::{Html, Selector};
use crate::model::ScheduleEvent;

pub type Schedule = HashMap<String, Vec<ScheduleEvent>>;

pub fn fetch_schedule() -> Result<Schedule> {
    let html = reqwest::blocking::get(
        "https://pinewoodbuilders.org/info/schedule"
    )?
    .text()?;

    let document = Html::parse_document(&html);
    let selector = Selector::parse(r#"script#schedule-data"#).unwrap();

    let script = document
        .select(&selector)
        .next()
        .ok_or_else(|| anyhow::anyhow!("schedule-data not found"))?;

    let json = script.inner_html();
    let raw: HashMap<String, HashMap<String, ScheduleEvent>> =
        serde_json::from_str(&json)?;

    let mut schedule = HashMap::new();
    for (group, events) in raw {
        schedule.insert(group, events.into_values().collect());
    }

    Ok(schedule)
}
