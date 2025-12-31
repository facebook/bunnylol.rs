/// Chocolatey command handler
/// Supports:
/// - choco/chocolatey -> https://community.chocolatey.org
/// - choco [search terms] -> https://community.chocolatey.org/packages?q=[search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct ChocoCommand;

impl BunnylolCommand for ChocoCommand {
    const BINDINGS: &'static [&'static str] = &["choco", "chocolatey"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://community.chocolatey.org".to_string()
        } else {
            build_search_url("https://community.chocolatey.org/packages", "q", query)
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to community.chocolatey.org or search for Windows packages"
                .to_string(),
            example: "choco git".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choco_command_base() {
        assert_eq!(
            ChocoCommand::process_args("choco"),
            "https://community.chocolatey.org"
        );
        assert_eq!(
            ChocoCommand::process_args("chocolatey"),
            "https://community.chocolatey.org"
        );
    }

    #[test]
    fn test_choco_command_search() {
        assert_eq!(
            ChocoCommand::process_args("choco git"),
            "https://community.chocolatey.org/packages?q=git"
        );
        assert_eq!(
            ChocoCommand::process_args("chocolatey vscode"),
            "https://community.chocolatey.org/packages?q=vscode"
        );
    }
}
