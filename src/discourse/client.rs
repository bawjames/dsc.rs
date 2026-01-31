use super::models::{AboutResponse, SiteResponse};
use crate::config::DiscourseConfig;
use crate::utils::normalize_baseurl;
use anyhow::{anyhow, Context, Result};
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue};

/// HTTP client for the Discourse API.
#[derive(Clone)]
pub struct DiscourseClient {
    baseurl: String,
    client: Client,
}

impl DiscourseClient {
    /// Create a new Discourse API client.
    pub fn new(config: &DiscourseConfig) -> Result<Self> {
        let baseurl = normalize_baseurl(&config.baseurl);
        if baseurl.is_empty() {
            return Err(anyhow!("baseurl is required"));
        }

        let mut headers = HeaderMap::new();
        if let (Some(apikey), Some(api_username)) =
            (config.apikey.as_ref(), config.api_username.as_ref())
        {
            headers.insert(
                "Api-Key",
                HeaderValue::from_str(apikey).context("invalid api key")?,
            );
            headers.insert(
                "Api-Username",
                HeaderValue::from_str(api_username).context("invalid api username")?,
            );
        }

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .context("building http client")?;

        Ok(Self { baseurl, client })
    }

    /// Return the configured base URL.
    pub fn baseurl(&self) -> &str {
        &self.baseurl
    }

    pub(crate) fn get(&self, path: &str) -> Result<Response> {
        let url = format!("{}{}", self.baseurl, path);
        self.client.get(url).send().context("sending request")
    }

    pub(crate) fn post(&self, path: &str) -> Result<reqwest::blocking::RequestBuilder> {
        let url = format!("{}{}", self.baseurl, path);
        Ok(self.client.post(url))
    }

    pub(crate) fn put(&self, path: &str) -> Result<reqwest::blocking::RequestBuilder> {
        let url = format!("{}{}", self.baseurl, path);
        Ok(self.client.put(url))
    }

    /// Fetch the Discourse site title.
    pub fn fetch_site_title(&self) -> Result<String> {
        let site_json_error = match self.get("/site.json") {
            Ok(response) => {
                let status = response.status();
                let text = response.text().context("reading site.json response body")?;
                if status.is_success() {
                    let body: SiteResponse =
                        serde_json::from_str(&text).context("parsing site.json")?;
                    return Ok(body.site.title);
                }
                anyhow!("site.json request failed with {}", status)
            }
            Err(err) => err,
        };

        let response = self.get("/")?;
        let status = response.status();
        let html = response.text().context("reading site HTML")?;
        if !status.is_success() {
            return Err(anyhow!(
                "site title lookup failed (site.json error: {}; HTML request failed with {})",
                site_json_error,
                status
            ));
        }
        if let Some(title) = extract_html_title(&html) {
            return Ok(title);
        }
        Err(anyhow!(
            "site title lookup failed (site.json error: {}; HTML missing <title>)",
            site_json_error
        ))
    }

    /// Fetch the current Discourse version if exposed via the API.
    pub fn fetch_version(&self) -> Result<Option<String>> {
        let response = self.get("/about.json")?;
        let status = response.status();
        let body: AboutResponse = response.json().context("reading about.json")?;
        if !status.is_success() {
            return Err(anyhow!("about.json request failed with {}", status));
        }
        Ok(body.about.version.or(body.about.installed_version))
    }
}

fn extract_html_title(html: &str) -> Option<String> {
    let haystack = html.as_bytes();
    let mut lower = Vec::with_capacity(haystack.len());
    for &byte in haystack {
        lower.push(byte.to_ascii_lowercase());
    }
    let open_tag = b"<title>";
    let close_tag = b"</title>";
    let start = find_subslice(&lower, open_tag)? + open_tag.len();
    let end = find_subslice(&lower[start..], close_tag)? + start;
    let title = String::from_utf8_lossy(&haystack[start..end])
        .trim()
        .to_string();
    if title.is_empty() {
        None
    } else {
        Some(title)
    }
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
