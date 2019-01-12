use chrono::{Duration, NaiveDate};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RustcVersion {
    pub major: u32,
    pub minor: u32,
}

impl RustcVersion {
    pub fn stable_at(date: NaiveDate) -> RustcVersion {
        let epoch_date = NaiveDate::from_ymd(2015, 12, 11);
        let epoch_release = 5;
        let release_duration = Duration::weeks(6);
        let releases = (date - epoch_date).num_days() / release_duration.num_days();
        RustcVersion {
            major: 1,
            minor: epoch_release + releases as u32,
        }
    }
}

pub enum ParsingError {
    NoDot,
    Int(ParseIntError),
}

impl FromStr for RustcVersion {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dot = s.find('.').ok_or(ParsingError::NoDot)?;
        let major = s[..dot].parse().map_err(ParsingError::Int)?;
        let minor = s[dot + 1..].parse().map_err(ParsingError::Int)?;
        Ok(RustcVersion { major, minor })
    }
}
