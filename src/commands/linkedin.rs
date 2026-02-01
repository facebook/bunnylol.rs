use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::encode_url;

pub struct LinkedInCommand;

impl BunnylolCommand for LinkedInCommand {
    const BINDINGS: &'static [&'static str] = &["li", "linkedin"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);

        if query.is_empty() {
            return "https://www.linkedin.com/".to_string();
        }

        let encoded_query = encode_url(query);
        format!(
            "https://www.linkedin.com/search/results/all/?keywords={}",
            encoded_query
        )
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo::new(
            Self::BINDINGS,
            "Navigate to LinkedIn or search",
            "li software engineer",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linkedin_command_no_args() {
        assert_eq!(
            LinkedInCommand::process_args(""),
            "https://www.linkedin.com/"
        );
    }

    #[test]
    fn test_linkedin_command_li_only() {
        assert_eq!(
            LinkedInCommand::process_args("li"),
            "https://www.linkedin.com/"
        );
    }

    #[test]
    fn test_linkedin_command_search() {
        assert_eq!(
            LinkedInCommand::process_args("test"),
            "https://www.linkedin.com/search/results/all/?keywords=test"
        );
    }

    #[test]
    fn test_linkedin_command_search_with_spaces() {
        assert_eq!(
            LinkedInCommand::process_args("software engineer"),
            "https://www.linkedin.com/search/results/all/?keywords=software%20engineer"
        );
    }

    #[test]
    fn test_linkedin_command_li_search() {
        assert_eq!(
            LinkedInCommand::process_args("li rust developer"),
            "https://www.linkedin.com/search/results/all/?keywords=rust%20developer"
        );
    }
}
