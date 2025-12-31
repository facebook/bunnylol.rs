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
            // If query contains '/', treat it as a project path
            format!("https://gitlab.com/{}", query)
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
}
