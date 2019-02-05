use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

pub mod issue_or_pr;
pub mod issues_with_label;

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
