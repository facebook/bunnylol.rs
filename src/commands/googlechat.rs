/// Google Chat command handler
/// Supports: gchat -> redirects to Google Chat
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

pub struct GoogleChatCommand;

impl BunnylolCommand for GoogleChatCommand {
    const BINDINGS: &'static [&'static str] = &["gchat"];

    fn process_args(_args: &str) -> String {
        "https://chat.google.com/".to_string()
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo::new(Self::BINDINGS, "Navigate to Google Chat", "gchat")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_chat_command() {
        assert_eq!(
            GoogleChatCommand::process_args("gchat"),
            "https://chat.google.com/"
        );
    }

    #[test]
    fn test_google_chat_command_with_args() {
        assert_eq!(
            GoogleChatCommand::process_args("gchat some args"),
            "https://chat.google.com/"
        );
    }
}
