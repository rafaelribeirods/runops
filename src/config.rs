use std::fs;

use anyhow::{Result, anyhow};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConfigConnection {
    pub target: String,
    pub r#type: String,
    pub port: u16
}

pub fn load() -> Result<Vec<ConfigConnection>> {
    let contents = get_file_contents()?;
    let connections = serde_yaml::from_str::<Vec<ConfigConnection>>(&contents)
        .map_err(|err| anyhow!(format!("Could not parse the config file: {}", err)))?;

    Ok(connections)
}

fn get_file_contents() -> Result<String> {
    fs::read_to_string("config.yaml")
        .map_err(|err| anyhow!(format!("Error reading the config file: {}", err)))
}