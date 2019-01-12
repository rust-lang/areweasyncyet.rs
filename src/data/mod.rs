use self::rfc::Rfc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

pub mod rfc;
pub mod rustc_version;

pub type IssueId = u32;

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub title: String,
    pub rfc: Option<Rfc>,
    pub repo: Option<String>,
    pub tracking: Option<IssueId>,
    pub stabilized: Option<Stabilization>,
    pub unresolved: Option<Rfc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stabilization {
    pub version: String,
    pub pr: IssueId,
}

pub fn read_items() -> Result<HashMap<String, Vec<Item>>, Box<dyn Error>> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/data.yml");
    let file = File::open(path)?;
    Ok(serde_yaml::from_reader(file)?)
}
