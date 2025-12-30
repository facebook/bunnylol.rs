use std::collections::HashMap;
use std::sync::OnceLock;

use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

static COMMAND_LOOKUP: OnceLock<HashMap<&'static str, fn(&str) -> String>> = OnceLock::new();
static BINDINGS_DATA: OnceLock<Vec<BunnylolCommandInfo>> = OnceLock::new();

/// Bunnylol Command Registry that manages all Bunnylol commands
///
/// This struct provides a centralized way to register and lookup commands
/// without requiring changes to the main routing logic when adding new services.
pub struct BunnylolCommandRegistry;

impl BunnylolCommandRegistry {
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

    /// Initialize the command lookup HashMap
    fn initialize_command_lookup() -> HashMap<&'static str, fn(&str) -> String> {
        use crate::commands::*;

        let mut map = HashMap::new();

        // Register all command bindings
        for binding in BindingsCommand::BINDINGS {
            map.insert(*binding, BindingsCommand::process_args as fn(&str) -> String);
        }
        for binding in GitHubCommand::BINDINGS {
            map.insert(*binding, GitHubCommand::process_args as fn(&str) -> String);
        }
        for binding in TwitterCommand::BINDINGS {
            map.insert(*binding, TwitterCommand::process_args as fn(&str) -> String);
        }
        for binding in RedditCommand::BINDINGS {
            map.insert(*binding, RedditCommand::process_args as fn(&str) -> String);
        }
        for binding in GmailCommand::BINDINGS {
            map.insert(*binding, GmailCommand::process_args as fn(&str) -> String);
        }
        for binding in DevBunnyCommand::BINDINGS {
            map.insert(*binding, DevBunnyCommand::process_args as fn(&str) -> String);
        }
        for binding in REICommand::BINDINGS {
            map.insert(*binding, REICommand::process_args as fn(&str) -> String);
        }
        for binding in InstagramCommand::BINDINGS {
            map.insert(*binding, InstagramCommand::process_args as fn(&str) -> String);
        }
        for binding in LinkedInCommand::BINDINGS {
            map.insert(*binding, LinkedInCommand::process_args as fn(&str) -> String);
        }
        for binding in FacebookCommand::BINDINGS {
            map.insert(*binding, FacebookCommand::process_args as fn(&str) -> String);
        }
        for binding in ThreadsCommand::BINDINGS {
            map.insert(*binding, ThreadsCommand::process_args as fn(&str) -> String);
        }
        for binding in WhatsAppCommand::BINDINGS {
            map.insert(*binding, WhatsAppCommand::process_args as fn(&str) -> String);
        }
        for binding in MetaCommand::BINDINGS {
            map.insert(*binding, MetaCommand::process_args as fn(&str) -> String);
        }
        for binding in CargoCommand::BINDINGS {
            map.insert(*binding, CargoCommand::process_args as fn(&str) -> String);
        }
        for binding in NpmCommand::BINDINGS {
            map.insert(*binding, NpmCommand::process_args as fn(&str) -> String);
        }
        for binding in OnePasswordCommand::BINDINGS {
            map.insert(*binding, OnePasswordCommand::process_args as fn(&str) -> String);
        }
        for binding in ClaudeCommand::BINDINGS {
            map.insert(*binding, ClaudeCommand::process_args as fn(&str) -> String);
        }
        for binding in ChatGPTCommand::BINDINGS {
            map.insert(*binding, ChatGPTCommand::process_args as fn(&str) -> String);
        }
        for binding in RustCommand::BINDINGS {
            map.insert(*binding, RustCommand::process_args as fn(&str) -> String);
        }
        for binding in HackCommand::BINDINGS {
            map.insert(*binding, HackCommand::process_args as fn(&str) -> String);
        }
        for binding in AmazonCommand::BINDINGS {
            map.insert(*binding, AmazonCommand::process_args as fn(&str) -> String);
        }
        for binding in YouTubeCommand::BINDINGS {
            map.insert(*binding, YouTubeCommand::process_args as fn(&str) -> String);
        }
        for binding in WikipediaCommand::BINDINGS {
            map.insert(*binding, WikipediaCommand::process_args as fn(&str) -> String);
        }
        for binding in DuckDuckGoCommand::BINDINGS {
            map.insert(*binding, DuckDuckGoCommand::process_args as fn(&str) -> String);
        }
        for binding in SchwabCommand::BINDINGS {
            map.insert(*binding, SchwabCommand::process_args as fn(&str) -> String);
        }
        for binding in SoundCloudCommand::BINDINGS {
            map.insert(*binding, SoundCloudCommand::process_args as fn(&str) -> String);
        }
        for binding in StockCommand::BINDINGS {
            map.insert(*binding, StockCommand::process_args as fn(&str) -> String);
        }
        for binding in GoogleDocsCommand::BINDINGS {
            map.insert(*binding, GoogleDocsCommand::process_args as fn(&str) -> String);
        }
        for binding in GoogleMapsCommand::BINDINGS {
            map.insert(*binding, GoogleMapsCommand::process_args as fn(&str) -> String);
        }
        for binding in GoogleSheetsCommand::BINDINGS {
            map.insert(*binding, GoogleSheetsCommand::process_args as fn(&str) -> String);
        }
        for binding in GoogleSlidesCommand::BINDINGS {
            map.insert(*binding, GoogleSlidesCommand::process_args as fn(&str) -> String);
        }
        for binding in GoogleChatCommand::BINDINGS {
            map.insert(*binding, GoogleChatCommand::process_args as fn(&str) -> String);
        }

        map
    }

    /// Process a command string and return the appropriate URL
    pub fn process_command(command: &str, full_args: &str) -> String {
        use crate::commands::*;

        // Check for prefix commands first (special case)
        if let Some(url) = Self::process_prefix_commands(command) {
            return url;
        }

        // Initialize lookup table once, then use O(1) HashMap lookup
        let lookup = COMMAND_LOOKUP.get_or_init(Self::initialize_command_lookup);

        match lookup.get(command) {
            Some(handler) => handler(full_args),
            None => GoogleSearchCommand::process_args(full_args),
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
        assert!(lookup.len() >= 55, "Expected at least 55 bindings, got {}", lookup.len());
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
        assert!(std::ptr::eq(commands, commands2), "Cache should return same reference");
    }
}
