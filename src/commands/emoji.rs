use crate::commands::common::{ensure_api_credentials, select_discourse};
use crate::config::Config;
use crate::discourse::DiscourseClient;
use crate::utils::slugify;
use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;

pub fn add_emoji(
    config: &Config,
    discourse_name: &str,
    emoji_path: &Path,
    emoji_name: Option<&str>,
) -> Result<()> {
    let discourse = select_discourse(config, Some(discourse_name))?;
    ensure_api_credentials(discourse)?;
    let client = DiscourseClient::new(discourse)?;
    if emoji_path.is_dir() {
        if emoji_name.is_some() {
            return Err(anyhow!(
                "emoji name is not allowed when uploading a directory"
            ));
        }
        let mut files = Vec::new();
        for entry in
            fs::read_dir(emoji_path).with_context(|| format!("reading {}", emoji_path.display()))?
        {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if !is_emoji_file(&path) {
                continue;
            }
            files.push(path);
        }
        files.sort();
        if files.is_empty() {
            return Err(anyhow!("no emoji image files found in directory"));
        }
        for path in files {
            let name = emoji_name_from_path(&path)?;
            client.upload_emoji(&path, &name)?;
            println!("uploaded {} from {}", name, path.display());
        }
        return Ok(());
    }

    let name = match emoji_name {
        Some(name) => name.to_string(),
        None => emoji_name_from_path(emoji_path)?,
    };
    client.upload_emoji(emoji_path, &name)?;
    Ok(())
}

pub fn list_emojis(config: &Config, discourse_name: &str) -> Result<()> {
    let discourse = select_discourse(config, Some(discourse_name))?;
    ensure_api_credentials(discourse)?;
    let client = DiscourseClient::new(discourse)?;
    let mut emojis = client.list_custom_emojis()?;
    emojis.sort_by(|a, b| a.name.cmp(&b.name));

    if emojis.is_empty() {
        println!("No custom emojis found");
        return Ok(());
    }

    println!("name\turl");
    for emoji in emojis {
        println!("{}\t{}", emoji.name, emoji.url);
    }
    Ok(())
}

fn emoji_name_from_path(path: &Path) -> Result<String> {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("emoji path missing filename: {}", path.display()))?;
    let slug = slugify(stem);
    let name = slug.replace('-', "_");
    if name.is_empty() {
        return Err(anyhow!("emoji name is empty for {}", path.display()));
    }
    Ok(name)
}

fn is_emoji_file(path: &Path) -> bool {
    let Some(ext) = path.extension().and_then(|s| s.to_str()) else {
        return false;
    };
    matches!(
        ext.to_ascii_lowercase().as_str(),
        "png" | "jpg" | "jpeg" | "gif" | "svg"
    )
}
