/// REI command handler
/// Supports: rei -> https://www.rei.com, rei [search terms] -> https://www.rei.com/search?q=[search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct REICommand;

impl BunnylolCommand for REICommand {
    const BINDINGS: &'static [&'static str] = &["rei"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://www.rei.com".to_string()
        } else {
            build_search_url("https://www.rei.com/search", "q", query)
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to REI or search for outdoor gear".to_string(),
            example: "rei hiking boots".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rei_command_base() {
        assert_eq!(REICommand::process_args("rei"), "https://www.rei.com");
    }

    #[test]
    fn test_rei_command_search() {
        assert_eq!(
            REICommand::process_args("rei hiking boots"),
            "https://www.rei.com/search?q=hiking%20boots"
        );
    }

    #[test]
    fn test_rei_command_search_multiple_words() {
        assert_eq!(
            REICommand::process_args("rei camping gear outdoor"),
            "https://www.rei.com/search?q=camping%20gear%20outdoor"
        );
    }
}
