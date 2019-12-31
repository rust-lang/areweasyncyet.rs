use crate::POSTS_FILE;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub title: String,
    pub date: String,
    pub url: String,
}

pub fn load_posts() -> Result<Vec<Post>> {
    let file = File::open(POSTS_FILE)?;
    Ok(serde_yaml::from_reader(file)?)
}
