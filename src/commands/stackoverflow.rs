/// Stack Overflow command handler
/// Supports:
/// - stackoverflow/so -> https://stackoverflow.com
/// - stackoverflow [search terms] -> https://stackoverflow.com/search?q=[search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct StackOverflowCommand;

impl BunnylolCommand for StackOverflowCommand {
    const BINDINGS: &'static [&'static str] = &["stackoverflow", "so"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://stackoverflow.com".to_string()
        } else {
            build_search_url("https://stackoverflow.com/search", "q", query)
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo::new(
            Self::BINDINGS,
            "Navigate to Stack Overflow or search for programming questions",
            "so rust ownership",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stackoverflow_command_base() {
        assert_eq!(
            StackOverflowCommand::process_args("so"),
            "https://stackoverflow.com"
        );
        assert_eq!(
            StackOverflowCommand::process_args("stackoverflow"),
            "https://stackoverflow.com"
        );
    }

    #[test]
    fn test_stackoverflow_command_search() {
        assert_eq!(
            StackOverflowCommand::process_args("so rust ownership"),
            "https://stackoverflow.com/search?q=rust%20ownership"
        );
        assert_eq!(
            StackOverflowCommand::process_args("stackoverflow async await"),
            "https://stackoverflow.com/search?q=async%20await"
        );
    }
}
