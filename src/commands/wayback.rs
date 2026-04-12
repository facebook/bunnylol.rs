/// Wayback Machine / Internet Archive command handler
/// Supports: wayback, archive
/// wayback [url/search] -> https://web.archive.org/web/*/[url/search]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_path_url;

pub struct WaybackCommand;

impl BunnylolCommand for WaybackCommand {
    const BINDINGS: &'static [&'static str] = &["wayback", "archive"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://web.archive.org/web/".to_string()
        } else {
            build_path_url("https://web.archive.org/web/*", query)
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo::new(
            Self::BINDINGS,
            "Look up a URL in the Wayback Machine / Internet Archive",
            "wayback google.com",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wayback_command_base() {
        assert_eq!(
            WaybackCommand::process_args("wayback"),
            "https://web.archive.org/web/"
        );
    }

    #[test]
    fn test_wayback_command_base_alias() {
        assert_eq!(
            WaybackCommand::process_args("archive"),
            "https://web.archive.org/web/"
        );
    }

    #[test]
    fn test_wayback_command_url() {
        assert_eq!(
            WaybackCommand::process_args("wayback google.com"),
            "https://web.archive.org/web/*/google.com"
        );
    }

    #[test]
    fn test_wayback_command_url_alias() {
        assert_eq!(
            WaybackCommand::process_args("archive google.com"),
            "https://web.archive.org/web/*/google.com"
        );
    }

    #[test]
    fn test_wayback_command_search() {
        assert_eq!(
            WaybackCommand::process_args("wayback some search query"),
            "https://web.archive.org/web/*/some%20search%20query"
        );
    }
}
