/// MDN command handler
/// Supports:
/// - mdn -> https://developer.mozilla.org
/// - mdn [search terms] -> https://developer.mozilla.org/en-US/search?q=[search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct MdnCommand;

impl BunnylolCommand for MdnCommand {
    const BINDINGS: &'static [&'static str] = &["mdn"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://developer.mozilla.org".to_string()
        } else {
            build_search_url("https://developer.mozilla.org/en-US/search", "q", query)
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to MDN Web Docs or search for web development resources"
                .to_string(),
            example: "mdn flexbox".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mdn_command_base() {
        assert_eq!(
            MdnCommand::process_args("mdn"),
            "https://developer.mozilla.org"
        );
    }

    #[test]
    fn test_mdn_command_search() {
        assert_eq!(
            MdnCommand::process_args("mdn flexbox"),
            "https://developer.mozilla.org/en-US/search?q=flexbox"
        );
        assert_eq!(
            MdnCommand::process_args("mdn array methods"),
            "https://developer.mozilla.org/en-US/search?q=array%20methods"
        );
    }
}
