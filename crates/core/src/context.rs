use std::path::Path;

use anyhow::Result;
use chrono::{Datelike, Local, Timelike, Utc};
use serde_json::{Value, json};

use crate::config::{Formats, Template};

pub fn create(
    formats: &Formats,
    template: &Template,
    document: Value,
    position: Value,
    project_root: Option<&Path>,
) -> Result<Value> {
    let now = if formats.timezone == "UTC" {
        Utc::now().fixed_offset()
    } else {
        Local::now().fixed_offset()
    };
    let host = hostname::get()?.to_string_lossy().into_owned();
    let project_name = project_root
        .and_then(Path::file_name)
        .map(|name| name.to_string_lossy());
    Ok(json!({
        "schema_version": 1,
        "variable": { "name": null },
        "now": {
            "unix_ms": now.timestamp_millis(),
            "timezone": now.offset().to_string(),
            "year": now.year(), "month": now.month(), "day": now.day(),
            "hour": now.hour(), "minute": now.minute(), "second": now.second(),
            "weekday": now.weekday().number_from_monday()
        },
        "host": { "name": host },
        "project": { "root": project_root, "name": project_name },
        "document": document,
        "position": position,
        "template": { "id": template.id, "trigger": template.trigger },
        "builtins": {
            "date": now.format(&formats.date).to_string(),
            "time": now.format(&formats.time).to_string(),
            "host": host
        },
        "symbol": null,
        "selection": null
    }))
}
