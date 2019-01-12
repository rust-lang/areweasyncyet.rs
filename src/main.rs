use lazy_static::lazy_static;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;

mod data;
mod page_gen;

lazy_static! {
    static ref OUT_DIR: &'static Path = Path::new("out");
}

fn main() -> Result<(), Box<dyn Error>> {
    let items = data::read_items()?;
    if OUT_DIR.is_dir() {
        clear_dir(&*OUT_DIR)?;
    } else {
        fs::create_dir_all(&*OUT_DIR)?;
    }
    page_gen::generate(&items)?;
    copy_static_files()?;
    fs::copy(
        concat!(env!("CARGO_MANIFEST_DIR"), "/CNAME"),
        OUT_DIR.join("CNAME"),
    )?;
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
