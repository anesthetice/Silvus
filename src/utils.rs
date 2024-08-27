// Imports
use time::{OffsetDateTime, UtcOffset};

pub fn get_local_datetime() -> OffsetDateTime {
    let local_offset = UtcOffset::current_local_offset().unwrap_or_else(|e| {
        eprintln!("Failed to get local offset, '{}'", e);
        UtcOffset::UTC
    });
    OffsetDateTime::now_utc().to_offset(local_offset)
}

pub fn datetime_to_pretty_string(dt: &OffsetDateTime) -> String {
    format!(
        "{:0>2}/{:0>2}/{:0>4} - {:0>2}:{:0>2}:{:0>2}",
        dt.day(),
        dt.month() as u8,
        dt.year(),
        dt.hour(),
        dt.minute(),
        dt.second(),
    )
}

pub fn datetime_to_path_string(dt: &OffsetDateTime) -> String {
    format!(
        "{:0>4}_{:0>2}_{:0>2}-{:0>2}_{:0>2}_{:0>2}",
        dt.year(),
        dt.month() as u8,
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second(),
    )
}
