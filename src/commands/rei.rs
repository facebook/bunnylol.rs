/// REI command handler
/// Supports: rei -> https://www.rei.com, rei [search terms] -> https://www.rei.com/search?q=[search terms]
use crate::utils::bunnylol_command::{BunnylolCommand, CommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct REICommand;

impl BunnylolCommand for REICommand {
    const COMMAND: &'static str = "rei";

    fn process_args(args: &str) -> String {
        if args == Self::COMMAND {
            "https://www.rei.com".to_string()
        } else {
            let query = Self::get_command_args(args);
            build_search_url("https://www.rei.com/search", "q", query)
        }
    }

    fn get_info() -> CommandInfo {
        CommandInfo {
            command: Self::COMMAND.to_string(),
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
