use self::rfc::Rfc;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use tera::{Context, Tera};

mod filters;
mod rfc;
mod rustc_version;
mod testers;

type IssueId = u32;

const INDEX_FILE: &str = "index.html";

pub fn generate() -> Result<(), Box<dyn Error>> {
    let mut tera = Tera::new("templates/**/*")?;
    tera.autoescape_on(vec![]);
    tera.register_filter("codify", filters::codify);
    tera.register_filter("pr_url", filters::pr_url);
    tera.register_filter("issue_url", filters::issue_url);
    tera.register_tester("in_stable", testers::in_stable);
    tera.register_tester("in_beta", testers::in_beta);
    let mut context = Context::new();
    context.insert("items", &read_items()?);
    context.insert("time", &Utc::now().to_rfc2822());
    let html = tera.render(INDEX_FILE, &context)?;
    fs::write(super::OUT_DIR.join(INDEX_FILE), html)?;
    Ok(())
}

fn read_items() -> Result<HashMap<String, Vec<Item>>, Box<dyn Error>> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/data.yml");
    let file = File::open(path)?;
    Ok(serde_yaml::from_reader(file)?)
}

#[derive(Debug, Deserialize, Serialize)]
struct Item {
    title: String,
    rfc: Option<Rfc>,
    repo: Option<String>,
    tracking: Option<IssueId>,
    stabilized: Option<Stabilization>,
    unresolved: Option<Rfc>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Stabilization {
    version: String,
    pr: IssueId,
}
