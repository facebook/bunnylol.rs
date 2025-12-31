/// Google Search command handler (default fallback)
/// Supports: g [search terms], or any unrecognized command
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct GoogleSearchCommand;

impl BunnylolCommand for GoogleSearchCommand {
    const BINDINGS: &'static [&'static str] = &["g"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        build_search_url("https://google.com/search", "q", query)
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: vec!["g".to_string(), "(default)".to_string()],
            description: "Search Google (default fallback for any unrecognized command)"
                .to_string(),
            example: "g rust programming".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_search_command_simple() {
        assert_eq!(
            GoogleSearchCommand::process_args("hello"),
            "https://google.com/search?q=hello"
        );
    }

    #[test]
    fn test_google_search_command_with_spaces() {
        assert_eq!(
            GoogleSearchCommand::process_args("hello world"),
            "https://google.com/search?q=hello%20world"
        );
    }

    #[test]
    fn test_google_search_command_with_g_prefix() {
        assert_eq!(
            GoogleSearchCommand::process_args("g hello world"),
            "https://google.com/search?q=hello%20world"
        );
    }

    #[test]
    fn test_google_search_command_g_only() {
        assert_eq!(
            GoogleSearchCommand::process_args("g"),
            "https://google.com/search?q="
        );
    }
}
