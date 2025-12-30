/// WhatsApp command handler
/// Supports: wa, whatsapp -> redirects to WhatsApp Web
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

pub struct WhatsAppCommand;

impl BunnylolCommand for WhatsAppCommand {
    const BINDINGS: &'static [&'static str] = &["wa", "whatsapp"];

    fn process_args(_args: &str) -> String {
        "https://www.whatsapp.com".to_string()
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to WhatsApp".to_string(),
            example: "wa".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whatsapp_command() {
        assert_eq!(
            WhatsAppCommand::process_args("wa"),
            "https://www.whatsapp.com"
        );
    }

    #[test]
    fn test_whatsapp_command_full_name() {
        assert_eq!(
            WhatsAppCommand::process_args("whatsapp"),
            "https://www.whatsapp.com"
        );
    }

    #[test]
    fn test_whatsapp_command_with_args() {
        assert_eq!(
            WhatsAppCommand::process_args("wa some args"),
            "https://www.whatsapp.com"
        );
    }
}
