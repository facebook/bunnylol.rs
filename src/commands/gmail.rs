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
        BunnylolCommandInfo::new(
            Self::BINDINGS,
            "Navigate to Gmail",
            "mail",
        )
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
