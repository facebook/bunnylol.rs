use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::encode_url;

pub struct WikipediaCommand;

impl BunnylolCommand for WikipediaCommand {
    const BINDINGS: &'static [&'static str] = &["wiki", "wikipedia"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            return "https://en.wikipedia.org/".to_string();
        }
        let encoded_query = encode_url(query);
        format!(
            "https://en.wikipedia.org/w/index.php?search={}&title=Special%3ASearch&ns0=1",
            encoded_query
        )
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: vec!["wiki".to_string(), "wikipedia".to_string()],
            description: "Search on Wikipedia".to_string(),
            example: "wiki rust programming".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wikipedia_command_simple() {
        assert_eq!(
            WikipediaCommand::process_args("hello"),
            "https://en.wikipedia.org/w/index.php?search=hello&title=Special%3ASearch&ns0=1"
        );
    }

    #[test]
    fn test_wikipedia_command_with_spaces() {
        assert_eq!(
            WikipediaCommand::process_args("hello world"),
            "https://en.wikipedia.org/w/index.php?search=hello%20world&title=Special%3ASearch&ns0=1"
        );
    }

    #[test]
    fn test_wikipedia_command_with_wiki_prefix() {
        assert_eq!(
            WikipediaCommand::process_args("wiki hello world"),
            "https://en.wikipedia.org/w/index.php?search=hello%20world&title=Special%3ASearch&ns0=1"
        );
    }

    #[test]
    fn test_wikipedia_command_wiki_only() {
        assert_eq!(
            WikipediaCommand::process_args("wiki"),
            "https://en.wikipedia.org/"
        );
    }
}
