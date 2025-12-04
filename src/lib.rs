use regex::Regex;
use std::error::Error;
use std::fmt::Write;
use walkdir::WalkDir;

pub fn total_size(root: String, pattern: String) -> Result<u64, Box<dyn Error>> {
    let mut total_size = 0;
    let re = Regex::new(&pattern)?;

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }

        if let Some(filename) = entry.file_name().to_str() {
            if re.is_match(filename) {
                if let Some(path) = entry.path().to_str() {
                    let size = entry.metadata()?.len();

                    total_size += size;

                    println!("path: {path}\nsize: {size} bytes");
                }
            }
        }
    }

    let mut total_size_str = String::new();

    let size_kb = total_size as f64 / 1024.;
    let size_mb = size_kb / 1024.;

    write!(&mut total_size_str, "Total size: {total_size} bytes")?;

    if size_kb > 1. {
        write!(&mut total_size_str, " / {size_kb:.2} KB")?;
    }

    if size_mb > 1. {
        write!(&mut total_size_str, " / {size_mb:.2} MB")?;
    }

    println!("{total_size_str}");

    return Ok(total_size);
}
