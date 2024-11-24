use std::fs;
use std::io;
use std::path::PathBuf;
use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct FileInfo {
    pub path: PathBuf,
    pub extension: String,
    pub modified_date: DateTime<Local>,
}

pub fn get_file_infos(dir_path: &str) -> io::Result<Vec<FileInfo>> {
    let entries = fs::read_dir(dir_path)?;
    let mut file_infos = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let metadata = fs::metadata(&path)?;
            let extension = path.extension().unwrap_or_default().to_str().unwrap_or_default().to_string();
            let modified = metadata.modified()?;
            let datetime: DateTime<Local> = modified.into();
            let file_info = FileInfo {
                path,
                extension,
                modified_date: datetime,
            };
            file_infos.push(file_info);
        }
    }

    Ok(file_infos)
}