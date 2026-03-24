/// Reddit command handler
/// Supports:
/// - r -> https://reddit.com
/// - r [search terms] -> https://www.reddit.com/search/?q=[search terms]
/// - r r/[subreddit] -> https://reddit.com/r/[subreddit]
/// - r r/[subreddit] [search terms] -> https://reddit.com/r/[subreddit]/search/?q=[search terms]
/// - r/[subreddit] -> https://www.reddit.com/r/[subreddit]/  (prefix shorthand)
/// - r/[subreddit] [search terms] -> https://www.reddit.com/r/[subreddit]/search/?q=[search terms]
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
        BunnylolCommandInfo::new(
            Self::BINDINGS,
            "Navigate to Reddit or search subreddits",
            "r r/rust",
        )
    }
}

impl RedditCommand {
    /// Handle the `r/SUBREDDITNAME` prefix shorthand (called from the command registry).
    /// - `r/myog` -> https://www.reddit.com/r/myog/
    /// - `r/myog search terms` -> https://www.reddit.com/r/myog/search/?q=search%20terms
    pub fn process_subreddit_prefix(full_args: &str) -> String {
        // full_args is the raw input, e.g. "r/myog" or "r/myog rust async"
        let (subreddit, search_terms) = match full_args.find(' ') {
            Some(space_idx) => {
                let subreddit = full_args[..space_idx].strip_prefix("r/").unwrap_or("");
                let search_terms = full_args[space_idx + 1..].trim();
                (subreddit, search_terms)
            }
            None => {
                let subreddit = full_args.strip_prefix("r/").unwrap_or("");
                (subreddit, "")
            }
        };

        if search_terms.is_empty() {
            format!("https://www.reddit.com/r/{}/", subreddit)
        } else {
            build_search_url(
                &format!("https://www.reddit.com/r/{}/search/", subreddit),
                "q",
                search_terms,
            )
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

    #[test]
    fn test_reddit_subreddit_prefix_direct() {
        assert_eq!(
            RedditCommand::process_subreddit_prefix("r/myog"),
            "https://www.reddit.com/r/myog/"
        );
    }

    #[test]
    fn test_reddit_subreddit_prefix_search() {
        assert_eq!(
            RedditCommand::process_subreddit_prefix("r/rust async await"),
            "https://www.reddit.com/r/rust/search/?q=async%20await"
        );
    }
}
