use crate::data::rustc_version::RustcVersion;
use chrono::Utc;
use lazy_static::lazy_static;
use tera::{self, Value};

lazy_static! {
    static ref CURRENT_STABLE: RustcVersion = {
        let today = Utc::today().naive_utc();
        RustcVersion::stable_at(today)
    };
    static ref CURRENT_BETA: RustcVersion = {
        let stable = &*CURRENT_STABLE;
        RustcVersion {
            major: stable.major,
            minor: stable.minor + 1,
        }
    };
}

pub fn in_stable(value: Option<Value>, _: Vec<Value>) -> tera::Result<bool> {
    let version = parse_version(value)?;
    Ok(version <= *CURRENT_STABLE)
}

pub fn in_beta(value: Option<Value>, _: Vec<Value>) -> tera::Result<bool> {
    let version = parse_version(value)?;
    Ok(version == *CURRENT_BETA)
}

fn parse_version(value: Option<Value>) -> tera::Result<RustcVersion> {
    let version = match value {
        Some(Value::String(s)) => s,
        _ => Err(format!("unknown type for version: {:?}", value))?,
    };
    Ok(version.parse().map_err(|_| format!("failed to parse version: {}", version))?)
}
