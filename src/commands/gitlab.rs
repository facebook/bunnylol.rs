/// GitLab command handler
/// Supports:
/// - gitlab/gl -> https://gitlab.com
/// - gitlab [user/project] -> https://gitlab.com/[user/project]
/// - gitlab [search terms] -> https://gitlab.com/search?search=[search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::encode_url;

pub struct GitlabCommand;

impl BunnylolCommand for GitlabCommand {
    const BINDINGS: &'static [&'static str] = &["gitlab", "gl"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://gitlab.com".to_string()
        } else if query.contains('/') {
            // Validate and encode project path (user/project format)
            let parts: Vec<&str> = query.split('/').collect();
            if parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() {
                // Valid user/project format - encode each part
                format!(
                    "https://gitlab.com/{}/{}",
                    encode_url(parts[0]),
                    encode_url(parts[1])
                )
            } else {
                // Invalid path format (e.g., foo//bar or foo/bar/baz), fall back to search
                format!("https://gitlab.com/search?search={}", encode_url(query))
            }
        } else {
            // Otherwise, treat it as a search query
            format!("https://gitlab.com/search?search={}", encode_url(query))
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to GitLab projects or search GitLab".to_string(),
            example: "gitlab gitlab-org/gitlab".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gitlab_command_base() {
        assert_eq!(GitlabCommand::process_args("gitlab"), "https://gitlab.com");
        assert_eq!(GitlabCommand::process_args("gl"), "https://gitlab.com");
    }

    #[test]
    fn test_gitlab_command_project() {
        assert_eq!(
            GitlabCommand::process_args("gitlab gitlab-org/gitlab"),
            "https://gitlab.com/gitlab-org/gitlab"
        );
        assert_eq!(
            GitlabCommand::process_args("gl user/project"),
            "https://gitlab.com/user/project"
        );
    }

    #[test]
    fn test_gitlab_command_search() {
        assert_eq!(
            GitlabCommand::process_args("gitlab kubernetes"),
            "https://gitlab.com/search?search=kubernetes"
        );
        assert_eq!(
            GitlabCommand::process_args("gl rust async"),
            "https://gitlab.com/search?search=rust%20async"
        );
    }

    #[test]
    fn test_gitlab_command_invalid_paths() {
        // Multiple slashes should fall back to search
        assert_eq!(
            GitlabCommand::process_args("gl org/team/project"),
            "https://gitlab.com/search?search=org/team/project"
        );
        // Double slash should fall back to search
        assert_eq!(
            GitlabCommand::process_args("gl foo//bar"),
            "https://gitlab.com/search?search=foo//bar"
        );
        // Leading slash should fall back to search
        assert_eq!(
            GitlabCommand::process_args("gl /foo"),
            "https://gitlab.com/search?search=/foo"
        );
        // Trailing slash should fall back to search
        assert_eq!(
            GitlabCommand::process_args("gl foo/"),
            "https://gitlab.com/search?search=foo/"
        );
    }

    #[test]
    fn test_gitlab_command_special_chars() {
        // Special characters should be encoded in paths
        assert_eq!(
            GitlabCommand::process_args("gl user-name/project-name"),
            "https://gitlab.com/user-name/project-name"
        );
        // Spaces in project names should be encoded
        assert_eq!(
            GitlabCommand::process_args("gl user/my project"),
            "https://gitlab.com/user/my%20project"
        );
    }
}
