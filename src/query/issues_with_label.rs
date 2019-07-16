use self::query::{IssueState, ResponseData, Variables};
use super::{GitHubQuery, Repo};
use crate::data::Issue;
use graphql_client::GraphQLQuery;
use log::info;
use matches::matches;
use std::error::Error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/query/issues_with_label.graphql",
    response_derives = "Debug"
)]
struct Query;

impl GitHubQuery<'_> {
    pub fn query_issues_with_label(
        &self,
        repo: &Repo,
        label: &str,
    ) -> Result<Vec<Issue>, Box<dyn Error>> {
        info!("fetching issues of label {} in {}...", label, repo);
        let mut result = Vec::new();
        let mut cursor = None;
        loop {
            let query = Query::build_query(Variables {
                owner: repo.owner.clone(),
                name: repo.name.clone(),
                label: label.to_string(),
                cursor,
            });
            let data: ResponseData = self.send_query("issues_with_labels", &query)?;
            let repository = data.repository.unwrap();
            let issues = repository.issues;
            let nodes = issues.nodes.unwrap();
            result.extend(nodes.into_iter().map(|issue| {
                let issue = issue.unwrap();
                Issue {
                    number: issue.number as u32,
                    title: issue.title,
                    open: matches!(issue.state, IssueState::OPEN),
                }
            }));
            let page_info = issues.page_info;
            if page_info.has_next_page {
                cursor = page_info.end_cursor;
            } else {
                break;
            }
        }
        Ok(result)
    }
}
