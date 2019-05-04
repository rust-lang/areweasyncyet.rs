use super::{IssueId, RFC_REPO, RUSTC_REPO};
use crate::query::Repo;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub struct InputData(pub HashMap<String, Vec<Item>>);

#[derive(Debug, Deserialize)]
pub struct Item {
    pub title: String,
    pub rfc: Option<String>,
    pub tracking: Option<IssueId>,
    pub issue_label: Option<String>,
    pub stabilized: Option<Stabilization>,
    pub unresolved: Option<String>,
    #[serde(default)]
    pub deps: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Stabilization {
    pub version: String,
    pub pr: IssueId,
}

impl InputData {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let data = serde_yaml::from_reader(file)?;
        Ok(InputData(data))
    }

    pub fn get_fetch_list(&self) -> FetchList<'_> {
        let mut fetch_list = FetchList::default();
        self.0
            .values()
            .flatten()
            .for_each(|item| fetch_list.fill_from_item(item));
        fetch_list
    }
}

#[derive(Default)]
pub struct FetchList<'a> {
    pub labels: Vec<(Repo, &'a str)>,
    pub issues: Vec<(Repo, IssueId)>,
}

impl<'a> FetchList<'a> {
    fn fill_from_item(&mut self, item: &'a Item) {
        if let Some(rfc) = &item.rfc {
            self.issues.push((RFC_REPO.clone(), parse_rfc_for_id(&rfc)));
        }
        if let Some(tracking) = &item.tracking {
            self.issues.push((RUSTC_REPO.clone(), *tracking));
        }
        if let Some(label) = &item.issue_label {
            self.labels.push((RUSTC_REPO.clone(), label.as_str()));
        }
        if let Some(stabilized) = &item.stabilized {
            self.issues.push((RUSTC_REPO.clone(), stabilized.pr));
        }
        if let Some(unresolved) = &item.unresolved {
            self.issues
                .push((RFC_REPO.clone(), parse_rfc_for_id(&unresolved)));
        }
        item.deps.iter().for_each(|dep| self.fill_from_item(dep));
    }
}

fn parse_rfc_for_id(rfc: &str) -> IssueId {
    let dash = rfc.find('-').unwrap_or_else(|| rfc.len());
    rfc[..dash].parse().expect("unexpected rfc number")
}
