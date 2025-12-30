/// Claude command handler
/// Supports: claude -> redirects to claude.ai
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

pub struct ClaudeCommand;

impl BunnylolCommand for ClaudeCommand {
    const BINDINGS: &'static [&'static str] = &["claude"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        println!(
            "ClaudeCommand::process_args - input: {:?}, query: {:?}",
            args, query
        );

        let result = match query {
            "platform" => "https://platform.claude.com".to_string(),
            "api" | "keys" | "apikey" => "https://platform.claude.com/settings/keys".to_string(),
            "billing" | "cost" => "https://claude.ai/settings/billing".to_string(),
            "artifacts" => "https://claude.ai/artifacts".to_string(),
            "artifacts my" => "https://claude.ai/artifacts/my".to_string(),
            "chats" => "https://claude.ai/recents".to_string(),
            "projects" => "https://claude.ai/projects".to_string(),
            "usage" => "https://claude.ai/settings/usage".to_string(),
            "upgrade" => "https://claude.ai/upgrade".to_string(),
            _ => "https://claude.ai".to_string(),
        };

        println!("   >> ClaudeCommand::process_args - returning: {}", result);
        result
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description:
                "Navigate to Claude AI (supports: billing, cost, artifacts, chats, projects)"
                    .to_string(),
            example: "claude projects".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claude_command() {
        assert_eq!(ClaudeCommand::process_args(""), "https://claude.ai");
    }

    #[test]
    fn test_claude_command_with_args() {
        assert_eq!(
            ClaudeCommand::process_args("some args"),
            "https://claude.ai"
        );
    }

    #[test]
    fn test_claude_billing() {
        assert_eq!(
            ClaudeCommand::process_args("billing"),
            "https://claude.ai/settings/billing"
        );
    }

    #[test]
    fn test_claude_cost() {
        assert_eq!(
            ClaudeCommand::process_args("cost"),
            "https://claude.ai/settings/billing"
        );
    }

    #[test]
    fn test_claude_artifacts() {
        assert_eq!(
            ClaudeCommand::process_args("artifacts"),
            "https://claude.ai/artifacts"
        );
    }

    #[test]
    fn test_claude_artifacts_my() {
        assert_eq!(
            ClaudeCommand::process_args("artifacts my"),
            "https://claude.ai/artifacts/my"
        );
    }

    #[test]
    fn test_claude_chats() {
        assert_eq!(
            ClaudeCommand::process_args("chats"),
            "https://claude.ai/recents"
        );
    }

    #[test]
    fn test_claude_projects() {
        assert_eq!(
            ClaudeCommand::process_args("projects"),
            "https://claude.ai/projects"
        );
    }
}
