/// Bindings command handler
/// Supports: bindings, list -> redirects to the bindings web portal
use crate::utils::bunnylol_command::{BunnylolCommand, CommandInfo};

pub struct BindingsCommand;

impl BunnylolCommand for BindingsCommand {
    const COMMAND: &'static str = "bindings";

    fn process_args(_args: &str) -> String {
        "/bindings".to_string()
    }

    fn get_info() -> CommandInfo {
        CommandInfo {
            command: Self::COMMAND.to_string(),
            description: "View all Bunnylol command bindings in a web portal".to_string(),
            example: "bindings".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bindings_command() {
        assert_eq!(
            BindingsCommand::process_args("bindings"),
            "/bindings"
        );
    }
}
