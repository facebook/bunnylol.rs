/// Facebook command handler
/// Supports: fb, fb [username/page], fb [search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::{build_path_url, build_search_url};

pub struct FacebookCommand;

impl FacebookCommand {
    fn construct_profile_url(profile: &str) -> String {
        build_path_url("https://www.facebook.com", profile)
    }

    fn construct_search_url(query: &str) -> String {
        build_search_url("https://www.facebook.com/search/top", "q", query)
    }
}

impl BunnylolCommand for FacebookCommand {
    const BINDINGS: &'static [&'static str] = &["fb"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://www.facebook.com".to_string()
        } else if !query.contains(' ') {
            // Single word without spaces - treat as profile/page
            Self::construct_profile_url(query)
        } else {
            // Multiple words - search
            Self::construct_search_url(query)
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Facebook pages or search Facebook".to_string(),
            example: "fb Meta".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_facebook_command_base() {
        assert_eq!(
            FacebookCommand::process_args("fb"),
            "https://www.facebook.com"
        );
    }

    #[test]
    fn test_facebook_command_profile() {
        assert_eq!(
            FacebookCommand::process_args("fb Meta"),
            "https://www.facebook.com/Meta"
        );
    }

    #[test]
    fn test_facebook_command_search() {
        assert_eq!(
            FacebookCommand::process_args("fb Meta AI"),
            "https://www.facebook.com/search/top?q=Meta%20AI"
        );
    }
}
