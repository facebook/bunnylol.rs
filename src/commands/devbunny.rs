/// DevBunny command handler
/// Supports: devbunny [command] -> http://localhost:8000/?cmd=[command]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

pub struct DevBunnyCommand;

impl BunnylolCommand for DevBunnyCommand {
    const BINDINGS: &'static [&'static str] = &["devbunny"];

    fn process_args(args: &str) -> String {
        let cmd_part = Self::get_command_args(args);
        format!(
            "http://localhost:8000/?cmd={}",
            utf8_percent_encode(cmd_part, NON_ALPHANUMERIC)
        )
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Test Bunnylol commands in development".to_string(),
            example: "devbunny gh facebook".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_devbunny_command_with_args() {
        assert_eq!(
            DevBunnyCommand::process_args("devbunny test query"),
            "http://localhost:8000/?cmd=test%20query"
        );
    }

    #[test]
    fn test_devbunny_command_base() {
        assert_eq!(
            DevBunnyCommand::process_args("devbunny"),
            "http://localhost:8000/?cmd="
        );
    }
}
