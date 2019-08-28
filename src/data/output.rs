use super::input::{InputData, Item as InputItem};
use super::{Issue, IssueId, Link};
use crate::fetcher::IssueData;
use crate::query::Repo;
use crate::{RFC_REPO, RUSTC_REPO};
use semver::Version;
use serde::Serialize;
use std::collections::HashMap;

pub struct OutputData(pub HashMap<String, Vec<Item>>);

impl OutputData {
    pub fn from_input(input: InputData, issue_data: &IssueData, latest_stable: &Version) -> Self {
        let builder = Builder {
            issue_data,
            latest_stable,
        };
        builder.build(input)
    }
}

#[derive(Debug, Serialize)]
pub struct Item {
    pub title: String,
    pub rfc: Option<Rfc>,
    pub tracking: Option<Issue>,
    pub issue_label: Option<String>,
    pub issues: Vec<Issue>,
    pub stabilized: Option<Stabilization>,
    pub unresolved: Option<Rfc>,
    pub link: Option<Link>,
    pub deps: Vec<Item>,
}

#[derive(Debug, Serialize)]
pub struct Rfc {
    issue: Issue,
    url: String,
    merged: bool,
}

#[derive(Debug, Serialize)]
pub struct Stabilization {
    pub state: VersionState,
    pub version: String,
    pub pr: Issue,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VersionState {
    Stable,
    Beta,
    Nightly,
}

struct Builder<'a> {
    issue_data: &'a IssueData,
    latest_stable: &'a Version,
}

impl Builder<'_> {
    fn build(&self, input: InputData) -> OutputData {
        let result = input
            .0
            .into_iter()
            .map(|(key, items)| (key, self.convert_items(items)))
            .collect();
        OutputData(result)
    }

    fn convert_items(&self, items: Vec<InputItem>) -> Vec<Item> {
        items
            .into_iter()
            .map(|item| self.convert_item(item))
            .collect()
    }

    fn convert_item(&self, item: InputItem) -> Item {
        Item {
            title: item.title,
            rfc: self.convert_rfc(item.rfc),
            tracking: self.get_optional_issue(&*RUSTC_REPO, item.tracking),
            issues: item
                .issue_label
                .as_ref()
                .map(|label| {
                    self.issue_data
                        // TODO Don't clone?
                        .labels[&(RUSTC_REPO.clone(), label.clone())]
                        .iter()
                        .map(|id| self.get_issue(&*RUSTC_REPO, *id))
                        .collect()
                })
                .unwrap_or_default(),
            issue_label: item.issue_label,
            stabilized: item.stabilized.map(|stabilized| Stabilization {
                state: self.get_version_state(&stabilized.version),
                version: stabilized.version,
                pr: self.get_issue(&*RUSTC_REPO, stabilized.pr),
            }),
            unresolved: self.convert_rfc(item.unresolved),
            link: item.link,
            deps: self.convert_items(item.deps),
        }
    }

    fn get_version_state(&self, version: &str) -> VersionState {
        let version =
            Version::parse(&format!("{}.0", version)).expect("invalid stabilization version");
        if *self.latest_stable >= version {
            return VersionState::Stable;
        }
        let mut beta = self.latest_stable.clone();
        beta.increment_minor();
        if beta >= version {
            return VersionState::Beta;
        }
        VersionState::Nightly
    }

    fn convert_rfc(&self, rfc: Option<String>) -> Option<Rfc> {
        let rfc = rfc?;
        let dash = rfc.find('-');
        let number = rfc[..dash.unwrap_or_else(|| rfc.len())]
            .parse()
            .expect("unexpected rfc number");
        let (url, merged) = if dash.is_none() {
            (
                format!("https://github.com/rust-lang/rfcs/pull/{}", rfc),
                false,
            )
        } else {
            let hash = rfc.find('#').unwrap_or_else(|| rfc.len());
            let (page, frag) = rfc.split_at(hash);
            (
                format!("https://rust-lang.github.io/rfcs/{}.html{}", page, frag),
                true,
            )
        };
        let issue = self.get_issue(&*RFC_REPO, number);
        Some(Rfc { issue, url, merged })
    }

    fn get_optional_issue(&self, repo: &Repo, id: Option<IssueId>) -> Option<Issue> {
        id.map(|id| self.get_issue(repo, id))
    }

    fn get_issue(&self, repo: &Repo, id: IssueId) -> Issue {
        // TODO Don't clone?
        self.issue_data.issues[&(repo.clone(), id)].clone()
    }
}
