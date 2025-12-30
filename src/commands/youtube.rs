/// YouTube command handler
/// Supports:
/// - yt/youtube -> https://youtube.com/
/// - yt [search terms] -> https://www.youtube.com/results?search_query=[search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct YouTubeCommand;

impl BunnylolCommand for YouTubeCommand {
    const BINDINGS: &'static [&'static str] = &["yt", "youtube"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://youtube.com/".to_string()
        } else if query == "studio" {
            "https://studio.youtube.com/".to_string()
        } else if query == "subscriptions" || query == "subs" {
            "https://www.youtube.com/feed/subscriptions".to_string()
        } else {
            build_search_url("https://www.youtube.com/results", "search_query", query)
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to YouTube or search for videos".to_string(),
            example: "yt rust programming".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_youtube_command_base() {
        assert_eq!(YouTubeCommand::process_args("yt"), "https://youtube.com/");
        assert_eq!(
            YouTubeCommand::process_args("youtube"),
            "https://youtube.com/"
        );
    }

    #[test]
    fn test_youtube_command_studio() {
        assert_eq!(
            YouTubeCommand::process_args("yt studio"),
            "https://studio.youtube.com/"
        );
        assert_eq!(
            YouTubeCommand::process_args("youtube studio"),
            "https://studio.youtube.com/"
        );
    }

    #[test]
    fn test_youtube_command_subscriptions() {
        assert_eq!(
            YouTubeCommand::process_args("yt subscriptions"),
            "https://www.youtube.com/feed/subscriptions"
        );
        assert_eq!(
            YouTubeCommand::process_args("youtube subscriptions"),
            "https://www.youtube.com/feed/subscriptions"
        );
        assert_eq!(
            YouTubeCommand::process_args("yt subs"),
            "https://www.youtube.com/feed/subscriptions"
        );
        assert_eq!(
            YouTubeCommand::process_args("youtube subs"),
            "https://www.youtube.com/feed/subscriptions"
        );
    }

    #[test]
    fn test_youtube_command_search() {
        assert_eq!(
            YouTubeCommand::process_args("yt rust programming"),
            "https://www.youtube.com/results?search_query=rust%20programming"
        );
        assert_eq!(
            YouTubeCommand::process_args("youtube tutorial"),
            "https://www.youtube.com/results?search_query=tutorial"
        );
    }
}
