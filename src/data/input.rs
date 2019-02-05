use super::{IssueId, RFC_REPO, RUSTC_REPO};
use crate::query::Repo;
use serde::Deserialize;
use serde_yaml::Result;
use std::collections::HashMap;
use std::io::Read;

pub type InputData = HashMap<String, Vec<Item>>;

#[derive(Debug, Deserialize)]
pub struct Item {
    pub title: String,
    pub rfc: Option<String>,
    pub tracking: Option<IssueId>,
    pub issue_label: Option<String>,
    pub stabilized: Option<Stabilization>,
    pub unresolved: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Stabilization {
    pub version: String,
    pub pr: IssueId,
}

pub fn read_data(reader: impl Read) -> Result<InputData> {
    serde_yaml::from_reader(reader)
}

pub fn get_list_to_fetch(data: &InputData) -> (Vec<(Repo, &str)>, Vec<(Repo, IssueId)>) {
    let mut labels = Vec::new();
    let mut issues = Vec::new();
    for item in data.values().flatten() {
        if let Some(rfc) = &item.rfc {
            issues.push((RFC_REPO.clone(), parse_rfc_for_id(&rfc)));
        }
        if let Some(tracking) = &item.tracking {
            issues.push((RUSTC_REPO.clone(), *tracking));
        }
        if let Some(label) = &item.issue_label {
            labels.push((RUSTC_REPO.clone(), label.as_str()));
        }
        if let Some(stabilized) = &item.stabilized {
            issues.push((RUSTC_REPO.clone(), stabilized.pr));
        }
        if let Some(unresolved) = &item.unresolved {
            issues.push((RFC_REPO.clone(), parse_rfc_for_id(&unresolved)));
        }
    }
    (labels, issues)
}

fn parse_rfc_for_id(rfc: &str) -> IssueId {
    let dash = rfc.find('-').unwrap_or_else(|| rfc.len());
    rfc[..dash].parse().expect("unexpected rfc number")
}
