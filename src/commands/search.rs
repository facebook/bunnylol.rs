/// Search engine fallback URL builder
/// Used when no command matches the input, routing to the configured default search engine
use crate::utils::url_encoding::build_search_url;

/// Build a search URL for the given engine and query string.
/// Falls back to Google for any unrecognized engine name.
pub fn search_url(engine: &str, query: &str) -> String {
    match engine {
        "ddg" | "duckduckgo" => build_search_url("https://duckduckgo.com/", "q", query),
        "bing" => build_search_url("https://www.bing.com/search", "q", query),
        _ => build_search_url("https://www.google.com/search", "q", query),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_url_google() {
        let url = search_url("google", "hello world");
        assert!(url.starts_with("https://www.google.com/search?q="));
        assert!(url.contains("hello"));
        assert!(url.contains("world"));
    }

    #[test]
    fn test_search_url_ddg() {
        let url = search_url("ddg", "test query");
        assert!(url.starts_with("https://duckduckgo.com/?q="));
    }

    #[test]
    fn test_search_url_duckduckgo_alias() {
        assert_eq!(
            search_url("ddg", "test query"),
            search_url("duckduckgo", "test query")
        );
    }

    #[test]
    fn test_search_url_bing() {
        let url = search_url("bing", "test query");
        assert!(url.starts_with("https://www.bing.com/search?q="));
    }

    #[test]
    fn test_search_url_unknown_defaults_to_google() {
        let url = search_url("unknown_engine", "test query");
        assert!(url.starts_with("https://www.google.com/search?q="));
    }
}
