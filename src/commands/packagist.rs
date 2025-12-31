/// Packagist command handler
/// Supports:
/// - packagist/composer -> https://packagist.org
/// - packagist [search terms] -> https://packagist.org/search/?query=[search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::encode_url;

pub struct PackagistCommand;

impl BunnylolCommand for PackagistCommand {
    const BINDINGS: &'static [&'static str] = &["packagist", "composer"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://packagist.org".to_string()
        } else {
            format!("https://packagist.org/search/?query={}", encode_url(query))
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to packagist.org or search for PHP packages".to_string(),
            example: "packagist symfony".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packagist_command_base() {
        assert_eq!(
            PackagistCommand::process_args("packagist"),
            "https://packagist.org"
        );
        assert_eq!(
            PackagistCommand::process_args("composer"),
            "https://packagist.org"
        );
    }

    #[test]
    fn test_packagist_command_search() {
        assert_eq!(
            PackagistCommand::process_args("packagist symfony"),
            "https://packagist.org/search/?query=symfony"
        );
        assert_eq!(
            PackagistCommand::process_args("composer laravel"),
            "https://packagist.org/search/?query=laravel"
        );
    }
}
