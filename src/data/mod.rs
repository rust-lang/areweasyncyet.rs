use crate::query::Repo;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

pub mod input;
pub mod output;
pub mod rustc_version;

lazy_static! {
    static ref RFC_REPO: Repo = Repo::new("rust-lang", "rfcs");
    static ref RUSTC_REPO: Repo = Repo::new("rust-lang", "rust");
}

pub type IssueId = u32;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Issue {
    pub number: u32,
    pub title: String,
    pub open: bool,
}
