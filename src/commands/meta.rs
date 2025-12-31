/// Meta command handler
/// Supports: meta -> redirects to Meta.com
/// Supports: meta accounts/account -> redirects to Meta Accounts Center
/// Supports: metaai/meta ai -> redirects to Meta AI
/// Supports: meta pay -> redirects to Meta Pay
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

pub struct MetaCommand;

impl BunnylolCommand for MetaCommand {
    const BINDINGS: &'static [&'static str] = &["meta", "metaai"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        match query {
            "accounts" | "account" => "https://accountscenter.meta.com".to_string(),
            "ai" => "https://www.meta.ai".to_string(),
            "pay" => {
                "https://accountscenter.meta.com/meta_pay_wallet/?referrer=accounts_center_home"
                    .to_string()
            }
            "" if args.starts_with("metaai") => "https://www.meta.ai".to_string(),
            _ => "https://www.meta.com".to_string(),
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Meta, Meta AI, Meta Accounts Center, or Meta Pay".to_string(),
            example: "meta accounts".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_command() {
        assert_eq!(MetaCommand::process_args("meta"), "https://www.meta.com");
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
    fn test_meta_command_ai() {
        assert_eq!(MetaCommand::process_args("meta ai"), "https://www.meta.ai");
    }

    #[test]
    fn test_metaai_command() {
        assert_eq!(MetaCommand::process_args("metaai"), "https://www.meta.ai");
    }

    #[test]
    fn test_meta_command_with_other_args() {
        assert_eq!(
            MetaCommand::process_args("meta some args"),
            "https://www.meta.com"
        );
    }

    #[test]
    fn test_meta_command_pay() {
        assert_eq!(
            MetaCommand::process_args("meta pay"),
            "https://accountscenter.meta.com/meta_pay_wallet/?referrer=accounts_center_home"
        );
    }
}
