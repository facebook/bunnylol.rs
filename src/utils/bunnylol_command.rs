/// Bunnylol Command trait that all URL builders must implement
pub trait BunnylolCommand {
    /// The command string that triggers this binding (e.g., "gh", "tw", "r")
    const COMMAND: &'static str;

    /// Process the command arguments and return the appropriate URL
    fn process_args(args: &str) -> String;

    /// Get the command portion from the full arguments string
    fn get_command_args(args: &str) -> &str {
        if args.len() <= Self::COMMAND.len() {
            ""
        } else {
            &args[Self::COMMAND.len()..].trim_start()
        }
    }

    /// Check if this binding matches the given command
    fn matches_command(command: &str) -> bool {
        command == Self::COMMAND
    }
}

/// Bunnylol Command Registry that manages all Bunnylol commands
///
/// This struct provides a centralized way to register and lookup commands
/// without requiring changes to the main routing logic when adding new services.
pub struct BunnylolCommandRegistry;

impl BunnylolCommandRegistry {
    /// Process a command string and return the appropriate URL
    pub fn process_command(command: &str, full_args: &str) -> String {
        use crate::commands::*;

        match command {
            cmd if DevBunnyCommand::matches_command(cmd) => {
                DevBunnyCommand::process_args(full_args)
            }
            cmd if GitHubCommand::matches_command(cmd) => GitHubCommand::process_args(full_args),
            cmd if GmailCommand::matches_command(cmd) => GmailCommand::process_args(full_args),
            cmd if InstagramCommand::matches_command(cmd) => {
                InstagramCommand::process_args(full_args)
            }
            cmd if RedditCommand::matches_command(cmd) => RedditCommand::process_args(full_args),
            cmd if ReiCommand::matches_command(cmd) => ReiCommand::process_args(full_args),
            cmd if TwitterCommand::matches_command(cmd) => TwitterCommand::process_args(full_args),
            cmd if YouTubeCommand::matches_command(cmd) => YouTubeCommand::process_args(full_args),
            _ => GoogleCommand::process_args(full_args),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock command for testing
    struct TestCommand;

    impl BunnylolCommand for TestCommand {
        const COMMAND: &'static str = "test";

        fn process_args(args: &str) -> String {
            if args == Self::COMMAND {
                "https://test.com".to_string()
            } else {
                let query = Self::get_command_args(args);
                format!("https://test.com/search?q={}", query)
            }
        }
    }

    #[test]
    fn test_bunnylol_command_get_command_args() {
        assert_eq!(TestCommand::get_command_args("test"), "");
        assert_eq!(TestCommand::get_command_args("test hello"), "hello");
        assert_eq!(
            TestCommand::get_command_args("test hello world"),
            "hello world"
        );
    }

    #[test]
    fn test_bunnylol_command_matches_command() {
        assert!(TestCommand::matches_command("test"));
        assert!(!TestCommand::matches_command("other"));
    }

    #[test]
    fn test_bunnylol_command_process_args() {
        assert_eq!(TestCommand::process_args("test"), "https://test.com");
        assert_eq!(
            TestCommand::process_args("test hello"),
            "https://test.com/search?q=hello"
        );
    }
}
