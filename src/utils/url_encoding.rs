/// Shared URL encoding utilities for Bunnylol commands
///
/// This module provides common URL encoding functionality to eliminate
/// duplication across different command implementations.
extern crate percent_encoding;

use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};

/// URL fragment encoding set used for percent encoding
/// Used as part of the percent_encoding library to safely encode URLs
pub const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'&')
    .add(b'=')
    .add(b'+')
    .add(b'#');

/// Encode a string for safe use in URLs
///
/// # Arguments
/// * `input` - The string to encode
///
/// # Returns
/// A URL-encoded string safe for use in URLs
///
/// # Example
/// ```
/// use bunnylol::utils::url_encoding::encode_url;
///
/// let encoded = encode_url("hello world");
/// assert_eq!(encoded, "hello%20world");
/// ```
pub fn encode_url(input: &str) -> String {
    utf8_percent_encode(input, FRAGMENT).to_string()
}

/// Build a search URL with proper encoding
///
/// # Arguments
/// * `base_url` - The base URL (e.g., "https://google.com/search")
/// * `query_param` - The query parameter name (e.g., "q")
/// * `query_value` - The search term to encode
///
/// # Returns
/// A complete search URL with properly encoded query parameters
///
/// # Example
/// ```
/// use bunnylol::utils::url_encoding::build_search_url;
///
/// let url = build_search_url("https://google.com/search", "q", "hello world");
/// assert_eq!(url, "https://google.com/search?q=hello%20world");
/// ```
pub fn build_search_url(base_url: &str, query_param: &str, query_value: &str) -> String {
    let encoded_query = encode_url(query_value);
    format!("{}?{}={}", base_url, query_param, encoded_query)
}

/// Build a simple path URL with proper encoding
///
/// # Arguments
/// * `base_url` - The base URL (e.g., "https://github.com")
/// * `path` - The path to append and encode
///
/// # Returns
/// A complete URL with properly encoded path
///
/// # Example
/// ```
/// use bunnylol::utils::url_encoding::build_path_url;
///
/// let url = build_path_url("https://github.com", "facebook/react");
/// assert_eq!(url, "https://github.com/facebook/react");
/// ```
pub fn build_path_url(base_url: &str, path: &str) -> String {
    let encoded_path = encode_url(path);
    format!("{}/{}", base_url.trim_end_matches('/'), encoded_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_url_simple() {
        assert_eq!(encode_url("hello"), "hello");
    }

    #[test]
    fn test_encode_url_with_spaces() {
        assert_eq!(encode_url("hello world"), "hello%20world");
    }

    #[test]
    fn test_encode_url_with_special_chars() {
        assert_eq!(encode_url("hello<world>"), "hello%3Cworld%3E");
    }

    #[test]
    fn test_build_search_url() {
        let url = build_search_url("https://google.com/search", "q", "hello world");
        assert_eq!(url, "https://google.com/search?q=hello%20world");
    }

    #[test]
    fn test_build_path_url() {
        let url = build_path_url("https://github.com", "facebook/react");
        assert_eq!(url, "https://github.com/facebook/react");
    }

    #[test]
    fn test_build_path_url_with_trailing_slash() {
        let url = build_path_url("https://github.com/", "facebook/react");
        assert_eq!(url, "https://github.com/facebook/react");
    }

    #[test]
    fn test_encode_url_with_ampersand() {
        assert_eq!(
            encode_url("Peak Sports & Spine"),
            "Peak%20Sports%20%26%20Spine"
        );
    }

    #[test]
    fn test_encode_url_with_equals() {
        assert_eq!(encode_url("a=b"), "a%3Db");
    }

    #[test]
    fn test_encode_url_with_hash() {
        assert_eq!(encode_url("a#b"), "a%23b");
    }

    #[test]
    fn test_encode_url_with_plus() {
        assert_eq!(encode_url("a+b"), "a%2Bb");
    }

    #[test]
    fn test_build_search_url_with_ampersand() {
        let url = build_search_url(
            "https://google.com/search",
            "q",
            "Peak Sports & Spine Physical Therapy",
        );
        assert_eq!(
            url,
            "https://google.com/search?q=Peak%20Sports%20%26%20Spine%20Physical%20Therapy"
        );
    }
}
