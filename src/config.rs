use std::{error::Error, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub languages: Vec<String>,
    pub download_dir: Option<PathBuf>,
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let mut path = dirs::config_dir()
        .ok_or("Failed to get config directory")?
        .join("subget");

    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    path.push("config.json");

    if !path.exists() {
        fs::write(&path, "")?;
    }

    let bytes = fs::read(&path)?;

    match serde_json::from_slice(&bytes) {
        Ok(config) => Ok(config),

        Err(_) => Ok(Config {
            api_key: "".to_string(),
            languages: vec![],
            download_dir: None,
        }),
    }
}

pub fn save_config(config: Config) -> Result<(), Box<dyn Error>> {
    let path = dirs::config_dir()
        .ok_or("Failed to get config directory")?
        .join("subget")
        .join("config.json");

    let bytes = serde_json::to_vec_pretty(&config)?;
    fs::write(&path, &bytes)?;

    Ok(())
}
