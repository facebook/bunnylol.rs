/// Meta command handler
/// Supports: meta -> redirects to Meta.com
use crate::utils::bunnylol_command::{BunnylolCommand, CommandInfo};

pub struct MetaCommand;

impl BunnylolCommand for MetaCommand {
    const BINDINGS: &'static [&'static str] = &["meta"];

    fn process_args(_args: &str) -> String {
        "https://www.meta.com".to_string()
    }

    fn get_info() -> CommandInfo {
        CommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Meta".to_string(),
            example: "meta".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_command() {
        assert_eq!(
            MetaCommand::process_args("meta"),
            "https://www.meta.com"
        );
    }

    #[test]
    fn test_meta_command_with_args() {
        assert_eq!(
            MetaCommand::process_args("meta some args"),
            "https://www.meta.com"
        );
    }
}
