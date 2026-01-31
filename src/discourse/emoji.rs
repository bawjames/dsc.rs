use super::client::DiscourseClient;
use super::models::CustomEmoji;
use anyhow::{anyhow, Context, Result};
use reqwest::StatusCode;
use serde_json::Value;
use std::path::Path;

impl DiscourseClient {
    /// Upload a custom emoji.
    pub fn upload_emoji(&self, emoji_path: &Path, emoji_name: &str) -> Result<()> {
        let file = std::fs::read(emoji_path)
            .with_context(|| format!("reading {}", emoji_path.display()))?;
        let part = reqwest::blocking::multipart::Part::bytes(file)
            .file_name(
                emoji_path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("emoji.png")
                    .to_string(),
            )
            .mime_str("image/png")
            .context("setting emoji mime")?;
        let form = reqwest::blocking::multipart::Form::new()
            .part("emoji[image]", part)
            .text("emoji[name]", emoji_name.to_string());

        let response = self
            .post("/admin/customize/emojis")?
            .multipart(form)
            .send()
            .context("uploading emoji")?;
        if !response.status().is_success() {
            return Err(anyhow!("emoji upload failed with {}", response.status()));
        }
        Ok(())
    }

    /// List custom emojis.
    pub fn list_custom_emojis(&self) -> Result<Vec<CustomEmoji>> {
        if let Ok(emojis) = self.list_admin_emojis() {
            return Ok(emojis);
        }
        self.list_public_emojis()
    }

    fn list_admin_emojis(&self) -> Result<Vec<CustomEmoji>> {
        let response = self.get("/admin/customize/emojis.json")?;
        let status = response.status();
        let text = response.text().context("reading emoji list response")?;
        if !status.is_success() {
            return Err(anyhow!("emoji list failed with {}: {}", status, text));
        }
        let value: Value = serde_json::from_str(&text).context("parsing emoji list json")?;
        let emojis = if let Some(arr) = value.as_array() {
            extract_emojis_from_array(arr, self.baseurl())
        } else if let Some(arr) = value.get("emojis").and_then(|v| v.as_array()) {
            extract_emojis_from_array(arr, self.baseurl())
        } else if let Some(map) = value.as_object() {
            let mut out = Vec::new();
            extract_emojis_from_map(map, self.baseurl(), &mut out);
            out
        } else {
            Vec::new()
        };
        Ok(emojis)
    }

    fn list_public_emojis(&self) -> Result<Vec<CustomEmoji>> {
        let response = self.get("/emoji.json")?;
        let status = response.status();
        let text = response.text().context("reading emoji.json response")?;
        if status == StatusCode::NOT_FOUND {
            return Ok(Vec::new());
        }
        if !status.is_success() {
            return Err(anyhow!(
                "emoji.json request failed with {}: {}",
                status,
                text
            ));
        }
        let value: Value = serde_json::from_str(&text).context("parsing emoji.json")?;
        let baseurl = self.baseurl().trim_end_matches('/');
        let mut out = Vec::new();
        if let Some(map) = value.get("custom_emoji").and_then(|v| v.as_object()) {
            extract_emojis_from_map(map, baseurl, &mut out);
        } else if let Some(map) = value.get("custom").and_then(|v| v.as_object()) {
            extract_emojis_from_map(map, baseurl, &mut out);
        } else if let Some(map) = value.get("emoji").and_then(|v| v.as_object()) {
            extract_emojis_from_map(map, baseurl, &mut out);
        }
        Ok(out)
    }
}

fn extract_emojis_from_array(emojis: &[Value], baseurl: &str) -> Vec<CustomEmoji> {
    let mut out = Vec::new();
    for item in emojis.iter() {
        let name = item.get("name").and_then(|v| v.as_str());
        let url = item
            .get("url")
            .and_then(|v| v.as_str())
            .or_else(|| item.get("image_url").and_then(|v| v.as_str()));
        if let (Some(name), Some(url)) = (name, url) {
            out.push(CustomEmoji {
                name: name.to_string(),
                url: normalize_emoji_url(baseurl, url),
            });
        }
    }
    out
}

fn extract_emojis_from_map(
    map: &serde_json::Map<String, Value>,
    baseurl: &str,
    out: &mut Vec<CustomEmoji>,
) {
    for (name, value) in map.iter() {
        if let Some(url) = value.as_str() {
            out.push(CustomEmoji {
                name: name.to_string(),
                url: normalize_emoji_url(baseurl, url),
            });
        }
    }
}

fn normalize_emoji_url(baseurl: &str, url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else if url.starts_with("//") {
        let scheme = if baseurl.starts_with("http://") {
            "http:"
        } else {
            "https:"
        };
        format!("{}{}", scheme, url)
    } else if url.starts_with('/') {
        format!("{}{}", baseurl, url)
    } else {
        format!("{}/{}", baseurl, url)
    }
}
