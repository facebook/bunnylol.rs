/// ProtonMail command handler
/// Supports: pmail (simple redirect to ProtonMail)
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

pub struct ProtonMailCommand;

impl BunnylolCommand for ProtonMailCommand {
    const BINDINGS: &'static [&'static str] = &["pmail"];

    fn process_args(_args: &str) -> String {
        "https://mail.proton.me".to_string()
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Protonmail".to_string(),
            example: "pmail".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pmail_command() {
        assert_eq!(
            ProtonMailCommand::process_args("pmail"),
            "https://mail.proton.me"
        );
    }

    #[test]
    fn test_pmail_command_with_args() {
        assert_eq!(
            ProtonMailCommand::process_args("pmail some args"),
            "https://mail.proton.me"
        );
    }
}
