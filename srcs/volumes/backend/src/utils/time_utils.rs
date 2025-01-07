
use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};

use super::{Error, Result};

pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn format_time(time: OffsetDateTime) -> String {
    time.format(&Rfc3339).unwrap()
}

pub fn now_utc_plus_sec_to_str(offset: f64) -> String {
    let time = OffsetDateTime::now_utc() + Duration::seconds_f64(offset);
    format_time(time)
}

pub fn parse_time(moment: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339)
        .map_err(|_| Error::DateTimeParseFail(moment.to_string()))
}
