use crate::data::Item;
use chrono::Utc;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use tera::{Context, Tera};

mod filters;
mod testers;

const INDEX_FILE: &str = "index.html";

pub fn generate(items: &HashMap<String, Vec<Item>>) -> Result<(), Box<dyn Error>> {
    let mut tera = Tera::new("templates/**/*")?;
    tera.autoescape_on(vec![]);
    tera.register_filter("codify", filters::codify);
    tera.register_filter("pr_url", filters::pr_url);
    tera.register_filter("issue_url", filters::issue_url);
    tera.register_tester("in_stable", testers::in_stable);
    tera.register_tester("in_beta", testers::in_beta);
    let mut context = Context::new();
    context.insert("items", &items);
    context.insert("time", &Utc::now().to_rfc2822());
    let html = tera.render(INDEX_FILE, &context)?;
    fs::write(super::OUT_DIR.join(INDEX_FILE), html)?;
    Ok(())
}
