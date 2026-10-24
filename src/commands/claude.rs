/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

/// Claude command handler
/// Supports:
/// - claude -> https://claude.ai
/// - claude platform/api/keys/apikey/billing/cost/artifacts/chats/projects/usage/upgrade
///   -> the matching claude.ai or platform.claude.com page
/// - claude [prompt] -> https://claude.ai/new?q=[prompt] (new chat with the prompt)
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

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
            "" => "https://claude.ai".to_string(),
            _ => build_search_url("https://claude.ai/new", "q", query),
        };

        println!("   >> ClaudeCommand::process_args - returning: {}", result);
        result
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo::new(
            Self::BINDINGS,
            "Navigate to Claude AI or start a chat with a prompt (supports: billing, cost, artifacts, chats, projects)",
            "claude explain rust lifetimes",
        )
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
    fn test_claude_command_binding_only() {
        assert_eq!(ClaudeCommand::process_args("claude"), "https://claude.ai");
    }

    #[test]
    fn test_claude_command_with_args() {
        assert_eq!(
            ClaudeCommand::process_args("some args"),
            "https://claude.ai/new?q=some%20args"
        );
    }

    #[test]
    fn test_claude_prompt() {
        assert_eq!(
            ClaudeCommand::process_args("claude explain rust lifetimes"),
            "https://claude.ai/new?q=explain%20rust%20lifetimes"
        );
    }

    #[test]
    fn test_claude_prompt_special_chars() {
        assert_eq!(
            ClaudeCommand::process_args("claude is 2+2=4 & why?"),
            "https://claude.ai/new?q=is%202%2B2%3D4%20%26%20why?"
        );
    }

    #[test]
    fn test_claude_subcommand_still_wins() {
        assert_eq!(
            ClaudeCommand::process_args("claude projects"),
            "https://claude.ai/projects"
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
