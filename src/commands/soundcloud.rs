use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::encode_url;

pub struct SoundCloudCommand;

impl BunnylolCommand for SoundCloudCommand {
    const BINDINGS: &'static [&'static str] = &["sc", "soundcloud"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);

        match query {
            "likes" => "https://soundcloud.com/you/likes".to_string(),
            "" => "https://soundcloud.com/discover".to_string(),
            _ => {
                let encoded_query = encode_url(query);
                format!("https://soundcloud.com/search?q={}", encoded_query)
            }
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to SoundCloud (supports: likes)".to_string(),
            example: "sc edm".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soundcloud_command_no_args() {
        assert_eq!(
            SoundCloudCommand::process_args(""),
            "https://soundcloud.com/discover"
        );
    }

    #[test]
    fn test_soundcloud_command_sc_only() {
        assert_eq!(
            SoundCloudCommand::process_args("sc"),
            "https://soundcloud.com/discover"
        );
    }

    #[test]
    fn test_soundcloud_command_likes() {
        assert_eq!(
            SoundCloudCommand::process_args("likes"),
            "https://soundcloud.com/you/likes"
        );
    }

    #[test]
    fn test_soundcloud_command_sc_likes() {
        assert_eq!(
            SoundCloudCommand::process_args("sc likes"),
            "https://soundcloud.com/you/likes"
        );
    }

    #[test]
    fn test_soundcloud_command_search() {
        assert_eq!(
            SoundCloudCommand::process_args("edm"),
            "https://soundcloud.com/search?q=edm"
        );
    }

    #[test]
    fn test_soundcloud_command_search_with_spaces() {
        assert_eq!(
            SoundCloudCommand::process_args("electronic music"),
            "https://soundcloud.com/search?q=electronic%20music"
        );
    }

    #[test]
    fn test_soundcloud_command_sc_search() {
        assert_eq!(
            SoundCloudCommand::process_args("sc house music"),
            "https://soundcloud.com/search?q=house%20music"
        );
    }
}
