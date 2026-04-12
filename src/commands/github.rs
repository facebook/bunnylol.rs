/// GitHub command handler
/// Supports: gh, gh @[user], gh [user/repo], gh token[s]/pat, gh settings, gh bills/billing,
/// gh notifications/notifs, gh teams, gh orgs, gh ssh/gpg/keys, gh security/passwords/auth/mfa/2fa,
/// gh emails, gh [search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::{build_path_url, build_search_url};

pub struct GitHubCommand;

impl BunnylolCommand for GitHubCommand {
    const BINDINGS: &'static [&'static str] = &["gh"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://github.com".to_string()
        } else if query == "settings" {
            "https://github.com/settings/profile".to_string()
        } else if query == "token" || query == "tokens" || query == "pat" {
            "https://github.com/settings/personal-access-tokens".to_string()
        } else if query == "bills" || query == "billing" {
            "https://github.com/settings/billing".to_string()
        } else if query == "notifications" || query == "notifs" {
            "https://github.com/settings/notifications".to_string()
        } else if query == "teams" {
            "https://github.com/settings/teams".to_string()
        } else if query == "orgs" {
            "https://github.com/settings/organizations".to_string()
        } else if query == "ssh" || query == "gpg" || query == "keys" {
            "https://github.com/settings/keys".to_string()
        } else if query == "security"
            || query == "passwords"
            || query == "auth"
            || query == "mfa"
            || query == "2fa"
        {
            "https://github.com/settings/security".to_string()
        } else if query == "emails" {
            "https://github.com/settings/emails".to_string()
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

    #[test]
    fn test_github_command_token() {
        assert_eq!(
            GitHubCommand::process_args("gh token"),
            "https://github.com/settings/personal-access-tokens"
        );
    }

    #[test]
    fn test_github_command_tokens() {
        assert_eq!(
            GitHubCommand::process_args("gh tokens"),
            "https://github.com/settings/personal-access-tokens"
        );
    }

    #[test]
    fn test_github_command_pat() {
        assert_eq!(
            GitHubCommand::process_args("gh pat"),
            "https://github.com/settings/personal-access-tokens"
        );
    }

    #[test]
    fn test_github_command_settings() {
        assert_eq!(
            GitHubCommand::process_args("gh settings"),
            "https://github.com/settings/profile"
        );
    }

    #[test]
    fn test_github_command_bills() {
        assert_eq!(
            GitHubCommand::process_args("gh bills"),
            "https://github.com/settings/billing"
        );
    }

    #[test]
    fn test_github_command_billing() {
        assert_eq!(
            GitHubCommand::process_args("gh billing"),
            "https://github.com/settings/billing"
        );
    }

    #[test]
    fn test_github_command_notifications() {
        assert_eq!(
            GitHubCommand::process_args("gh notifications"),
            "https://github.com/settings/notifications"
        );
    }

    #[test]
    fn test_github_command_notifs() {
        assert_eq!(
            GitHubCommand::process_args("gh notifs"),
            "https://github.com/settings/notifications"
        );
    }

    #[test]
    fn test_github_command_teams() {
        assert_eq!(
            GitHubCommand::process_args("gh teams"),
            "https://github.com/settings/teams"
        );
    }

    #[test]
    fn test_github_command_orgs() {
        assert_eq!(
            GitHubCommand::process_args("gh orgs"),
            "https://github.com/settings/organizations"
        );
    }

    #[test]
    fn test_github_command_ssh() {
        assert_eq!(
            GitHubCommand::process_args("gh ssh"),
            "https://github.com/settings/keys"
        );
    }

    #[test]
    fn test_github_command_gpg() {
        assert_eq!(
            GitHubCommand::process_args("gh gpg"),
            "https://github.com/settings/keys"
        );
    }

    #[test]
    fn test_github_command_keys() {
        assert_eq!(
            GitHubCommand::process_args("gh keys"),
            "https://github.com/settings/keys"
        );
    }

    #[test]
    fn test_github_command_security() {
        assert_eq!(
            GitHubCommand::process_args("gh security"),
            "https://github.com/settings/security"
        );
    }

    #[test]
    fn test_github_command_passwords() {
        assert_eq!(
            GitHubCommand::process_args("gh passwords"),
            "https://github.com/settings/security"
        );
    }

    #[test]
    fn test_github_command_auth() {
        assert_eq!(
            GitHubCommand::process_args("gh auth"),
            "https://github.com/settings/security"
        );
    }

    #[test]
    fn test_github_command_mfa() {
        assert_eq!(
            GitHubCommand::process_args("gh mfa"),
            "https://github.com/settings/security"
        );
    }

    #[test]
    fn test_github_command_2fa() {
        assert_eq!(
            GitHubCommand::process_args("gh 2fa"),
            "https://github.com/settings/security"
        );
    }

    #[test]
    fn test_github_command_emails() {
        assert_eq!(
            GitHubCommand::process_args("gh emails"),
            "https://github.com/settings/emails"
        );
    }
}
