/// YouTube command handler
/// Supports: yt, yt @[handler], yt [search terms]
use crate::utils::bunnylol_command::BunnylolCommand;
use crate::utils::url_encoding::{build_path_url, build_search_url};

pub struct YouTubeCommand;

impl YouTubeCommand {
    fn construct_channel_url(channel: &str) -> String {
        build_path_url("https://youtube.com", channel)
    }

    fn construct_search_url(query: &str) -> String {
        build_search_url("https://youtube.com/results", "search_query", query)
    }
}

impl BunnylolCommand for YouTubeCommand {
    const COMMAND: &'static str = "yt";

    fn process_args(args: &str) -> String {
        if args == Self::COMMAND {
            "https://youtube.com".to_string()
        } else {
            let query = Self::get_command_args(args);

            // Check if it looks like a YouTube channel
            if query.starts_with('@') {
                Self::construct_channel_url(query)
            } else {
                Self::construct_search_url(query)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_youtube_command_base() {
        assert_eq!(YouTubeCommand::process_args("yt"), "https://youtube.com");
    }

    #[test]
    fn test_youtube_command_channel() {
        assert_eq!(
            YouTubeCommand::process_args("yt @meta"),
            "https://youtube.com/@meta"
        );
    }

    #[test]
    fn test_youtube_command_search() {
        assert_eq!(
            YouTubeCommand::process_args("yt stella sora"),
            "https://youtube.com/results?search_query=stella%20sora"
        );
    }

    #[test]
    fn test_construct_youtube_channel_url() {
        assert_eq!(
            YouTubeCommand::construct_channel_url("@mannyseete1996"),
            "https://youtube.com/@mannyseete1996"
        );
    }

    #[test]
    fn test_construct_youtube_search_url() {
        assert_eq!(
            YouTubeCommand::construct_search_url("stella sora"),
            "https://youtube.com/results?search_query=stella%20sora"
        );
    }
}
