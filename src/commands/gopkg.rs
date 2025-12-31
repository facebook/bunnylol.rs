/// Go package command handler
/// Supports:
/// - go/golang/gopkg -> https://pkg.go.dev
/// - go [search terms] -> https://pkg.go.dev/search?q=[search terms]
/// - go playground -> https://go.dev/play/
/// - go tour -> https://go.dev/tour/
/// - go docs -> https://go.dev/doc/
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct GopkgCommand;

impl BunnylolCommand for GopkgCommand {
    const BINDINGS: &'static [&'static str] = &["go", "golang", "gopkg"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        match query {
            "" => "https://pkg.go.dev".to_string(),
            "playground" | "play" => "https://go.dev/play/".to_string(),
            "tour" => "https://go.dev/tour/".to_string(),
            "docs" | "doc" => "https://go.dev/doc/".to_string(),
            _ => build_search_url("https://pkg.go.dev/search", "q", query),
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to pkg.go.dev or search for Go packages".to_string(),
            example: "go http".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gopkg_command_base() {
        assert_eq!(GopkgCommand::process_args("go"), "https://pkg.go.dev");
        assert_eq!(GopkgCommand::process_args("golang"), "https://pkg.go.dev");
        assert_eq!(GopkgCommand::process_args("gopkg"), "https://pkg.go.dev");
    }

    #[test]
    fn test_gopkg_command_search() {
        assert_eq!(
            GopkgCommand::process_args("go http"),
            "https://pkg.go.dev/search?q=http"
        );
        assert_eq!(
            GopkgCommand::process_args("gopkg gorilla/mux"),
            "https://pkg.go.dev/search?q=gorilla/mux"
        );
    }

    #[test]
    fn test_gopkg_command_playground() {
        assert_eq!(
            GopkgCommand::process_args("go playground"),
            "https://go.dev/play/"
        );
        assert_eq!(
            GopkgCommand::process_args("go play"),
            "https://go.dev/play/"
        );
    }

    #[test]
    fn test_gopkg_command_tour() {
        assert_eq!(
            GopkgCommand::process_args("go tour"),
            "https://go.dev/tour/"
        );
    }

    #[test]
    fn test_gopkg_command_docs() {
        assert_eq!(GopkgCommand::process_args("go docs"), "https://go.dev/doc/");
        assert_eq!(GopkgCommand::process_args("go doc"), "https://go.dev/doc/");
    }
}
