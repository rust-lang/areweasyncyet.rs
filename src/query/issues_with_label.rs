use self::query::{IssueState, ResponseData, Variables};
use super::QueryError;
use crate::data::Issue;
use graphql_client::{GraphQLQuery, Response};
use log::info;
use matches::matches;
use reqwest::RequestBuilder;
use std::error::Error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/query/issues_with_label.graphql",
    response_derives = "Debug"
)]
struct Query;

pub fn query(
    build_req: impl Fn() -> RequestBuilder,
    label: &str,
) -> Result<Vec<Issue>, Box<dyn Error>> {
    info!("fetching issues of label {}...", label);
    let mut result = Vec::new();
    let mut cursor = None;
    loop {
        let query = Query::build_query(Variables {
            label: label.to_string(),
            cursor,
        });
        let resp = build_req()
            .json(&query)
            .send()?
            .json::<Response<ResponseData>>()?;
        if let Some(errors) = resp.errors {
            Err(QueryError {
                name: "issues_with_label",
                errors,
            })?;
        }
        let data = resp.data.unwrap();
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
