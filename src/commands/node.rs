/// Node.js documentation command handler
/// Supports:
/// - node/nodejs -> https://nodejs.org/api/
/// - node [module] -> https://nodejs.org/api/[module].html
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

pub struct NodeCommand;

impl BunnylolCommand for NodeCommand {
    const BINDINGS: &'static [&'static str] = &["node", "nodejs"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://nodejs.org/api/".to_string()
        } else if !query.contains(' ') {
            // Single word queries are treated as module names
            format!("https://nodejs.org/api/{}.html", query)
        } else {
            // Multi-word queries just go to base docs (no search available)
            "https://nodejs.org/api/".to_string()
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Node.js API documentation or specific module docs"
                .to_string(),
            example: "node fs".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_command_base() {
        assert_eq!(NodeCommand::process_args("node"), "https://nodejs.org/api/");
        assert_eq!(
            NodeCommand::process_args("nodejs"),
            "https://nodejs.org/api/"
        );
    }

    #[test]
    fn test_node_command_module() {
        assert_eq!(
            NodeCommand::process_args("node fs"),
            "https://nodejs.org/api/fs.html"
        );
        assert_eq!(
            NodeCommand::process_args("nodejs http"),
            "https://nodejs.org/api/http.html"
        );
        assert_eq!(
            NodeCommand::process_args("node stream"),
            "https://nodejs.org/api/stream.html"
        );
    }

    #[test]
    fn test_node_command_multiword() {
        assert_eq!(
            NodeCommand::process_args("node file system"),
            "https://nodejs.org/api/"
        );
    }
}
