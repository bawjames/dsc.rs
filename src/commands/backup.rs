use crate::commands::common::{ensure_api_credentials, select_discourse};
use crate::config::Config;
use crate::discourse::DiscourseClient;
use anyhow::Result;

pub fn backup_create(config: &Config, discourse_name: &str) -> Result<()> {
    let discourse = select_discourse(config, Some(discourse_name))?;
    ensure_api_credentials(discourse)?;
    let client = DiscourseClient::new(discourse)?;
    client.create_backup()?;
    Ok(())
}

pub fn backup_list(config: &Config, discourse_name: &str) -> Result<()> {
    let discourse = select_discourse(config, Some(discourse_name))?;
    ensure_api_credentials(discourse)?;
    let client = DiscourseClient::new(discourse)?;
    let backups = client.list_backups()?;
    let raw = serde_json::to_string_pretty(&backups)?;
    println!("{}", raw);
    Ok(())
}

pub fn backup_restore(config: &Config, discourse_name: &str, backup_path: &str) -> Result<()> {
    let discourse = select_discourse(config, Some(discourse_name))?;
    ensure_api_credentials(discourse)?;
    let client = DiscourseClient::new(discourse)?;
    client.restore_backup(backup_path)?;
    Ok(())
}
