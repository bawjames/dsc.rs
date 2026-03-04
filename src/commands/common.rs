use crate::api::DiscourseClient;
use crate::config::{Config, DiscourseConfig, find_discourse};
use anyhow::{Result, anyhow};
use std::fmt::Display;

pub fn select_discourse<'a>(
    config: &'a Config,
    discourse_name: Option<&str>,
) -> Result<&'a DiscourseConfig> {
    if let Some(name) = discourse_name {
        return find_discourse(config, name).ok_or_else(|| not_found("discourse", name));
    }
    Err(anyhow!("missing discourse for command"))
}

pub fn ensure_api_credentials(discourse: &DiscourseConfig) -> Result<()> {
    let apikey = discourse.apikey.as_deref().unwrap_or("").trim();
    let api_username = discourse.api_username.as_deref().unwrap_or("").trim();
    if apikey.is_empty() || api_username.is_empty() {
        return Err(missing_config(
            "apikey/api_username",
            &format!("discourse {}", discourse.name),
            "apikey and api_username",
        ));
    }
    Ok(())
}

pub fn not_found(resource: &str, identifier: impl Display) -> anyhow::Error {
    anyhow!("{} not found: {}", resource, identifier)
}

pub fn missing_config(field: &str, resource: &str, hint: &str) -> anyhow::Error {
    anyhow!(
        "missing {} for {}; please set {} or check your config",
        field,
        resource,
        hint
    )
}

pub fn parse_tags(raw: &str) -> Vec<String> {
    raw.split(|ch| ch == ';' || ch == ',')
        .map(|tag| tag.trim().to_string())
        .filter(|tag| !tag.is_empty())
        .collect()
}

pub fn fetch_fullname_from_url(baseurl: &str) -> Option<String> {
    let temp = DiscourseConfig {
        name: "temp".to_string(),
        baseurl: baseurl.to_string(),
        ..DiscourseConfig::default()
    };
    let client = match DiscourseClient::new(&temp) {
        Ok(client) => client,
        Err(err) => {
            println!("Failed to query site title for {}: {}", baseurl, err);
            return None;
        }
    };
    match client.fetch_site_title() {
        Ok(title) => {
            let title = title.trim().to_string();
            if title.is_empty() { None } else { Some(title) }
        }
        Err(err) => {
            println!("Failed to query site title for {}: {}", baseurl, err);
            None
        }
    }
}
