/// DuckDuckGo Search command handler
/// Supports: ddg [search terms], duckduckgo [search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct DuckDuckGoCommand;

impl BunnylolCommand for DuckDuckGoCommand {
    const BINDINGS: &'static [&'static str] = &["ddg", "duckduckgo"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        build_search_url("https://duckduckgo.com/", "q", query)
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Search DuckDuckGo".to_string(),
            example: "ddg rust programming".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duckduckgo_command_simple() {
        assert_eq!(
            DuckDuckGoCommand::process_args("hello"),
            "https://duckduckgo.com/?q=hello"
        );
    }

    #[test]
    fn test_duckduckgo_command_with_spaces() {
        assert_eq!(
            DuckDuckGoCommand::process_args("macos photos hidden"),
            "https://duckduckgo.com/?q=macos%20photos%20hidden"
        );
    }

    #[test]
    fn test_duckduckgo_command_with_ddg_prefix() {
        assert_eq!(
            DuckDuckGoCommand::process_args("ddg hello world"),
            "https://duckduckgo.com/?q=hello%20world"
        );
    }

    #[test]
    fn test_duckduckgo_command_with_full_prefix() {
        assert_eq!(
            DuckDuckGoCommand::process_args("duckduckgo hello world"),
            "https://duckduckgo.com/?q=hello%20world"
        );
    }

    #[test]
    fn test_duckduckgo_command_ddg_only() {
        assert_eq!(
            DuckDuckGoCommand::process_args("ddg"),
            "https://duckduckgo.com/?q="
        );
    }
}
