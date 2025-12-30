/// NPM command handler
/// Supports:
/// - npm/npmjs -> https://www.npmjs.com
/// - npm [search terms] -> https://www.npmjs.com/search?q=[search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct NpmCommand;

impl BunnylolCommand for NpmCommand {
    const BINDINGS: &'static [&'static str] = &["npm", "npmjs"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://www.npmjs.com".to_string()
        } else {
            build_search_url("https://www.npmjs.com/search", "q", query)
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to npmjs.com or search for npm packages".to_string(),
            example: "npm react".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npm_command_base() {
        assert_eq!(NpmCommand::process_args("npm"), "https://www.npmjs.com");
        assert_eq!(NpmCommand::process_args("npmjs"), "https://www.npmjs.com");
    }

    #[test]
    fn test_npm_command_search() {
        assert_eq!(
            NpmCommand::process_args("npm react"),
            "https://www.npmjs.com/search?q=react"
        );
        assert_eq!(
            NpmCommand::process_args("npmjs express middleware"),
            "https://www.npmjs.com/search?q=express%20middleware"
        );
    }
}
