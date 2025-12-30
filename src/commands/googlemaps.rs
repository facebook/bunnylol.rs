use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::encode_url;

pub struct GoogleMapsCommand;

impl BunnylolCommand for GoogleMapsCommand {
    const BINDINGS: &'static [&'static str] = &["gmaps", "maps"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);

        if query.is_empty() {
            return "https://www.google.com/maps".to_string();
        }

        let encoded_query = encode_url(query);
        format!("https://www.google.com/maps/search/{}/", encoded_query)
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Google Maps or search for a location".to_string(),
            example: "gmaps san francisco".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_maps_command_no_args() {
        assert_eq!(
            GoogleMapsCommand::process_args(""),
            "https://www.google.com/maps"
        );
    }

    #[test]
    fn test_google_maps_command_gmaps_only() {
        assert_eq!(
            GoogleMapsCommand::process_args("gmaps"),
            "https://www.google.com/maps"
        );
    }

    #[test]
    fn test_google_maps_command_maps_only() {
        assert_eq!(
            GoogleMapsCommand::process_args("maps"),
            "https://www.google.com/maps"
        );
    }

    #[test]
    fn test_google_maps_command_search() {
        assert_eq!(
            GoogleMapsCommand::process_args("starbucks"),
            "https://www.google.com/maps/search/starbucks/"
        );
    }

    #[test]
    fn test_google_maps_command_search_with_spaces() {
        assert_eq!(
            GoogleMapsCommand::process_args("san francisco"),
            "https://www.google.com/maps/search/san%20francisco/"
        );
    }

    #[test]
    fn test_google_maps_command_gmaps_search() {
        assert_eq!(
            GoogleMapsCommand::process_args("gmaps coffee shop"),
            "https://www.google.com/maps/search/coffee%20shop/"
        );
    }
}
