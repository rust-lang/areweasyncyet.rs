use super::IssueId;
use crate::query::{issue_or_pr, issues_with_label};
use reqwest::RequestBuilder;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Item {
    title: String,
    rfc: Option<String>,
    tracking: Option<IssueId>,
    issue_label: Option<String>,
    stabilized: Option<Stabilization>,
    unresolved: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Stabilization {
    version: String,
    pr: IssueId,
}

pub struct Converter<ReqBuildFunc> {
    build_req: ReqBuildFunc,
    cache: HashMap<IssueId, super::Issue>,
}

impl<ReqBuildFunc: Fn() -> RequestBuilder> Converter<ReqBuildFunc> {
    pub fn new(build_req: ReqBuildFunc) -> Self {
        Converter {
            build_req,
            cache: HashMap::new(),
        }
    }

    pub fn convert(&mut self, item: Item) -> Result<super::Item, Box<dyn Error>> {
        let issues = transpose(
            item.issue_label
                .as_ref()
                .map(|label| self.fetch_issues(&label)),
        )?;
        let tracking = transpose(item.tracking.map(|v| self.convert_issue(None, v)))?;
        Ok(super::Item {
            title: item.title,
            rfc: transpose(item.rfc.map(|v| self.convert_rfc(&v)))?,
            tracking,
            issue_label: item.issue_label,
            issues,
            stabilized: transpose(item.stabilized.map(|v| self.convert_stabilized(v)))?,
            unresolved: transpose(item.unresolved.map(|v| self.convert_rfc(&v)))?,
        })
    }

    fn convert_rfc(&mut self, rfc: &str) -> Result<super::Rfc, Box<dyn Error>> {
        let dash = rfc.find('-');
        let number = rfc[..dash.unwrap_or_else(|| rfc.len())].parse()?;
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
        let issue = self.convert_issue(Some("rust-lang/rfcs"), number)?;
        Ok(super::Rfc { issue, url, merged })
    }

    fn convert_stabilized(
        &mut self,
        stabilized: Stabilization,
    ) -> Result<super::Stabilization, Box<dyn Error>> {
        Ok(super::Stabilization {
            version: stabilized.version,
            pr: self.convert_issue(None, stabilized.pr)?,
        })
    }

    fn convert_issue(
        &mut self,
        repo: Option<&str>,
        id: IssueId,
    ) -> Result<super::Issue, Box<dyn Error>> {
        if let Some(issue) = self.cache.get(&id) {
            return Ok(issue.clone());
        }
        let (owner, name) = if let Some(repo) = repo {
            let slash = repo.find('/').expect("invalid repo");
            (&repo[..slash], &repo[slash + 1..])
        } else {
            ("rust-lang", "rust")
        };
        let issue = issue_or_pr::query(&self.build_req, owner, name, id)?;
        self.cache.insert(id, issue.clone());
        Ok(issue)
    }

    fn fetch_issues(&mut self, label: &str) -> Result<Vec<super::Issue>, Box<dyn Error>> {
        let issues = issues_with_label::query(&self.build_req, label)?;
        for issue in issues.iter() {
            self.cache.insert(issue.number, issue.clone());
        }
        Ok(issues)
    }
}

fn transpose<T, E>(v: Option<Result<T, E>>) -> Result<Option<T>, E> {
    match v {
        Some(Ok(v)) => Ok(Some(v)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}
