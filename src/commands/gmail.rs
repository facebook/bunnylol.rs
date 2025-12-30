/// Gmail command handler
/// Supports: mail (simple redirect to Gmail)
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

pub struct GmailCommand;

impl BunnylolCommand for GmailCommand {
    const BINDINGS: &'static [&'static str] = &["gmail", "mail"];

    fn process_args(_args: &str) -> String {
        "https://mail.google.com".to_string()
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Gmail".to_string(),
            example: "mail".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gmail_command() {
        assert_eq!(
            GmailCommand::process_args("mail"),
            "https://mail.google.com"
        );
    }

    #[test]
    fn test_gmail_command_with_args() {
        // Gmail command ignores any additional arguments
        assert_eq!(
            GmailCommand::process_args("mail some args"),
            "https://mail.google.com"
        );
    }
}
