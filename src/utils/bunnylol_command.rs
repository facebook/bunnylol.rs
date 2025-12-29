use serde::Serialize;

/// Information about a registered command binding
#[derive(Serialize)]
pub struct CommandInfo {
    pub bindings: Vec<String>,
    pub description: String,
    pub example: String,
}

/// Bunnylol Command trait that all URL builders must implement
pub trait BunnylolCommand {
    /// All command strings that trigger this binding (e.g., ["gh", "github"])
    const BINDINGS: &'static [&'static str];

    /// Process the command arguments and return the appropriate URL
    fn process_args(args: &str) -> String;

    /// Get the command portion from the full arguments string
    fn get_command_args(args: &str) -> &str {
        // Check if args starts with any of the bindings
        for binding in Self::BINDINGS {
            if args.split_whitespace().next() == Some(*binding) {
                if args.len() <= binding.len() {
                    return "";
                } else {
                    return args[binding.len()..].trim_start();
                }
            }
        }
        args
    }

    /// Check if this binding matches the given command
    fn matches_command(command: &str) -> bool {
        Self::BINDINGS.contains(&command)
    }

    /// Get information about this command (description and examples)
    fn get_info() -> CommandInfo;
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
            cmd if BindingsCommand::matches_command(cmd) => { BindingsCommand::process_args(full_args) }
            cmd if GitHubCommand::matches_command(cmd) => GitHubCommand::process_args(full_args),
            cmd if TwitterCommand::matches_command(cmd) => TwitterCommand::process_args(full_args),
            cmd if RedditCommand::matches_command(cmd) => RedditCommand::process_args(full_args),
            cmd if GmailCommand::matches_command(cmd) => GmailCommand::process_args(full_args),
            cmd if DevBunnyCommand::matches_command(cmd) => {
                DevBunnyCommand::process_args(full_args)
            }
            cmd if REICommand::matches_command(cmd) => REICommand::process_args(full_args),
            cmd if InstagramCommand::matches_command(cmd) => InstagramCommand::process_args(full_args),
            cmd if FacebookCommand::matches_command(cmd) => FacebookCommand::process_args(full_args),
            cmd if ThreadsCommand::matches_command(cmd) => ThreadsCommand::process_args(full_args),
            cmd if WhatsAppCommand::matches_command(cmd) => WhatsAppCommand::process_args(full_args),
            cmd if MetaCommand::matches_command(cmd) => MetaCommand::process_args(full_args),
            cmd if CargoCommand::matches_command(cmd) => CargoCommand::process_args(full_args),
            cmd if NpmCommand::matches_command(cmd) => NpmCommand::process_args(full_args),
            cmd if ClaudeCommand::matches_command(cmd) => ClaudeCommand::process_args(full_args),
            cmd if ChatGPTCommand::matches_command(cmd) => ChatGPTCommand::process_args(full_args),
            cmd if RustCommand::matches_command(cmd) => RustCommand::process_args(full_args),
            cmd if HackCommand::matches_command(cmd) => HackCommand::process_args(full_args),
            cmd if AmazonCommand::matches_command(cmd) => AmazonCommand::process_args(full_args),
            cmd if YouTubeCommand::matches_command(cmd) => YouTubeCommand::process_args(full_args),
            cmd if GoogleDocsCommand::matches_command(cmd) => GoogleDocsCommand::process_args(full_args),
            cmd if GoogleSheetsCommand::matches_command(cmd) => GoogleSheetsCommand::process_args(full_args),
            cmd if GoogleSlidesCommand::matches_command(cmd) => GoogleSlidesCommand::process_args(full_args),
            cmd if GoogleChatCommand::matches_command(cmd) => GoogleChatCommand::process_args(full_args),
            _ => GoogleSearchCommand::process_args(full_args),
        }
    }

    /// Get all registered command bindings
    pub fn get_all_commands() -> Vec<CommandInfo> {
        use crate::commands::*;

        vec![
            BindingsCommand::get_info(),
            GitHubCommand::get_info(),
            TwitterCommand::get_info(),
            RedditCommand::get_info(),
            GmailCommand::get_info(),
            DevBunnyCommand::get_info(),
            REICommand::get_info(),
            InstagramCommand::get_info(),
            FacebookCommand::get_info(),
            ThreadsCommand::get_info(),
            WhatsAppCommand::get_info(),
            MetaCommand::get_info(),
            CargoCommand::get_info(),
            NpmCommand::get_info(),
            ClaudeCommand::get_info(),
            ChatGPTCommand::get_info(),
            RustCommand::get_info(),
            HackCommand::get_info(),
            AmazonCommand::get_info(),
            YouTubeCommand::get_info(),
            GoogleDocsCommand::get_info(),
            GoogleSheetsCommand::get_info(),
            GoogleSlidesCommand::get_info(),
            GoogleChatCommand::get_info(),
            GoogleSearchCommand::get_info(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock command for testing
    struct TestCommand;

    impl BunnylolCommand for TestCommand {
        const BINDINGS: &'static [&'static str] = &["test", "t"];

        fn process_args(args: &str) -> String {
            let query = Self::get_command_args(args);
            if query.is_empty() {
                "https://test.com".to_string()
            } else {
                format!("https://test.com/search?q={}", query)
            }
        }

        fn get_info() -> CommandInfo {
            CommandInfo {
                bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
                description: "Test command".to_string(),
                example: "test query".to_string(),
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
        assert!(TestCommand::matches_command("t"));
        assert!(!TestCommand::matches_command("other"));
    }

    #[test]
    fn test_bunnylol_command_process_args() {
        assert_eq!(TestCommand::process_args("test"), "https://test.com");
        assert_eq!(TestCommand::process_args("t"), "https://test.com");
        assert_eq!(
            TestCommand::process_args("test hello"),
            "https://test.com/search?q=hello"
        );
        assert_eq!(
            TestCommand::process_args("t hello"),
            "https://test.com/search?q=hello"
        );
    }
}
