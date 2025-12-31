/// Threads command handler
/// Supports: threads, threads @[username], threads [search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::{build_path_url, build_search_url};

pub struct ThreadsCommand;

impl ThreadsCommand {
    fn construct_profile_url(profile: &str) -> String {
        build_path_url("https://www.threads.net", &format!("@{}", profile))
    }

    fn construct_search_url(query: &str) -> String {
        build_search_url("https://www.threads.net/search", "q", query)
    }
}

impl BunnylolCommand for ThreadsCommand {
    const BINDINGS: &'static [&'static str] = &["threads"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://www.threads.net".to_string()
        } else {
            // Check if it looks like a Threads profile
            if let Some(username) = query.strip_prefix('@') {
                if !username.is_empty() {
                    Self::construct_profile_url(username)
                } else {
                    // Just '@' with no username - go to homepage
                    "https://www.threads.net".to_string()
                }
            } else {
                Self::construct_search_url(query)
            }
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Threads profiles or search Threads".to_string(),
            example: "threads @zuck".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threads_command_base() {
        assert_eq!(
            ThreadsCommand::process_args("threads"),
            "https://www.threads.net"
        );
    }

    #[test]
    fn test_threads_command_profile() {
        assert_eq!(
            ThreadsCommand::process_args("threads @zuck"),
            "https://www.threads.net/@zuck"
        );
    }

    #[test]
    fn test_threads_command_search() {
        assert_eq!(
            ThreadsCommand::process_args("threads tech news"),
            "https://www.threads.net/search?q=tech%20news"
        );
    }

    #[test]
    fn test_threads_command_empty_username() {
        assert_eq!(
            ThreadsCommand::process_args("threads @"),
            "https://www.threads.net"
        );
    }
}
