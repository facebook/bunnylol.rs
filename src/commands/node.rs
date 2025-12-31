/// Node.js documentation command handler
/// Supports:
/// - node/nodejs -> https://nodejs.org/api/
/// - node [module] -> https://nodejs.org/api/[module].html
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

pub struct NodeCommand;

impl NodeCommand {
    /// Validates if a string is a valid Node.js module name
    /// Module names can contain alphanumeric characters, underscores, hyphens, and dots
    fn is_valid_module_name(name: &str) -> bool {
        !name.is_empty()
            && name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
            && !name.starts_with('.')
            && !name.starts_with('-')
    }
}

impl BunnylolCommand for NodeCommand {
    const BINDINGS: &'static [&'static str] = &["node", "nodejs"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://nodejs.org/api/".to_string()
        } else if !query.contains(' ') && Self::is_valid_module_name(query) {
            // Single word queries with valid module names are treated as module names
            format!("https://nodejs.org/api/{}.html", query)
        } else {
            // Multi-word queries or invalid module names just go to base docs
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

    #[test]
    fn test_node_command_invalid_module_names() {
        // Path traversal attempts should go to base docs
        assert_eq!(
            NodeCommand::process_args("node ../../../etc/passwd"),
            "https://nodejs.org/api/"
        );
        // Special characters should go to base docs
        assert_eq!(
            NodeCommand::process_args("node <script>"),
            "https://nodejs.org/api/"
        );
        // Starting with dot should go to base docs
        assert_eq!(
            NodeCommand::process_args("node .hidden"),
            "https://nodejs.org/api/"
        );
        // Starting with hyphen should go to base docs
        assert_eq!(
            NodeCommand::process_args("node -flag"),
            "https://nodejs.org/api/"
        );
    }

    #[test]
    fn test_node_command_valid_module_names() {
        // Valid modules with hyphens
        assert_eq!(
            NodeCommand::process_args("node node-fetch"),
            "https://nodejs.org/api/node-fetch.html"
        );
        // Valid modules with underscores
        assert_eq!(
            NodeCommand::process_args("node async_hooks"),
            "https://nodejs.org/api/async_hooks.html"
        );
        // Valid modules with dots (like node:fs.promises)
        assert_eq!(
            NodeCommand::process_args("node fs.promises"),
            "https://nodejs.org/api/fs.promises.html"
        );
    }
}
