use self::query::QueryRepositoryIssueOrPullRequest as IssueOrPr;
use self::query::{IssueState, PullRequestState};
use self::query::{ResponseData, Variables};
use super::QueryError;
use crate::data::{Issue, IssueId};
use graphql_client::{GraphQLQuery, Response};
use log::info;
use matches::matches;
use reqwest::RequestBuilder;
use std::error::Error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/query/issue_or_pr.graphql",
    response_derives = "Debug"
)]
struct Query;

pub fn query(
    build_req: impl Fn() -> RequestBuilder,
    owner: &str,
    name: &str,
    number: IssueId,
) -> Result<Issue, Box<dyn Error>> {
    info!("fetching issue {}/{}#{}...", owner, name, number);
    let query = Query::build_query(Variables {
        owner: owner.to_string(),
        name: name.to_string(),
        number: i64::from(number),
    });
    let resp = build_req()
        .json(&query)
        .send()?
        .json::<Response<ResponseData>>()?;
    if let Some(errors) = resp.errors {
        Err(QueryError {
            name: "issue_or_pr",
            errors,
        })?;
    }
    let data = resp.data.unwrap();
    let repository = data.repository.unwrap();
    match repository.issue_or_pull_request.unwrap() {
        IssueOrPr::Issue(issue) => Ok(Issue {
            number,
            title: issue.title,
            open: matches!(issue.issue_state, IssueState::OPEN),
        }),
        IssueOrPr::PullRequest(pr) => Ok(Issue {
            number,
            title: pr.title,
            open: matches!(pr.pr_state, PullRequestState::OPEN),
        }),
    }
}
