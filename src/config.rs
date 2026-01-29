use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub roots: Vec<String>,
    #[serde(default)]
    pub marks: HashMap<String, String>,
}

pub fn path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("f")
        .join("config.json")
}

pub fn load() -> Result<Config> {
    let path = path();

    if !path.exists() {
        let config = Config::default();
        save(&config)?;
        return Ok(config);
    }

    let content = fs::read_to_string(&path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn save(config: &Config) -> Result<()> {
    let path = path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = serde_json::to_string_pretty(config)?;
    fs::write(&path, content)?;
    Ok(())
}

pub fn add_root(path: &str) -> Result<()> {
    let mut config = load()?;
    let expanded = shellexpand::tilde(path).to_string();

    if !config.roots.contains(&expanded) {
        config.roots.push(expanded.clone());
        save(&config)?;
        eprintln!("{expanded}");
    }

    Ok(())
}

pub fn remove_root(path: &str) -> Result<()> {
    let mut config = load()?;
    let expanded = shellexpand::tilde(path).to_string();

    if let Some(pos) = config.roots.iter().position(|x| x == &expanded) {
        config.roots.remove(pos);
        save(&config)?;
    }

    Ok(())
}
