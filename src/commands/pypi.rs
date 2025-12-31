/// PyPI command handler
/// Supports:
/// - pypi/pip -> https://pypi.org
/// - pypi [search terms] -> https://pypi.org/search/?q=[search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct PypiCommand;

impl BunnylolCommand for PypiCommand {
    const BINDINGS: &'static [&'static str] = &["pypi", "pip"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://pypi.org".to_string()
        } else {
            build_search_url("https://pypi.org/search/", "q", query)
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to pypi.org or search for Python packages".to_string(),
            example: "pypi requests".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pypi_command_base() {
        assert_eq!(PypiCommand::process_args("pypi"), "https://pypi.org");
        assert_eq!(PypiCommand::process_args("pip"), "https://pypi.org");
    }

    #[test]
    fn test_pypi_command_search() {
        assert_eq!(
            PypiCommand::process_args("pypi requests"),
            "https://pypi.org/search/?q=requests"
        );
        assert_eq!(
            PypiCommand::process_args("pip django rest framework"),
            "https://pypi.org/search/?q=django%20rest%20framework"
        );
    }
}
