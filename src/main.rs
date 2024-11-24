use clap::{Arg, Command};
use std::io;
use messy_buster::{config::{self, Config}, file_info, file_mover};

fn main() -> io::Result<()> {
    let matches = Command::new("messy-buster")
        .version("0.1.0")
        .author("hidekingerz <hidekingerz@gmail.com>")
        .about("Organizes files by extension and date")
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .value_name("DIRECTORY")
                .help("Sets the base directory")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("save")
                .short('s')
                .long("save")
                .help("Save the directory to config.toml")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .help("Simulate the file organization")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("show-config")
                .long("show-config")
                .help("Show the current configuration")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let mut config = config::read_config("config.toml").unwrap_or(Config {
        base_dir: String::new(),
    });

    if matches.get_flag("show-config") {
        println!("Current base directory: {}", config.base_dir);
        return Ok(());
    }

    if let Some(dir) = matches.get_one::<String>("directory") {
        config.base_dir = dir.to_string();
        if matches.get_flag("save") {
            config::write_config("config.toml", &config).expect("Failed to write config file");
            println!("Configuration saved to config.toml");
            return Ok(());
        }
    }

    let dir_path = &config.base_dir;
    let dry_run = matches.get_flag("dry-run");
    let file_infos = file_info::get_file_infos(dir_path)?;
    file_mover::process_files(file_infos, dir_path, dry_run)?;

    Ok(())
}
