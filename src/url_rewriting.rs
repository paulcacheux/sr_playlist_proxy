//! Module containing the logic to convert manifests containing absolute URLs to manifests
//! containing only relative URLs. The relative URLs contain the original URLs as a path segment
//! after a prefix segment.

use base64;
use reqwest::Url;

/// The prefix segment constant
pub const URL_REWRITING_PREFIX: &str = "relative_to_absolute_proxy_path";

/// Returns a new relative URL from an absolute one, containing the original URL as a path segment
///
/// # Errors
///
/// This function returns an error if the given URL is relative
pub fn rewrite_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = Url::parse(url)?; // this won't parse a relative URL

    // So from here we are working with a URL from a possibly different domain
    let b64_url = base64::encode_config(url.to_string().as_bytes(), base64::URL_SAFE);
    Ok(format!(
        "/{}/{}{}",
        URL_REWRITING_PREFIX,
        b64_url,
        url.path()
    ))
}

/// Returns a new manifest with only relative URLs
pub fn rewrite_manifest(manifest_content: &str) -> String {
    let mut result = String::new();

    for line in manifest_content.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if line.trim_start().starts_with("#") {
            result.push_str(line);
        } else {
            match rewrite_url(line) {
                Ok(new_url) => result.push_str(&new_url),
                Err(_) => result.push_str(line),
            }
        }
        result.push('\n');
    }

    result
}
