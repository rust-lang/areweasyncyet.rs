use graphql_client::Response;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{self, Display};

mod issue_or_pr;
mod issues_with_label;
mod latest_tag;

#[derive(Clone, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Repo {
    pub owner: String,
    pub name: String,
}

impl Repo {
    pub fn new(owner: &str, name: &str) -> Self {
        Self {
            owner: owner.to_string(),
            name: name.to_string(),
        }
    }
}

impl Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

pub struct GitHubQuery<'a> {
    client: &'a Client,
    token: &'a str,
}

impl<'a> GitHubQuery<'a> {
    pub fn new(client: &'a Client, token: &'a str) -> Self {
        GitHubQuery { client, token }
    }

    fn send_query<Q, D>(&self, name: &'static str, query: &Q) -> Result<D, Box<dyn Error>>
    where
        Q: Serialize,
        for<'de> Response<D>: Deserialize<'de>,
    {
        let resp = self
            .client
            .post("https://api.github.com/graphql")
            .bearer_auth(self.token)
            .json(query)
            .send()?
            .json::<Response<D>>()?;
        if let Some(errors) = resp.errors {
            return Err(QueryError { name, errors }.into());
        }
        Ok(resp.data.unwrap())
    }
}

#[derive(Debug)]
struct QueryError {
    name: &'static str,
    errors: Vec<graphql_client::Error>,
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "query '{}' fails:", self.name)?;
        for error in self.errors.iter() {
            writeln!(f, "{}", error)?;
        }
        Ok(())
    }
}

impl Error for QueryError {}
