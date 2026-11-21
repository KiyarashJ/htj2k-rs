use std::path::{Path, PathBuf};


pub fn default_output_path(input: &str) -> PathBuf {
    let path = Path::new(input);

    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));

    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    parent.join(format!("{stem}_htj2k.dcm"))
}
