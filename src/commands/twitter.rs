/// Twitter command handler
/// Supports: tw, tw @[username], tw [search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::{build_path_url, build_search_url};

pub struct TwitterCommand;

impl TwitterCommand {
    fn construct_profile_url(profile: &str) -> String {
        build_path_url("https://twitter.com", profile)
    }

    fn construct_search_url(query: &str) -> String {
        build_search_url("https://twitter.com/search", "q", query)
    }
}

impl BunnylolCommand for TwitterCommand {
    const BINDINGS: &'static [&'static str] = &["tw"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://twitter.com".to_string()
        } else {
            // Check if it looks like a Twitter profile
            if let Some(username) = query.strip_prefix('@') {
                if !username.is_empty() {
                    Self::construct_profile_url(username)
                } else {
                    // Just '@' with no username - go to homepage
                    "https://twitter.com".to_string()
                }
            } else {
                Self::construct_search_url(query)
            }
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Twitter profiles or search Twitter".to_string(),
            example: "tw @MetaOpenSource".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twitter_command_base() {
        assert_eq!(TwitterCommand::process_args("tw"), "https://twitter.com");
    }

    #[test]
    fn test_twitter_command_profile() {
        assert_eq!(
            TwitterCommand::process_args("tw @MetaOpenSource"),
            "https://twitter.com/MetaOpenSource"
        );
    }

    #[test]
    fn test_twitter_command_search() {
        assert_eq!(
            TwitterCommand::process_args("tw hello world"),
            "https://twitter.com/search?q=hello%20world"
        );
    }

    #[test]
    fn test_construct_twitter_profile_url() {
        assert_eq!(
            TwitterCommand::construct_profile_url("jsjoeio"),
            "https://twitter.com/jsjoeio"
        );
    }

    #[test]
    fn test_construct_twitter_search_url() {
        assert_eq!(
            TwitterCommand::construct_search_url("hello world"),
            "https://twitter.com/search?q=hello%20world"
        );
    }

    #[test]
    fn test_twitter_command_empty_username() {
        assert_eq!(TwitterCommand::process_args("tw @"), "https://twitter.com");
    }
}
