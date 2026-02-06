/// Google Sheets command handler
/// Supports: gsheets -> redirects to Google Sheets
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

pub struct GoogleSheetsCommand;

impl BunnylolCommand for GoogleSheetsCommand {
    const BINDINGS: &'static [&'static str] = &["gsheets"];

    fn process_args(_args: &str) -> String {
        "https://docs.google.com/spreadsheets/u/0/".to_string()
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo::new(
            Self::BINDINGS,
            "Navigate to Google Sheets",
            "gsheets",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_sheets_command() {
        assert_eq!(
            GoogleSheetsCommand::process_args("gsheets"),
            "https://docs.google.com/spreadsheets/u/0/"
        );
    }

    #[test]
    fn test_google_sheets_command_with_args() {
        assert_eq!(
            GoogleSheetsCommand::process_args("gsheets some args"),
            "https://docs.google.com/spreadsheets/u/0/"
        );
    }
}
