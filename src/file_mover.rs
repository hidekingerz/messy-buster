use std::collections::HashMap;
use std::fs;
use std::io;
use crate::file_info::FileInfo;

fn create_directory(base_dir: &str, date_str: &str, extension: &str) -> io::Result<String> {
    let new_dir = format!("{}/{}/{}", base_dir, date_str, extension);
    fs::create_dir_all(&new_dir)?;
    Ok(new_dir)
}

fn move_file(file_info: &FileInfo, new_dir: &str, dry_run: bool, folder_tree: &mut HashMap<String, Vec<String>>) -> io::Result<()> {
    let file_name = file_info.path.file_name().unwrap().to_str().unwrap();
    let new_path = format!("{}/{}", new_dir, file_name);
    folder_tree.entry(new_dir.to_string()).or_default().push(file_name.to_string());
    if !dry_run {
        fs::rename(&file_info.path, &new_path)?;
    }
    Ok(())
}

fn show_folder_tree(folder_tree: &HashMap<String, Vec<String>>) {
    println!("The following folder structure will be created:");
    for (folder, files) in folder_tree {
        println!("{}", folder);
        for file in files {
            println!("  {}", file);
        }
    }
}

pub fn process_files(file_infos: Vec<FileInfo>, base_dir: &str, dry_run: bool) -> io::Result<()> {
    let mut folder_tree: HashMap<String, Vec<String>> = HashMap::new();

    for file_info in file_infos {
        let date_str = file_info.modified_date.format("%Y-%m-%d").to_string();
        let new_dir = create_directory(base_dir, &date_str, &file_info.extension)?;
        move_file(&file_info, &new_dir, dry_run, &mut folder_tree)?;
    }

    show_folder_tree(&folder_tree);

    Ok(())
}