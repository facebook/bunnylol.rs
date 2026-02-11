/// Facebook command handler
/// Supports: fb, fb [username/page], fb [search terms]
/// Subcommands: mp/buy/sell -> Marketplace
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::{build_path_url, build_search_url};

pub struct FacebookCommand;

impl FacebookCommand {
    const MARKETPLACE_URL: &'static str = "https://www.facebook.com/marketplace";

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
        match query {
            "" => "https://www.facebook.com".to_string(),
            "mp" | "buy" | "sell" => Self::MARKETPLACE_URL.to_string(),
            _ if !query.contains(' ') => Self::construct_profile_url(query),
            _ => Self::construct_search_url(query),
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo::new(
            Self::BINDINGS,
            "Navigate to Facebook pages or search Facebook",
            "fb Meta",
        )
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

    #[test]
    fn test_facebook_command_marketplace_mp() {
        assert_eq!(
            FacebookCommand::process_args("fb mp"),
            FacebookCommand::MARKETPLACE_URL
        );
    }

    #[test]
    fn test_facebook_command_marketplace_buy() {
        assert_eq!(
            FacebookCommand::process_args("fb buy"),
            FacebookCommand::MARKETPLACE_URL
        );
    }

    #[test]
    fn test_facebook_command_marketplace_sell() {
        assert_eq!(
            FacebookCommand::process_args("fb sell"),
            FacebookCommand::MARKETPLACE_URL
        );
    }
}
