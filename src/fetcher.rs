use crate::data::input::FetchList;
use crate::data::{Issue, IssueId};
use crate::query::{GitHubQuery, Repo};
use serde::{Deserialize, Serialize};
use serde_with::rust::hashmap_as_tuple_list;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Default, Deserialize, Serialize)]
pub struct IssueData {
    #[serde(with = "hashmap_as_tuple_list")]
    pub labels: HashMap<(Repo, String), Vec<IssueId>>,
    #[serde(with = "hashmap_as_tuple_list")]
    pub issues: HashMap<(Repo, IssueId), Issue>,
}

impl IssueData {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn store_to_file(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    /// Fetch and fill into self when corresponding information does not exist.
    /// Nothing would be updated if everything is available.
    ///
    /// Returns whether anything is updated when succeeded.
    pub fn fetch_data(
        &mut self,
        query: &GitHubQuery,
        fetch_list: &FetchList,
    ) -> Result<bool, Box<dyn Error>> {
        let mut updated = false;
        for (repo, label) in fetch_list.labels.iter() {
            let key = (repo.clone(), label.to_string());
            if self.labels.contains_key(&key) {
                continue;
            }
            let issues = query.query_issues_with_label(repo, label)?;
            let issues = issues
                .iter()
                .map(|issue| {
                    let id = issue.number;
                    self.issues.insert((repo.clone(), id), issue.clone());
                    id
                })
                .collect();
            self.labels.insert(key, issues);
            updated = true;
        }
        for (repo, issue_id) in fetch_list.issues.iter() {
            let key = (repo.clone(), *issue_id);
            if self.issues.contains_key(&key) {
                continue;
            }
            let issue = query.query_issue_or_pr(repo, *issue_id)?;
            self.issues.insert(key, issue);
            updated = true;
        }
        Ok(updated)
    }
}
