use chrono::{Duration, NaiveDate};
use combine::Parser;
use combine::error::StringStreamError;
use combine::parser::char::{char, digit};
use combine::range::recognize;
use combine::skip_many1;
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

impl FromStr for RustcVersion {
    type Err = StringStreamError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        (number(), char('.'), number())
            .map(|(major, _, minor)| {
                RustcVersion { major, minor }
            })
            .parse(s)
            .and_then(|(version, remaining)| {
                if remaining.is_empty() {
                    Ok(version)
                } else {
                    Err(StringStreamError::UnexpectedParse)
                }
            })
    }
}

fn number<'a>() -> impl Parser<Input = &'a str, Output = u32> {
    recognize(skip_many1(digit()))
        .and_then(|s: &str| s.parse().map_err(|_| StringStreamError::UnexpectedParse))
}
