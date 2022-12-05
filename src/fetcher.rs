use crate::data::input::FetchList;
use crate::data::{Issue, IssueId};
use crate::query::{GitHubQuery, Repo};
use anyhow::{Error, Result};
use futures_util::future::ok;
use futures_util::stream::{FuturesUnordered, TryStreamExt};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

#[serde_as]
#[derive(Default, Deserialize, Serialize)]
pub struct IssueData {
    #[serde_as(as = "Vec<(_, _)>")]
    pub labels: HashMap<(Repo, String), Vec<IssueId>>,
    #[serde_as(as = "Vec<(_, _)>")]
    pub issues: HashMap<(Repo, IssueId), Issue>,
}

impl IssueData {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn store_to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let file = File::create(path)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    /// Fetch and fill into self when corresponding information does not exist.
    /// Nothing would be updated if everything is available.
    ///
    /// Returns whether anything is updated when succeeded.
    pub async fn fetch_data(
        &mut self,
        query: &GitHubQuery<'_>,
        fetch_list: &FetchList<'_>,
    ) -> Result<bool> {
        let mut updated = false;
        fetch_list
            .labels
            .iter()
            .filter_map(|(repo, label)| {
                let key = (repo.clone(), label.to_string());
                if self.labels.contains_key(&key) {
                    None
                } else {
                    Some(async {
                        let (repo, label) = &key;
                        let issues = query.query_issues_with_label(repo, label).await?;
                        Ok::<_, Error>((key, issues))
                    })
                }
            })
            .collect::<FuturesUnordered<_>>()
            .try_for_each_concurrent(None, |(key, issues)| {
                let (repo, _) = &key;
                let issues = issues
                    .into_iter()
                    .map(|issue| {
                        let id = issue.number;
                        self.issues.insert((repo.clone(), id), issue);
                        id
                    })
                    .collect();
                self.labels.insert(key, issues);
                updated = true;
                ok(())
            })
            .await?;

        fetch_list
            .issues
            .iter()
            .filter_map(|(repo, issue_id)| {
                let key = (repo.clone(), *issue_id);
                if self.issues.contains_key(&key) {
                    None
                } else {
                    Some(async {
                        let (repo, issue_id) = &key;
                        let issue = query.query_issue_or_pr(repo, *issue_id).await?;
                        Ok::<_, Error>((key, issue))
                    })
                }
            })
            .collect::<FuturesUnordered<_>>()
            .try_for_each_concurrent(None, |(key, issue)| {
                self.issues.insert(key, issue);
                updated = true;
                ok(())
            })
            .await?;

        Ok(updated)
    }
}
