use crate::data::{Issue, IssueId};
use crate::query::{issue_or_pr, issues_with_label, Repo};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use serde_with::rust::hashmap_as_tuple_list;
use std::collections::HashMap;
use std::error::Error;

#[derive(Default, Deserialize, Serialize)]
pub struct IssueData {
    #[serde(with = "hashmap_as_tuple_list")]
    pub labels: HashMap<(Repo, String), Vec<IssueId>>,
    #[serde(with = "hashmap_as_tuple_list")]
    pub issues: HashMap<(Repo, IssueId), Issue>,
}

/// Fetch and fill into `data` when corresponding information does not exist.
/// Nothing would be updated if everything is available.
///
/// Returns whether anything is updated when succeeded.
pub fn fetch_data(
    build_req: impl Fn() -> RequestBuilder,
    labels: &[(Repo, &str)],
    issues: &[(Repo, IssueId)],
    data: &mut IssueData,
) -> Result<bool, Box<dyn Error>> {
    let mut updated = false;
    for (repo, label) in labels.iter() {
        let key = (repo.clone(), label.to_string());
        if data.labels.contains_key(&key) {
            continue;
        }
        let issues = issues_with_label::query(&build_req, repo, label)?;
        let issues = issues
            .iter()
            .map(|issue| {
                let id = issue.number;
                data.issues.insert((repo.clone(), id), issue.clone());
                id
            })
            .collect();
        data.labels.insert(key, issues);
        updated = true;
    }
    for (repo, issue_id) in issues.iter() {
        let key = (repo.clone(), *issue_id);
        if data.issues.contains_key(&key) {
            continue;
        }
        let issue = issue_or_pr::query(&build_req, repo, *issue_id)?;
        data.issues.insert(key, issue);
        updated = true;
    }
    Ok(updated)
}
