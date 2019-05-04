use crate::data::output::Item;
use crate::posts::Post;
use chrono::Utc;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use tera::{Context, Tera};

mod filters;

const INDEX_FILE: &str = "index.html";

pub fn generate(items: &HashMap<String, Vec<Item>>, posts: &[Post]) -> Result<(), Box<dyn Error>> {
    let mut tera = Tera::new("templates/**/*.html")?;
    tera.register_filter("codify", filters::codify);
    tera.register_filter("pr_url", filters::pr_url);
    tera.register_filter("issue_url", filters::issue_url);
    let mut context = Context::new();
    context.insert("items", &items);
    context.insert("posts", &posts);
    context.insert("time", &Utc::now().to_rfc2822());
    let html = tera.render(INDEX_FILE, &context)?;
    fs::write(super::OUT_DIR.join(INDEX_FILE), html)?;
    Ok(())
}
