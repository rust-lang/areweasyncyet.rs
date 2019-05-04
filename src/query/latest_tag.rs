use self::query::{ResponseData, Variables};
use super::Repo;
use crate::query::GitHubQuery;
use graphql_client::GraphQLQuery;
use log::info;
use std::error::Error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/query/latest_tag.graphql",
    response_derives = "Debug"
)]
struct Query;

impl GitHubQuery<'_> {
    pub fn query_latest_tag(&self, repo: &Repo) -> Result<String, Box<dyn Error>> {
        info!("getting latest tag on {}...", repo);
        let query = Query::build_query(Variables {
            owner: repo.owner.clone(),
            name: repo.name.clone(),
        });
        let data: ResponseData = self.send_query("latest_tag", &query)?;
        let repository = data.repository.unwrap();
        let refs = repository.refs.unwrap();
        let mut nodes = refs.nodes.unwrap();
        let node = nodes.pop().unwrap().unwrap();
        Ok(node.name)
    }
}
