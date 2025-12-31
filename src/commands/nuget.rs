/// NuGet command handler
/// Supports:
/// - nuget -> https://www.nuget.org
/// - nuget [search terms] -> https://www.nuget.org/packages?q=[search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct NugetCommand;

impl BunnylolCommand for NugetCommand {
    const BINDINGS: &'static [&'static str] = &["nuget"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://www.nuget.org".to_string()
        } else {
            build_search_url("https://www.nuget.org/packages", "q", query)
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to nuget.org or search for .NET packages".to_string(),
            example: "nuget newtonsoft".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nuget_command_base() {
        assert_eq!(NugetCommand::process_args("nuget"), "https://www.nuget.org");
    }

    #[test]
    fn test_nuget_command_search() {
        assert_eq!(
            NugetCommand::process_args("nuget newtonsoft"),
            "https://www.nuget.org/packages?q=newtonsoft"
        );
        assert_eq!(
            NugetCommand::process_args("nuget entityframework"),
            "https://www.nuget.org/packages?q=entityframework"
        );
    }
}
