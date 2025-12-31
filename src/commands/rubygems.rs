/// RubyGems command handler
/// Supports:
/// - rubygems/gem/gems -> https://rubygems.org
/// - rubygems [search terms] -> https://rubygems.org/search?query=[search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::encode_url;

pub struct RubygemsCommand;

impl BunnylolCommand for RubygemsCommand {
    const BINDINGS: &'static [&'static str] = &["rubygems", "gem", "gems"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://rubygems.org".to_string()
        } else {
            format!("https://rubygems.org/search?query={}", encode_url(query))
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to rubygems.org or search for Ruby gems".to_string(),
            example: "gem rails".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rubygems_command_base() {
        assert_eq!(RubygemsCommand::process_args("gem"), "https://rubygems.org");
        assert_eq!(
            RubygemsCommand::process_args("rubygems"),
            "https://rubygems.org"
        );
        assert_eq!(
            RubygemsCommand::process_args("gems"),
            "https://rubygems.org"
        );
    }

    #[test]
    fn test_rubygems_command_search() {
        assert_eq!(
            RubygemsCommand::process_args("gem rails"),
            "https://rubygems.org/search?query=rails"
        );
        assert_eq!(
            RubygemsCommand::process_args("rubygems devise authentication"),
            "https://rubygems.org/search?query=devise%20authentication"
        );
    }
}
