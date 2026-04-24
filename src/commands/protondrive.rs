/// Proton Drive command handler
/// Supports: pdrive (redirect or search Proton Drive)
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url_with_separator;

pub struct ProtonDriveCommand;

impl BunnylolCommand for ProtonDriveCommand {
    const BINDINGS: &'static [&'static str] = &["pdrive"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://drive.proton.me".to_string()
        } else {
            build_search_url_with_separator("https://drive.proton.me/u/1/search", "q", query, "#")
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo::new(
            Self::BINDINGS,
            "Navigate to or search Proton Drive",
            "pdrive my document",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protondrive_command() {
        assert_eq!(
            ProtonDriveCommand::process_args("pdrive"),
            "https://drive.proton.me"
        );
    }

    #[test]
    fn test_protondrive_command_search() {
        assert_eq!(
            ProtonDriveCommand::process_args("pdrive file1"),
            "https://drive.proton.me/u/1/search#q=file1"
        );
    }
}
