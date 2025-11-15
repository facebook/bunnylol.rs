/// Meta command handler
/// Supports: meta -> redirects to Meta.com
/// Supports: meta accounts/account -> redirects to Meta Accounts Center
use crate::utils::bunnylol_command::{BunnylolCommand, CommandInfo};

pub struct MetaCommand;

impl BunnylolCommand for MetaCommand {
    const BINDINGS: &'static [&'static str] = &["meta"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        match query {
            "accounts" | "account" => "https://accountscenter.meta.com".to_string(),
            _ => "https://www.meta.com".to_string(),
        }
    }

    fn get_info() -> CommandInfo {
        CommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Meta or Meta Accounts Center".to_string(),
            example: "meta accounts".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_command() {
        assert_eq!(
            MetaCommand::process_args("meta"),
            "https://www.meta.com"
        );
    }

    #[test]
    fn test_meta_command_accounts() {
        assert_eq!(
            MetaCommand::process_args("meta accounts"),
            "https://accountscenter.meta.com"
        );
    }

    #[test]
    fn test_meta_command_account() {
        assert_eq!(
            MetaCommand::process_args("meta account"),
            "https://accountscenter.meta.com"
        );
    }

    #[test]
    fn test_meta_command_with_other_args() {
        assert_eq!(
            MetaCommand::process_args("meta some args"),
            "https://www.meta.com"
        );
    }
}
