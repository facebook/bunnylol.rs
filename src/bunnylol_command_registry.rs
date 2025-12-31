use std::collections::HashMap;
use std::sync::OnceLock;

use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

// Type alias for command handler functions
type CommandHandler = fn(&str) -> String;

// Global command lookup table, initialized once on first access
static COMMAND_LOOKUP: OnceLock<HashMap<&'static str, CommandHandler>> = OnceLock::new();
static BINDINGS_DATA: OnceLock<Vec<BunnylolCommandInfo>> = OnceLock::new();

/// Bunnylol Command Registry that manages all Bunnylol commands
///
/// This struct provides a centralized way to register and lookup commands
/// without requiring changes to the main routing logic when adding new services.
pub struct BunnylolCommandRegistry;

impl BunnylolCommandRegistry {
    /// Initialize the command lookup HashMap
    /// Maps all command aliases to their handler functions
    fn initialize_command_lookup() -> HashMap<&'static str, CommandHandler> {
        use crate::commands::*;

        let mut map = HashMap::new();

        // Register all commands with their aliases
        for alias in BindingsCommand::BINDINGS {
            map.insert(*alias, BindingsCommand::process_args as CommandHandler);
        }
        for alias in GitHubCommand::BINDINGS {
            map.insert(*alias, GitHubCommand::process_args as CommandHandler);
        }
        for alias in TwitterCommand::BINDINGS {
            map.insert(*alias, TwitterCommand::process_args as CommandHandler);
        }
        for alias in RedditCommand::BINDINGS {
            map.insert(*alias, RedditCommand::process_args as CommandHandler);
        }
        for alias in GmailCommand::BINDINGS {
            map.insert(*alias, GmailCommand::process_args as CommandHandler);
        }
        for alias in DevBunnyCommand::BINDINGS {
            map.insert(*alias, DevBunnyCommand::process_args as CommandHandler);
        }
        for alias in REICommand::BINDINGS {
            map.insert(*alias, REICommand::process_args as CommandHandler);
        }
        for alias in InstagramCommand::BINDINGS {
            map.insert(*alias, InstagramCommand::process_args as CommandHandler);
        }
        for alias in LinkedInCommand::BINDINGS {
            map.insert(*alias, LinkedInCommand::process_args as CommandHandler);
        }
        for alias in FacebookCommand::BINDINGS {
            map.insert(*alias, FacebookCommand::process_args as CommandHandler);
        }
        for alias in ThreadsCommand::BINDINGS {
            map.insert(*alias, ThreadsCommand::process_args as CommandHandler);
        }
        for alias in WhatsAppCommand::BINDINGS {
            map.insert(*alias, WhatsAppCommand::process_args as CommandHandler);
        }
        for alias in MetaCommand::BINDINGS {
            map.insert(*alias, MetaCommand::process_args as CommandHandler);
        }
        for alias in CargoCommand::BINDINGS {
            map.insert(*alias, CargoCommand::process_args as CommandHandler);
        }
        for alias in NpmCommand::BINDINGS {
            map.insert(*alias, NpmCommand::process_args as CommandHandler);
        }
        for alias in OnePasswordCommand::BINDINGS {
            map.insert(*alias, OnePasswordCommand::process_args as CommandHandler);
        }
        for alias in ClaudeCommand::BINDINGS {
            map.insert(*alias, ClaudeCommand::process_args as CommandHandler);
        }
        for alias in ChatGPTCommand::BINDINGS {
            map.insert(*alias, ChatGPTCommand::process_args as CommandHandler);
        }
        for alias in RustCommand::BINDINGS {
            map.insert(*alias, RustCommand::process_args as CommandHandler);
        }
        for alias in HackCommand::BINDINGS {
            map.insert(*alias, HackCommand::process_args as CommandHandler);
        }
        for alias in AmazonCommand::BINDINGS {
            map.insert(*alias, AmazonCommand::process_args as CommandHandler);
        }
        for alias in YouTubeCommand::BINDINGS {
            map.insert(*alias, YouTubeCommand::process_args as CommandHandler);
        }
        for alias in WikipediaCommand::BINDINGS {
            map.insert(*alias, WikipediaCommand::process_args as CommandHandler);
        }
        for alias in DuckDuckGoCommand::BINDINGS {
            map.insert(*alias, DuckDuckGoCommand::process_args as CommandHandler);
        }
        for alias in SchwabCommand::BINDINGS {
            map.insert(*alias, SchwabCommand::process_args as CommandHandler);
        }
        for alias in SoundCloudCommand::BINDINGS {
            map.insert(*alias, SoundCloudCommand::process_args as CommandHandler);
        }
        for alias in StockCommand::BINDINGS {
            map.insert(*alias, StockCommand::process_args as CommandHandler);
        }
        for alias in GoogleDocsCommand::BINDINGS {
            map.insert(*alias, GoogleDocsCommand::process_args as CommandHandler);
        }
        for alias in GoogleMapsCommand::BINDINGS {
            map.insert(*alias, GoogleMapsCommand::process_args as CommandHandler);
        }
        for alias in GoogleSheetsCommand::BINDINGS {
            map.insert(*alias, GoogleSheetsCommand::process_args as CommandHandler);
        }
        for alias in GoogleSlidesCommand::BINDINGS {
            map.insert(*alias, GoogleSlidesCommand::process_args as CommandHandler);
        }
        for alias in GoogleChatCommand::BINDINGS {
            map.insert(*alias, GoogleChatCommand::process_args as CommandHandler);
        }

        map
    }

    /// Process commands that use special prefixes (like $ for stock tickers)
    fn process_prefix_commands(command: &str) -> Option<String> {
        use crate::commands::*;

        if command.starts_with('$') {
            // Don't process bare $ - let it fall through to default search
            if command.len() <= 1 {
                return None;
            }
            return Some(StockCommand::process_ticker(command));
        }

        None
    }

    /// Process a command string and return the appropriate URL
    pub fn process_command(command: &str, full_args: &str) -> String {
        Self::process_command_with_config(command, full_args, None)
    }

    /// Process a command string with optional config for custom search engine
    pub fn process_command_with_config(
        command: &str,
        full_args: &str,
        config: Option<&crate::config::BunnylolConfig>,
    ) -> String {
        use crate::commands::*;

        // Check for prefix commands first (special case)
        if let Some(url) = Self::process_prefix_commands(command) {
            return url;
        }

        // Initialize lookup table once, then use O(1) HashMap lookup
        let lookup = COMMAND_LOOKUP.get_or_init(Self::initialize_command_lookup);

        match lookup.get(command) {
            Some(handler) => handler(full_args),
            None => {
                // Use configured search engine if provided, otherwise default to Google
                if let Some(cfg) = config {
                    cfg.get_search_url(full_args)
                } else {
                    GoogleSearchCommand::process_args(full_args)
                }
            }
        }
    }

    /// Get all registered command bindings
    pub fn get_all_commands() -> &'static Vec<BunnylolCommandInfo> {
        BINDINGS_DATA.get_or_init(|| {
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
                LinkedInCommand::get_info(),
                FacebookCommand::get_info(),
                ThreadsCommand::get_info(),
                WhatsAppCommand::get_info(),
                MetaCommand::get_info(),
                CargoCommand::get_info(),
                NpmCommand::get_info(),
                OnePasswordCommand::get_info(),
                ClaudeCommand::get_info(),
                ChatGPTCommand::get_info(),
                RustCommand::get_info(),
                HackCommand::get_info(),
                AmazonCommand::get_info(),
                YouTubeCommand::get_info(),
                WikipediaCommand::get_info(),
                DuckDuckGoCommand::get_info(),
                SchwabCommand::get_info(),
                SoundCloudCommand::get_info(),
                StockCommand::get_info(),
                GoogleDocsCommand::get_info(),
                GoogleMapsCommand::get_info(),
                GoogleSheetsCommand::get_info(),
                GoogleSlidesCommand::get_info(),
                GoogleChatCommand::get_info(),
                GoogleSearchCommand::get_info(),
            ]
        })
    }
}

#[cfg(test)]
mod cache_tests {
    use super::*;

    #[test]
    fn test_command_lookup_contains_all_bindings() {
        let lookup = COMMAND_LOOKUP.get_or_init(BunnylolCommandRegistry::initialize_command_lookup);

        // Verify key bindings are present (using actual command bindings)
        assert!(lookup.contains_key("gh"));
        assert!(lookup.contains_key("ig"));
        assert!(lookup.contains_key("instagram"));
        assert!(lookup.contains_key("tw"));
        assert!(lookup.contains_key("r"));
        assert!(lookup.contains_key("reddit"));

        // Verify we have 55+ total bindings (31 commands with multiple aliases each)
        assert!(
            lookup.len() >= 55,
            "Expected at least 55 bindings, got {}",
            lookup.len()
        );
    }

    #[test]
    fn test_command_lookup_correctness() {
        use crate::commands::*;

        let lookup = COMMAND_LOOKUP.get_or_init(BunnylolCommandRegistry::initialize_command_lookup);

        // Test GitHub command handler
        let gh_handler = lookup.get("gh").expect("GitHub command should exist");
        assert_eq!(gh_handler("gh"), GitHubCommand::process_args("gh"));

        // Test Instagram command handler
        let ig_handler = lookup.get("ig").expect("Instagram command should exist");
        assert_eq!(ig_handler("ig"), InstagramCommand::process_args("ig"));
    }

    #[test]
    fn test_bindings_data_cache() {
        let commands = BunnylolCommandRegistry::get_all_commands();

        // Verify we have all expected commands
        assert_eq!(commands.len(), 33, "Expected 33 commands");

        // Verify cache returns same pointer (not regenerated)
        let commands2 = BunnylolCommandRegistry::get_all_commands();
        assert!(
            std::ptr::eq(commands, commands2),
            "Cache should return same reference"
        );
    }
}
