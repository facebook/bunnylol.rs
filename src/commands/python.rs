/// Python documentation command handler
/// Supports:
/// - python/pydocs/py -> https://docs.python.org/3/
/// - python [search terms] -> https://docs.python.org/3/search.html?q=[search terms]
/// - python tutorial -> https://docs.python.org/3/tutorial/
/// - python library -> https://docs.python.org/3/library/
/// - python reference -> https://docs.python.org/3/reference/
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct PythonCommand;

impl BunnylolCommand for PythonCommand {
    const BINDINGS: &'static [&'static str] = &["python", "pydocs", "py"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        match query {
            "" => "https://docs.python.org/3/".to_string(),
            "tutorial" => "https://docs.python.org/3/tutorial/".to_string(),
            "library" | "lib" => "https://docs.python.org/3/library/".to_string(),
            "reference" | "ref" => "https://docs.python.org/3/reference/".to_string(),
            _ => build_search_url("https://docs.python.org/3/search.html", "q", query),
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Python documentation or search for Python resources"
                .to_string(),
            example: "python list".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_command_base() {
        assert_eq!(
            PythonCommand::process_args("python"),
            "https://docs.python.org/3/"
        );
        assert_eq!(
            PythonCommand::process_args("pydocs"),
            "https://docs.python.org/3/"
        );
        assert_eq!(
            PythonCommand::process_args("py"),
            "https://docs.python.org/3/"
        );
    }

    #[test]
    fn test_python_command_search() {
        assert_eq!(
            PythonCommand::process_args("python list"),
            "https://docs.python.org/3/search.html?q=list"
        );
        assert_eq!(
            PythonCommand::process_args("py dict methods"),
            "https://docs.python.org/3/search.html?q=dict%20methods"
        );
    }

    #[test]
    fn test_python_command_tutorial() {
        assert_eq!(
            PythonCommand::process_args("python tutorial"),
            "https://docs.python.org/3/tutorial/"
        );
    }

    #[test]
    fn test_python_command_library() {
        assert_eq!(
            PythonCommand::process_args("python library"),
            "https://docs.python.org/3/library/"
        );
        assert_eq!(
            PythonCommand::process_args("python lib"),
            "https://docs.python.org/3/library/"
        );
    }

    #[test]
    fn test_python_command_reference() {
        assert_eq!(
            PythonCommand::process_args("python reference"),
            "https://docs.python.org/3/reference/"
        );
        assert_eq!(
            PythonCommand::process_args("python ref"),
            "https://docs.python.org/3/reference/"
        );
    }
}
