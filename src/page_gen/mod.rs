use crate::data::output::Item;
use crate::posts::Post;
use anyhow::Result;
use chrono::Utc;
use std::collections::HashMap;
use std::fs;
use tera::{Context, Tera};

mod filters;

const INDEX_FILE: &str = "index.html";

pub struct PageGenData {
    pub items: HashMap<String, Vec<Item>>,
    pub posts: Vec<Post>,
}

pub fn generate(data: &PageGenData) -> Result<()> {
    let mut tera = Tera::new("templates/**/*.html")?;
    tera.register_filter("codify", filters::codify);
    tera.register_filter("pr_url", filters::pr_url);
    tera.register_filter("issue_url", filters::issue_url);
    let mut context = Context::new();
    context.insert("items", &data.items);
    context.insert("posts", &data.posts);
    context.insert("time", &Utc::now().to_rfc2822());
    let html = tera.render(INDEX_FILE, &context)?;
    fs::write(super::OUT_DIR.join(INDEX_FILE), html)?;
    Ok(())
}
