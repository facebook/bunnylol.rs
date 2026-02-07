/// Go documentation command handler
/// Supports:
/// - godocs -> https://go.dev/doc/
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

pub struct GodocsCommand;

impl BunnylolCommand for GodocsCommand {
    const BINDINGS: &'static [&'static str] = &["godocs"];

    fn process_args(_args: &str) -> String {
        // Always redirect to Go documentation
        "https://go.dev/doc/".to_string()
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo::new(
            Self::BINDINGS,
            "Navigate to Go language documentation",
            "godocs",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_godocs_command() {
        assert_eq!(GodocsCommand::process_args("godocs"), "https://go.dev/doc/");
        assert_eq!(
            GodocsCommand::process_args("godocs anything"),
            "https://go.dev/doc/"
        );
    }
}
