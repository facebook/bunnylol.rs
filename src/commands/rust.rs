/// Rust command handler
/// Supports:
/// - rust -> https://doc.rust-lang.org/stable/std/index.html
/// - rust [search terms] -> https://doc.rust-lang.org/stable/std/index.html?search=[search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct RustCommand;

impl BunnylolCommand for RustCommand {
    const BINDINGS: &'static [&'static str] = &["rust"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://doc.rust-lang.org/stable/std/index.html".to_string()
        } else {
            build_search_url(
                "https://doc.rust-lang.org/stable/std/index.html",
                "search",
                query,
            )
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Rust documentation or search Rust std docs".to_string(),
            example: "rust HashMap".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_command_base() {
        assert_eq!(
            RustCommand::process_args("rust"),
            "https://doc.rust-lang.org/stable/std/index.html"
        );
    }

    #[test]
    fn test_rust_command_search() {
        assert_eq!(
            RustCommand::process_args("rust HashMap"),
            "https://doc.rust-lang.org/stable/std/index.html?search=HashMap"
        );
        assert_eq!(
            RustCommand::process_args("rust Vec String"),
            "https://doc.rust-lang.org/stable/std/index.html?search=Vec%20String"
        );
    }
}
