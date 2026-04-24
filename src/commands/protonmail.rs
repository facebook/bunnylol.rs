/// Proton Mail command handler
/// Supports: pmail (simple redirect to Proton Mail)
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

pub struct ProtonMailCommand;

impl BunnylolCommand for ProtonMailCommand {
    const BINDINGS: &'static [&'static str] = &["pmail"];

    fn process_args(_args: &str) -> String {
        "https://mail.proton.me".to_string()
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo::new(Self::BINDINGS, "Navigate to Proton Mail", "pmail")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protonmail_command() {
        assert_eq!(
            ProtonMailCommand::process_args("pmail"),
            "https://mail.proton.me"
        );
    }

    #[test]
    fn test_protonmail_command_with_args() {
        assert_eq!(
            ProtonMailCommand::process_args("pmail some args"),
            "https://mail.proton.me"
        );
    }
}
