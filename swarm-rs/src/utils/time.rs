use std::time::SystemTime;

use chrono::{DateTime, Utc};

pub fn system_time_to_iso(time: SystemTime) -> String {
    let time: DateTime<Utc> = time.into();
    time.to_rfc3339()
}

pub fn current_time_iso() -> String {
    let current_time = SystemTime::now();
    let time: DateTime<Utc> = current_time.into();
    time.to_rfc3339()
}

pub fn today() -> String {
    let current_time = SystemTime::now();
    let time: DateTime<Utc> = current_time.into();
    let formatted_date = time.format("%Y-%m-%d").to_string();
    formatted_date
}

pub fn today_with_format(format: &str) -> String {
    let current_time = SystemTime::now();
    let time: DateTime<Utc> = current_time.into();
    let formatted_date = time.format(format).to_string();
    formatted_date
}
