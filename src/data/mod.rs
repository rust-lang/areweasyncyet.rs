use serde::{Deserialize, Serialize};

pub mod input;
pub mod output;

pub type IssueId = u32;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Issue {
    pub number: u32,
    pub title: String,
    pub open: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    pub text: String,
    pub url: String,
}
