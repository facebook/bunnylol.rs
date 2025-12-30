/// ChatGPT command handler
/// Supports: chatgpt -> redirects to chatgpt.com
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

pub struct ChatGPTCommand;

impl BunnylolCommand for ChatGPTCommand {
    const BINDINGS: &'static [&'static str] = &["chatgpt"];

    fn process_args(_args: &str) -> String {
        "https://chatgpt.com".to_string()
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to ChatGPT".to_string(),
            example: "chatgpt".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chatgpt_command() {
        assert_eq!(
            ChatGPTCommand::process_args("chatgpt"),
            "https://chatgpt.com"
        );
    }

    #[test]
    fn test_chatgpt_command_with_args() {
        assert_eq!(
            ChatGPTCommand::process_args("chatgpt some args"),
            "https://chatgpt.com"
        );
    }
}
