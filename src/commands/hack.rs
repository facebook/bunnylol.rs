/// Hack command handler
/// Supports:
/// - hack -> https://docs.hhvm.com/hack/
/// - hack [search terms] -> https://docs.hhvm.com/search?term=[search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct HackCommand;

impl BunnylolCommand for HackCommand {
    const BINDINGS: &'static [&'static str] = &["hack"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://docs.hhvm.com/hack/".to_string()
        } else {
            build_search_url("https://docs.hhvm.com/search", "term", query)
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Hack documentation or search Hack docs".to_string(),
            example: "hack async".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hack_command_base() {
        assert_eq!(
            HackCommand::process_args("hack"),
            "https://docs.hhvm.com/hack/"
        );
    }

    #[test]
    fn test_hack_command_search() {
        assert_eq!(
            HackCommand::process_args("hack async"),
            "https://docs.hhvm.com/search?term=async"
        );
        assert_eq!(
            HackCommand::process_args("hack vec dict"),
            "https://docs.hhvm.com/search?term=vec%20dict"
        );
    }
}
