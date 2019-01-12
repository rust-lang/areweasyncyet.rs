use self::internal::Converter;
use reqwest::Client;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

mod internal;
pub mod rustc_version;

pub type IssueId = u32;

#[derive(Debug, Serialize)]
pub struct Item {
    pub title: String,
    pub rfc: Option<Rfc>,
    pub repo: Option<String>,
    pub tracking: Option<Issue>,
    pub issue_label: Option<String>,
    pub issues: Option<Vec<Issue>>,
    pub stabilized: Option<Stabilization>,
    pub unresolved: Option<Rfc>,
}

#[derive(Debug, Serialize)]
pub struct Rfc {
    issue: Issue,
    url: String,
    merged: bool,
}

#[derive(Debug, Serialize)]
pub struct Stabilization {
    pub version: String,
    pub pr: Issue,
}

#[derive(Clone, Debug, Serialize)]
pub struct Issue {
    pub number: u32,
    pub title: String,
    pub open: bool,
}

pub fn generate_data(
    client: &Client,
    token: &str,
) -> Result<HashMap<String, Vec<Item>>, Box<dyn Error>> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/data.yml");
    let file = File::open(path)?;
    let data: HashMap<_, Vec<internal::Item>> = serde_yaml::from_reader(file)?;
    let mut converter = Converter::new(client, token);
    data.into_iter()
        .map(|(group, items)| {
            let items = items
                .into_iter()
                .map(|item| converter.convert(item))
                .collect::<Result<Vec<Item>, _>>()?;
            Ok((group, items))
        })
        .collect()
}
