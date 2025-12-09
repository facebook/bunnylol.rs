/// Claude command handler
/// Supports: claude -> redirects to claude.ai
use crate::utils::bunnylol_command::{BunnylolCommand, CommandInfo};

pub struct ClaudeCommand;

impl BunnylolCommand for ClaudeCommand {
    const BINDINGS: &'static [&'static str] = &["claude"];

    fn process_args(_args: &str) -> String {
        "https://claude.ai".to_string()
    }

    fn get_info() -> CommandInfo {
        CommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Claude AI".to_string(),
            example: "claude".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claude_command() {
        assert_eq!(ClaudeCommand::process_args("claude"), "https://claude.ai");
    }

    #[test]
    fn test_claude_command_with_args() {
        assert_eq!(
            ClaudeCommand::process_args("claude some args"),
            "https://claude.ai"
        );
    }
}
