/// Google Docs command handler
/// Supports: docs, gdoc -> redirects to Google Docs
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

pub struct GoogleDocsCommand;

impl BunnylolCommand for GoogleDocsCommand {
    const BINDINGS: &'static [&'static str] = &["docs", "gdoc"];

    fn process_args(_args: &str) -> String {
        "https://docs.google.com/document/u/0/".to_string()
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Google Docs".to_string(),
            example: "docs".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_docs_command() {
        assert_eq!(
            GoogleDocsCommand::process_args("docs"),
            "https://docs.google.com/document/u/0/"
        );
        assert_eq!(
            GoogleDocsCommand::process_args("gdoc"),
            "https://docs.google.com/document/u/0/"
        );
    }

    #[test]
    fn test_google_docs_command_with_args() {
        assert_eq!(
            GoogleDocsCommand::process_args("docs some args"),
            "https://docs.google.com/document/u/0/"
        );
    }
}
