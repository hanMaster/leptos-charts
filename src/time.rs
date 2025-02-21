use crate::error::Result;
use chrono::prelude::*;

pub fn now_timestamp() -> Result<String> {
    let ts = Utc::now().timestamp_millis().to_string();
    Ok(ts)
}

pub fn timestamp_to_time(ts: &str, interval: &str) -> Result<String> {
    let ts = ts.parse::<i64>()?;
    let datetime = DateTime::from_timestamp_millis(ts).unwrap();

    let timestamp_str = match interval {
        "1" | "5" | "15" | "30" | "60" => datetime.format("%H:%M").to_string(),
        "240" => datetime.format("%b %d").to_string(),
        "D" => datetime.format("%b %d").to_string(),
        "W" => datetime.format("%Y %b").to_string(),
        "M" => datetime.format("%Y %m").to_string(),
        _ => datetime.format("%Y-%m-%d %H:%M").to_string(),
    };

    Ok(timestamp_str)
}

pub fn get_interval_label(interval: &str) -> String {
    match interval {
        "1" => "1 min".to_string().to_string(),
        "5" => "5 min".to_string(),
        "15" => "15 min".to_string(),
        "30" => "30 min".to_string(),
        "60" => "1 hour".to_string(),
        "240" => "4 hours".to_string(),
        "D" => "Day".to_string(),
        "W" => "Week".to_string(),
        "M" => "Month".to_string(),
        _ => "".to_string(),
    }
}
