/// Homebrew command handler
/// Supports:
/// - brew/homebrew -> https://formulae.brew.sh
/// - brew [search terms] -> https://formulae.brew.sh/?search=[search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::encode_url;

pub struct BrewCommand;

impl BunnylolCommand for BrewCommand {
    const BINDINGS: &'static [&'static str] = &["brew", "homebrew"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://formulae.brew.sh".to_string()
        } else {
            format!("https://formulae.brew.sh/?search={}", encode_url(query))
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to formulae.brew.sh or search for Homebrew packages".to_string(),
            example: "brew wget".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brew_command_base() {
        assert_eq!(
            BrewCommand::process_args("brew"),
            "https://formulae.brew.sh"
        );
        assert_eq!(
            BrewCommand::process_args("homebrew"),
            "https://formulae.brew.sh"
        );
    }

    #[test]
    fn test_brew_command_search() {
        assert_eq!(
            BrewCommand::process_args("brew wget"),
            "https://formulae.brew.sh/?search=wget"
        );
        assert_eq!(
            BrewCommand::process_args("homebrew nginx"),
            "https://formulae.brew.sh/?search=nginx"
        );
    }
}
