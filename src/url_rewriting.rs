//! Module containing the logic to convert manifests containing absolute URLs to manifests
//! containing only relative URLs. The relative URLs contain the original URLs as a path segment
//! after a prefix segment.

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

        if line.trim_start().starts_with('#') {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rewrite_url() {
        assert!(rewrite_url("/test.m3u").is_err());
        assert_eq!(
            rewrite_url("http://test.test/test.ts").map_err(|_| ()),
            Ok(
                "/relative_to_absolute_proxy_path/aHR0cDovL3Rlc3QudGVzdC90ZXN0LnRz/test.ts"
                    .to_string()
            )
        );
    }

    #[test]
    fn test_rewrite_manifest() {
        let input_manifest = "
#EXINF TEST
/test.m3u
http://test.test/test.ts
        ";

        let expected_result = "#EXINF TEST
/test.m3u
/relative_to_absolute_proxy_path/aHR0cDovL3Rlc3QudGVzdC90ZXN0LnRz/test.ts\n";

        assert_eq!(rewrite_manifest(input_manifest), expected_result);
    }
}
