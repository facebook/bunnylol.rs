/// Reddit command handler
/// Supports:
/// - r -> https://reddit.com
/// - r [search terms] -> https://www.reddit.com/search/?q=[search terms]
/// - r r/[subreddit] -> https://reddit.com/r/[subreddit]
/// - r r/[subreddit] [search terms] -> https://reddit.com/r/[subreddit]/search/?q=[search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct RedditCommand;

impl BunnylolCommand for RedditCommand {
    const BINDINGS: &'static [&'static str] = &["r", "reddit"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://reddit.com".to_string()
        } else {
            // Check if it starts with r/ (subreddit pattern)
            if let Some(subreddit_part) = query.strip_prefix("r/") {
                // Check if there are search terms after the subreddit
                if let Some(space_idx) = subreddit_part.find(' ') {
                    let subreddit = &subreddit_part[..space_idx];
                    let search_terms = &subreddit_part[space_idx + 1..];
                    build_search_url(
                        &format!("https://reddit.com/r/{}/search/", subreddit),
                        "q",
                        search_terms,
                    )
                } else {
                    // Just a subreddit
                    format!("https://reddit.com/r/{}", subreddit_part)
                }
            } else {
                // General reddit search
                build_search_url("https://www.reddit.com/search/", "q", query)
            }
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Reddit or search subreddits".to_string(),
            example: "r r/rust".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reddit_command_base() {
        assert_eq!(RedditCommand::process_args("r"), "https://reddit.com");
    }

    #[test]
    fn test_reddit_command_general_search() {
        assert_eq!(
            RedditCommand::process_args("r rust programming"),
            "https://www.reddit.com/search/?q=rust%20programming"
        );
    }

    #[test]
    fn test_reddit_command_subreddit() {
        assert_eq!(
            RedditCommand::process_args("r r/rust"),
            "https://reddit.com/r/rust"
        );
    }

    #[test]
    fn test_reddit_command_subreddit_search() {
        assert_eq!(
            RedditCommand::process_args("r r/rust async await"),
            "https://reddit.com/r/rust/search/?q=async%20await"
        );
    }
}
