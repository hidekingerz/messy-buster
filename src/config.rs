use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub base_dir: String,
}

pub fn read_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(file_path)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

pub fn write_config(file_path: &str, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_content = toml::to_string(config)?;
    let mut file = fs::File::create(file_path)?;
    file.write_all(config_content.as_bytes())?;
    Ok(())
}