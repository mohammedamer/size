use regex::Regex;
use std::error::Error;
use std::fmt::Write;
use std::path::Path;
use std::{fs, io};
use walkdir::WalkDir;

fn dir_size(path: &Path) -> Result<u64, Box<dyn Error>> {
    let mut size = 0;

    for entry in WalkDir::new(path) {
        let entry = entry?;
        size += entry.metadata()?.len();
    }

    Ok(size)
}

pub fn total_size(root: &str, pattern: &str) -> Result<u64, Box<dyn Error>> {
    if Path::new(&root).is_file() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Root can't be a file!",
        )));
    }

    let re = Regex::new(pattern)?;

    let mut agg_size = 0;

    for entry in fs::read_dir(root)? {
        let entry = entry?;

        if let Some(filename) = entry.file_name().to_str() {
            if re.is_match(filename) {
                if let Some(path) = entry.path().to_str() {
                    let mut size = 0;
                    if entry.file_type()?.is_file() {
                        size = entry.metadata()?.len();
                    } else if entry.file_type()?.is_dir() {
                        size = dir_size(&entry.path())?;
                    }

                    agg_size += size;

                    println!("path: {path}\nsize: {size} bytes");
                }
            } else if entry.file_type()?.is_dir() {
                if let Some(path) = entry.path().to_str() {
                    agg_size += total_size(path, pattern)?;
                }
            }
        }
    }

    return Ok(agg_size);
}

pub fn print_fmt(size: u64) -> Result<(), Box<dyn Error>> {
    let mut total_size_str = String::new();

    let size_kb = size as f64 / 1024.;
    let size_mb = size_kb / 1024.;

    write!(&mut total_size_str, "Total size: {size} bytes")?;

    if size_kb > 1. {
        write!(&mut total_size_str, " / {size_kb:.2} KB")?;
    }

    if size_mb > 1. {
        write!(&mut total_size_str, " / {size_mb:.2} MB")?;
    }

    println!("{total_size_str}");
    Ok(())
}
