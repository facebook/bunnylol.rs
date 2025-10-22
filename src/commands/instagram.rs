/// Instagram command handler
/// Supports: ig, ig [user]
use crate::utils::bunnylol_command::BunnylolCommand;
use crate::utils::url_encoding::build_path_url;

pub struct InstagramCommand;

impl BunnylolCommand for InstagramCommand {
    const COMMAND: &'static str = "ig";

    fn process_args(args: &str) -> String {
        if args == Self::COMMAND {
            "https://instagram.com".to_string()
        } else {
            let query = Self::get_command_args(args);
            build_path_url("https://instagram.com", query)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instagram_command_base() {
        assert_eq!(
            InstagramCommand::process_args("ig"),
            "https://instagram.com"
        );
    }

    #[test]
    fn test_instagram_command_user() {
        assert_eq!(
            InstagramCommand::process_args("ig facebook"),
            "https://instagram.com/facebook"
        );
    }
}
