/// Gmail command handler
/// Supports: mail (simple redirect to Gmail)
use crate::utils::bunnylol_command::{BunnylolCommand, CommandInfo};

pub struct GmailCommand;

impl BunnylolCommand for GmailCommand {
    const COMMAND: &'static str = "mail";

    fn process_args(_args: &str) -> String {
        "https://mail.google.com".to_string()
    }

    fn get_info() -> CommandInfo {
        CommandInfo {
            command: Self::COMMAND.to_string(),
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
