use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Top-level configuration for dsc.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub discourse: Vec<DiscourseConfig>,
}

/// Configuration for a single Discourse install.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DiscourseConfig {
    pub name: String,
    pub baseurl: String,
    #[serde(default)]
    pub apikey: Option<String>,
    #[serde(default)]
    pub api_username: Option<String>,
    #[serde(default)]
    pub changelog_path: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub changelog_topic_id: Option<u64>,
    #[serde(default)]
    pub ssh_host: Option<String>,
}

/// Load configuration from a TOML file.
pub fn load_config(path: &Path) -> Result<Config> {
    if !path.exists() {
        return Ok(Config::default());
    }
    let raw = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let config: Config = toml::from_str(&raw).with_context(|| "parsing config")?;
    Ok(config)
}

/// Save configuration to a TOML file.
pub fn save_config(path: &Path, config: &Config) -> Result<()> {
    let raw = toml::to_string_pretty(config).with_context(|| "serializing config")?;
    fs::write(path, raw).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

/// Find a discourse by name.
pub fn find_discourse<'a>(config: &'a Config, name: &str) -> Option<&'a DiscourseConfig> {
    config.discourse.iter().find(|d| d.name == name)
}

/// Find a discourse by name (mutable).
pub fn find_discourse_mut<'a>(
    config: &'a mut Config,
    name: &str,
) -> Option<&'a mut DiscourseConfig> {
    config.discourse.iter_mut().find(|d| d.name == name)
}
