/// Amazon command handler
/// Supports:
/// - az/amzn/azn/amazon -> https://amazon.com/
/// - az [search terms] -> https://www.amazon.com/s?k=[search terms]
use crate::utils::bunnylol_command::{BunnylolCommand, CommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct AmazonCommand;

impl BunnylolCommand for AmazonCommand {
    const BINDINGS: &'static [&'static str] = &["az", "amzn", "azn", "amazon"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://amazon.com/".to_string()
        } else {
            build_search_url("https://www.amazon.com/s", "k", query)
        }
    }

    fn get_info() -> CommandInfo {
        CommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Amazon or search for products".to_string(),
            example: "az headphones".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amazon_command_base() {
        assert_eq!(AmazonCommand::process_args("az"), "https://amazon.com/");
        assert_eq!(AmazonCommand::process_args("amzn"), "https://amazon.com/");
        assert_eq!(AmazonCommand::process_args("azn"), "https://amazon.com/");
        assert_eq!(
            AmazonCommand::process_args("amazon"),
            "https://amazon.com/"
        );
    }

    #[test]
    fn test_amazon_command_search() {
        assert_eq!(
            AmazonCommand::process_args("az headphones"),
            "https://www.amazon.com/s?k=headphones"
        );
        assert_eq!(
            AmazonCommand::process_args("amazon wireless mouse"),
            "https://www.amazon.com/s?k=wireless%20mouse"
        );
    }
}
