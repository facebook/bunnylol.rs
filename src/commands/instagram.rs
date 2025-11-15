/// Instagram command handler
/// Supports: ig, ig @[username], ig [search terms]
use crate::utils::bunnylol_command::{BunnylolCommand, CommandInfo};
use crate::utils::url_encoding::{build_path_url, build_search_url};

pub struct InstagramCommand;

impl InstagramCommand {
    fn construct_profile_url(profile: &str) -> String {
        build_path_url("https://www.instagram.com", profile)
    }

    fn construct_search_url(query: &str) -> String {
        build_search_url("https://www.instagram.com/explore/search/keyword", "q", query)
    }
}

impl BunnylolCommand for InstagramCommand {
    const BINDINGS: &'static [&'static str] = &["ig"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://www.instagram.com".to_string()
        } else {
            // Check if it looks like an Instagram profile
            if query.starts_with('@') {
                Self::construct_profile_url(&query[1..])
            } else {
                Self::construct_search_url(query)
            }
        }
    }

    fn get_info() -> CommandInfo {
        CommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Instagram profiles or search Instagram".to_string(),
            example: "ig @instagram".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instagram_command_base() {
        assert_eq!(
            InstagramCommand::process_args("ig"),
            "https://www.instagram.com"
        );
    }

    #[test]
    fn test_instagram_command_profile() {
        assert_eq!(
            InstagramCommand::process_args("ig @instagram"),
            "https://www.instagram.com/instagram"
        );
    }

    #[test]
    fn test_instagram_command_search() {
        assert_eq!(
            InstagramCommand::process_args("ig travel photography"),
            "https://www.instagram.com/explore/search/keyword?q=travel%20photography"
        );
    }
}
