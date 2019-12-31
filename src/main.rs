use crate::data::input::InputData;
use crate::data::output::OutputData;
use crate::fetcher::IssueData;
use crate::page_gen::PageGenData;
use crate::query::{GitHubQuery, Repo};
use anyhow::{Context, Result};
use futures_util::future::try_join;
use once_cell::sync::Lazy;
use semver::Version;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

mod data;
mod fetcher;
mod page_gen;
mod posts;
mod query;

const DATA_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data.yml");
const POSTS_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/posts.yml");
const CACHE_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/cache.json");

static OUT_DIR: Lazy<&'static Path> = Lazy::new(|| Path::new("out"));
static RFC_REPO: Lazy<Repo> = Lazy::new(|| Repo::new("rust-lang", "rfcs"));
static RUSTC_REPO: Lazy<Repo> = Lazy::new(|| Repo::new("rust-lang", "rust"));

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    env_logger::init();
    let token = env::var("GITHUB_TOKEN")?;
    let client = reqwest::Client::new();
    let query = GitHubQuery::new(&client, &token);
    let data = load_page_gen_data(&query).await?;

    // Generate page
    if OUT_DIR.is_dir() {
        clear_dir(&*OUT_DIR).context("failed to clear out dir")?;
    } else {
        fs::create_dir_all(&*OUT_DIR).context("failed to create out dir")?;
    }
    page_gen::generate(&data).context("failed to generate data")?;
    copy_static_files().context("failed to copy static files")?;
    fs::copy(
        concat!(env!("CARGO_MANIFEST_DIR"), "/CNAME"),
        OUT_DIR.join("CNAME"),
    ).context("failed to copy CNAME")?;
    Ok(())
}

async fn load_page_gen_data(query: &GitHubQuery<'_>) -> Result<PageGenData> {
    let input_data = InputData::from_file(DATA_FILE).context("failed to read input data")?;
    let fetch_list = input_data.get_fetch_list();

    let mut issue_data = IssueData::from_file(CACHE_FILE).unwrap_or_default();
    let (latest_tag, _) = try_join(
        query.query_latest_tag(&*RUSTC_REPO),
        issue_data.fetch_data(query, &fetch_list),
    )
    .await?;
    issue_data.store_to_file(CACHE_FILE).context("failed to store to cache file")?;

    let latest_stable = Version::parse(&latest_tag)?;
    let output_data = OutputData::from_input(input_data, &issue_data, &latest_stable);

    Ok(PageGenData {
        items: output_data.0,
        posts: posts::load_posts().context("failed to load posts")?,
    })
}

fn clear_dir(dir: &Path) -> io::Result<()> {
    for entry in dir.read_dir()? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            fs::remove_file(&entry.path())?;
        } else if file_type.is_dir() {
            fs::remove_dir_all(&entry.path())?;
        } else {
            unreachable!("unknown file type");
        }
    }
    Ok(())
}

fn copy_static_files() -> io::Result<()> {
    let src = concat!(env!("CARGO_MANIFEST_DIR"), "/static");
    copy_dir(src.as_ref(), OUT_DIR.as_ref())
}

fn copy_dir(src: &Path, dest: &Path) -> io::Result<()> {
    for entry in src.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            fs::copy(path, dest.join(file_name))?;
        } else if file_type.is_dir() {
            let dest_dir = dest.join(file_name);
            fs::create_dir(&dest_dir)?;
            copy_dir(&path, &dest_dir)?;
        } else {
            unreachable!("unknown file type");
        }
    }
    Ok(())
}
