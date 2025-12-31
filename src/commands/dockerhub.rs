/// Docker Hub command handler
/// Supports:
/// - dockerhub/docker -> https://hub.docker.com
/// - dockerhub [search terms] -> https://hub.docker.com/search?q=[search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct DockerhubCommand;

impl BunnylolCommand for DockerhubCommand {
    const BINDINGS: &'static [&'static str] = &["dockerhub", "docker"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://hub.docker.com".to_string()
        } else {
            build_search_url("https://hub.docker.com/search", "q", query)
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Docker Hub or search for container images".to_string(),
            example: "docker nginx".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dockerhub_command_base() {
        assert_eq!(
            DockerhubCommand::process_args("docker"),
            "https://hub.docker.com"
        );
        assert_eq!(
            DockerhubCommand::process_args("dockerhub"),
            "https://hub.docker.com"
        );
    }

    #[test]
    fn test_dockerhub_command_search() {
        assert_eq!(
            DockerhubCommand::process_args("docker nginx"),
            "https://hub.docker.com/search?q=nginx"
        );
        assert_eq!(
            DockerhubCommand::process_args("dockerhub postgres"),
            "https://hub.docker.com/search?q=postgres"
        );
    }
}
