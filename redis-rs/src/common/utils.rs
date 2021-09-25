use std::str::FromStr;

use chrono::{DateTime, Duration, Local, TimeZone};

use super::error::{Error, ErrorKind};

pub fn parse_str<I, O>(value: &I) -> Option<O>
where
    I: AsRef<str>,
    O: FromStr,
{
    return value.as_ref().parse().ok();
}

pub fn parse_millis<I>(value: I) -> Result<DateTime<Local>, super::error::Error>
where
    I: AsRef<str>,
{
    let time = Local.timestamp_millis(
        value
            .as_ref()
            .parse::<i64>()
            .map_err(|e| Error::new(ErrorKind::Parser, &e.to_string()))?,
    );
    return Ok(time);
}

pub fn elasped(start: DateTime<Local>) -> Duration {
    return Local::now() - start;
}
