use std::fs;
use log::*;
use regex::Regex;

pub fn rename_files_with_prefix(dir: &str, pattern: &str, template: &str) -> std::io::Result<()> {
    let re = Regex::new(pattern).expect("Invalid regex");

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                
                    if re.is_match(stem) && ext.eq_ignore_ascii_case("png") {
                        let new_stem = template.replace("{}", stem);
                        let new_name = format!("{}.{}", new_stem, ext);
                        let new_path = path.with_file_name(&new_name);

                        fs::rename(&path, &new_path)?;
                        debug!("Renamed: {:?} → {:?}", stem, new_name);
                    } else {
                        debug!("Skipped: {:?}", path.file_name().unwrap());
                    }
                }
            }
        }
    }
    Ok(())
}