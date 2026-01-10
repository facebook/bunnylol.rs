/// ProtonDrive command handler
/// Supports: pdrive (redirect to ProtonDrive), pdrive [filename] -> https://drive.proton.me/u/1/search#q=[filename])
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url_with_attribute_separator;

pub struct ProtonDriveCommand;

impl BunnylolCommand for ProtonDriveCommand {
    const BINDINGS: &'static [&'static str] = &["pdrive"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://drive.proton.me".to_string()
        } else {
            build_search_url_with_attribute_separator(
                "https://drive.proton.me/u/1/search",
                "q",
                query,
                "#",
            )
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to ProtonDrive or search for files".to_string(),
            example: "pdrive".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdrive_command() {
        assert_eq!(
            ProtonDriveCommand::process_args("pdrive"),
            "https://drive.proton.me"
        );
    }

    #[test]
    fn test_pdrive_command_search() {
        assert_eq!(
            ProtonDriveCommand::process_args("pdrive file1"),
            "https://drive.proton.me/u/1/search#q=file1"
        );
    }
}
