use crate::fetcher::IssueData;
use lazy_static::lazy_static;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io;
use std::path::Path;

mod data;
mod fetcher;
mod page_gen;
mod query;

const DATA_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data.yml");
const CACHE_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/cache.json");

lazy_static! {
    static ref OUT_DIR: &'static Path = Path::new("out");
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenv::dotenv();
    env_logger::init();
    let token = env::var("GITHUB_TOKEN")?;

    let input_data = data::input::read_data(File::open(DATA_FILE)?)?;
    let (labels, issues) = data::input::get_list_to_fetch(&input_data);

    let mut issue_data = load_cached_issue_data().unwrap_or_default();
    let client = reqwest::Client::new();
    let token = token.as_str();
    let build_req = || {
        client
            .post("https://api.github.com/graphql")
            .bearer_auth(token)
    };
    fetcher::fetch_data(build_req, &labels, &issues, &mut issue_data)?;
    store_issue_data(&issue_data)?;

    let output_data = data::output::generate(input_data, &issue_data);

    // Generate page
    if OUT_DIR.is_dir() {
        clear_dir(&*OUT_DIR)?;
    } else {
        fs::create_dir_all(&*OUT_DIR)?;
    }
    page_gen::generate(&output_data)?;
    copy_static_files()?;
    fs::copy(
        concat!(env!("CARGO_MANIFEST_DIR"), "/CNAME"),
        OUT_DIR.join("CNAME"),
    )?;
    Ok(())
}

fn load_cached_issue_data() -> Result<IssueData, Box<dyn Error>> {
    let file = File::open(CACHE_FILE)?;
    Ok(serde_json::from_reader(file)?)
}

fn store_issue_data(data: &IssueData) -> Result<(), Box<dyn Error>> {
    let file = File::create(CACHE_FILE)?;
    serde_json::to_writer(file, data)?;
    Ok(())
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
