/// Instagram command handler
/// Supports: ig, instagram, ig @[username], ig [search terms]
/// Supports: ig reels -> redirects to Instagram Reels
/// Supports: ig messages/msg/chat -> redirects to Instagram Direct Inbox
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::{build_path_url, build_search_url};

pub struct InstagramCommand;

impl InstagramCommand {
    fn construct_profile_url(profile: &str) -> String {
        build_path_url("https://www.instagram.com", profile)
    }

    fn construct_search_url(query: &str) -> String {
        build_search_url(
            "https://www.instagram.com/explore/search/keyword",
            "q",
            query,
        )
    }
}

impl BunnylolCommand for InstagramCommand {
    const BINDINGS: &'static [&'static str] = &["ig", "instagram"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://www.instagram.com".to_string()
        } else {
            // Check for specific subcommands first
            match query {
                "reels" => "https://www.instagram.com/reels/".to_string(),
                "messages" | "msg" | "chat" => {
                    "https://www.instagram.com/direct/inbox/".to_string()
                }
                _ => {
                    // Check if it looks like an Instagram profile
                    if let Some(username) = query.strip_prefix('@') {
                        if !username.is_empty() {
                            Self::construct_profile_url(username)
                        } else {
                            // Just '@' with no username - go to homepage
                            "https://www.instagram.com".to_string()
                        }
                    } else {
                        Self::construct_search_url(query)
                    }
                }
            }
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description:
                "Navigate to Instagram profiles, search Instagram, or access Reels/Messages"
                    .to_string(),
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
    fn test_instagram_command_base_full_name() {
        assert_eq!(
            InstagramCommand::process_args("instagram"),
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
    fn test_instagram_command_profile_full_name() {
        assert_eq!(
            InstagramCommand::process_args("instagram @instagram"),
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

    #[test]
    fn test_instagram_command_search_full_name() {
        assert_eq!(
            InstagramCommand::process_args("instagram travel photography"),
            "https://www.instagram.com/explore/search/keyword?q=travel%20photography"
        );
    }

    #[test]
    fn test_instagram_command_reels() {
        assert_eq!(
            InstagramCommand::process_args("ig reels"),
            "https://www.instagram.com/reels/"
        );
    }

    #[test]
    fn test_instagram_command_messages() {
        assert_eq!(
            InstagramCommand::process_args("ig messages"),
            "https://www.instagram.com/direct/inbox/"
        );
    }

    #[test]
    fn test_instagram_command_msg() {
        assert_eq!(
            InstagramCommand::process_args("ig msg"),
            "https://www.instagram.com/direct/inbox/"
        );
    }

    #[test]
    fn test_instagram_command_chat() {
        assert_eq!(
            InstagramCommand::process_args("ig chat"),
            "https://www.instagram.com/direct/inbox/"
        );
    }

    #[test]
    fn test_instagram_command_empty_username() {
        assert_eq!(
            InstagramCommand::process_args("ig @"),
            "https://www.instagram.com"
        );
    }
}
