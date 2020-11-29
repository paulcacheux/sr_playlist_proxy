use base64;
use reqwest::Url;

pub const URL_REWRITING_PREFIX: &str = "relative_to_absolute_proxy_path";

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
