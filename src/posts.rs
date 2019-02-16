use crate::POSTS_FILE;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub title: String,
    pub date: String,
    pub url: String,
}

pub fn load_posts() -> Result<Vec<Post>, Box<dyn Error>> {
    let file = File::open(POSTS_FILE)?;
    Ok(serde_yaml::from_reader(file)?)
}
