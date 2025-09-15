use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Override {
    pub min: Option<i16>,
    pub max: Option<i16>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub r#override: HashMap<String, Override>,
    pub languages: Vec<String>,
}

pub fn load_config(path: &std::path::Path) -> anyhow::Result<Config> {
    let contents = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
