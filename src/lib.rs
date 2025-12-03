use regex::Regex;
use std::error::Error;
use walkdir::WalkDir;

pub fn total_size(root: String, pattern: String) -> Result<(), Box<dyn Error>> {
    let re = Regex::new(&pattern)?;

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if let Some(filename) = entry.file_name().to_str() {
            if re.is_match(filename) {
                if let Some(path) = entry.path().to_str() {
                    println!("{path}");
                }
            }
        }
    }

    return Ok(());
}
