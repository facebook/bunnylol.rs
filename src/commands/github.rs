/// GitHub command handler
/// Supports: gh, gh @[user], gh [user/repo], gh [search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::{build_path_url, build_search_url};

pub struct GitHubCommand;

impl BunnylolCommand for GitHubCommand {
    const BINDINGS: &'static [&'static str] = &["gh"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://github.com".to_string()
        } else if let Some(username) = query.strip_prefix('@') {
            if username.is_empty() {
                "https://github.com".to_string()
            } else {
                build_path_url("https://github.com", username)
            }
        } else if let Some((author, repo)) = query.split_once('/') {
            if !author.is_empty() && !repo.is_empty() {
                build_path_url("https://github.com", query)
            } else {
                format!(
                    "{}&type=repositories",
                    build_search_url("https://github.com/search", "q", query)
                )
            }
        } else {
            format!(
                "{}&type=repositories",
                build_search_url("https://github.com/search", "q", query)
            )
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo::new(
            Self::BINDINGS,
            "Navigate to GitHub profiles, repositories, or search GitHub",
            "gh facebook/react",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_command_base() {
        assert_eq!(GitHubCommand::process_args("gh"), "https://github.com");
    }

    #[test]
    fn test_github_command_profile() {
        assert_eq!(
            GitHubCommand::process_args("gh @facebook"),
            "https://github.com/facebook"
        );
    }

    #[test]
    fn test_github_command_empty_username() {
        assert_eq!(GitHubCommand::process_args("gh @"), "https://github.com");
    }

    #[test]
    fn test_github_command_repo() {
        assert_eq!(
            GitHubCommand::process_args("gh facebook/react"),
            "https://github.com/facebook/react"
        );
    }

    #[test]
    fn test_github_command_search() {
        assert_eq!(
            GitHubCommand::process_args("gh rust async"),
            "https://github.com/search?q=rust%20async&type=repositories"
        );
    }

    #[test]
    fn test_github_command_search_single_word() {
        assert_eq!(
            GitHubCommand::process_args("gh react"),
            "https://github.com/search?q=react&type=repositories"
        );
    }
}
