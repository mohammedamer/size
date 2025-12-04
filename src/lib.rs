use regex::Regex;
use std::error::Error;
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

    println!("total size: {total_size} bytes");

    return Ok(total_size);
}
