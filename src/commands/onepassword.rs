/// 1Password command
///
/// Shortcut to 1Password home page
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

pub struct OnePasswordCommand;

impl BunnylolCommand for OnePasswordCommand {
    const BINDINGS: &'static [&'static str] = &["1password", "1p", "onepassword"];

    fn process_args(_args: &str) -> String {
        "https://my.1password.com/home".to_string()
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "1Password home page".to_string(),
            example: "1p".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_onepassword_1password() {
        assert_eq!(
            OnePasswordCommand::process_args("1password"),
            "https://my.1password.com/home"
        );
    }

    #[test]
    fn test_onepassword_1p() {
        assert_eq!(
            OnePasswordCommand::process_args("1p"),
            "https://my.1password.com/home"
        );
    }

    #[test]
    fn test_onepassword_onepassword() {
        assert_eq!(
            OnePasswordCommand::process_args("onepassword"),
            "https://my.1password.com/home"
        );
    }
}
