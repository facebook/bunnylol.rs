/// Cargo/Crates command handler
/// Supports:
/// - cargo -> https://crates.io
/// - cargo [search terms] -> https://crates.io/search?q=[search terms]
/// - cargo settings -> https://crates.io/settings/profile
/// - cargo tokens/api -> https://crates.io/settings/tokens
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct CargoCommand;

impl BunnylolCommand for CargoCommand {
    const BINDINGS: &'static [&'static str] = &["cargo", "crates"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        match query {
            "" => "https://crates.io".to_string(),
            "settings" => "https://crates.io/settings/profile".to_string(),
            "tokens" | "api" => "https://crates.io/settings/tokens".to_string(),
            _ => build_search_url("https://crates.io/search", "q", query),
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to crates.io or search for Rust crates".to_string(),
            example: "cargo serde".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_command_base() {
        assert_eq!(CargoCommand::process_args("cargo"), "https://crates.io");
        assert_eq!(CargoCommand::process_args("crates"), "https://crates.io");
    }

    #[test]
    fn test_cargo_command_search() {
        assert_eq!(
            CargoCommand::process_args("cargo serde"),
            "https://crates.io/search?q=serde"
        );
        assert_eq!(
            CargoCommand::process_args("crates tokio async"),
            "https://crates.io/search?q=tokio%20async"
        );
    }

    #[test]
    fn test_cargo_command_settings() {
        assert_eq!(
            CargoCommand::process_args("cargo settings"),
            "https://crates.io/settings/profile"
        );
    }

    #[test]
    fn test_cargo_command_tokens() {
        assert_eq!(
            CargoCommand::process_args("cargo tokens"),
            "https://crates.io/settings/tokens"
        );
        assert_eq!(
            CargoCommand::process_args("cargo api"),
            "https://crates.io/settings/tokens"
        );
    }
}
