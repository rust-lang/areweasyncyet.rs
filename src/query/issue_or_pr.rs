use self::query::QueryRepositoryIssueOrPullRequest as IssueOrPr;
use self::query::{IssueState, PullRequestState};
use self::query::{ResponseData, Variables};
use super::{GitHubQuery, Repo};
use crate::data::{Issue, IssueId};
use graphql_client::GraphQLQuery;
use log::info;
use matches::matches;
use std::error::Error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/query/issue_or_pr.graphql",
    response_derives = "Debug"
)]
struct Query;

impl GitHubQuery<'_> {
    pub fn query_issue_or_pr(&self, repo: &Repo, number: IssueId) -> Result<Issue, Box<dyn Error>> {
        info!("fetching issue {}#{}...", repo, number);
        let query = Query::build_query(Variables {
            owner: repo.owner.clone(),
            name: repo.name.clone(),
            number: i64::from(number),
        });
        let data: ResponseData = self.send_query("issue_or_pr", &query)?;
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
}
