use std::collections::HashMap;
use std::sync::OnceLock;

use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

// Type alias for command handler functions
type CommandHandler = fn(&str) -> String;

// Global command lookup table, initialized once on first access
static COMMAND_LOOKUP: OnceLock<HashMap<&'static str, CommandHandler>> = OnceLock::new();
static BINDINGS_DATA: OnceLock<Vec<BunnylolCommandInfo>> = OnceLock::new();

/// Macro to register all commands in one place
/// This prevents bugs where a command is defined but not registered
macro_rules! register_commands {
    ($($cmd:ty),+ $(,)?) => {
        /// Initialize the command lookup HashMap
        /// Maps all command aliases to their handler functions
        fn initialize_command_lookup() -> HashMap<&'static str, CommandHandler> {
            let mut map = HashMap::new();

            $(
                for alias in <$cmd>::BINDINGS {
                    map.insert(*alias, <$cmd>::process_args as CommandHandler);
                }
            )+

            map
        }

        /// Get all registered command bindings
        fn get_all_commands_impl() -> Vec<BunnylolCommandInfo> {
            vec![
                $(
                    <$cmd>::get_info(),
                )+
            ]
        }
    };
}

/// Bunnylol Command Registry that manages all Bunnylol commands
///
/// This struct provides a centralized way to register and lookup commands
/// without requiring changes to the main routing logic when adding new services.
pub struct BunnylolCommandRegistry;

impl BunnylolCommandRegistry {
    // Register all commands here - ADD NEW COMMANDS TO THIS LIST
    register_commands! {
        crate::commands::BindingsCommand,
        crate::commands::GitHubCommand,
        crate::commands::GitlabCommand,
        crate::commands::TwitterCommand,
        crate::commands::RedditCommand,
        crate::commands::GmailCommand,
        crate::commands::REICommand,
        crate::commands::InstagramCommand,
        crate::commands::LinkedInCommand,
        crate::commands::FacebookCommand,
        crate::commands::ThreadsCommand,
        crate::commands::WhatsAppCommand,
        crate::commands::MetaCommand,
        crate::commands::CargoCommand,
        crate::commands::NpmCommand,
        crate::commands::OnePasswordCommand,
        crate::commands::ClaudeCommand,
        crate::commands::ChatGPTCommand,
        crate::commands::RustCommand,
        crate::commands::HackCommand,
        crate::commands::AmazonCommand,
        crate::commands::YouTubeCommand,
        crate::commands::WikipediaCommand,
        crate::commands::DuckDuckGoCommand,
        crate::commands::SchwabCommand,
        crate::commands::SoundCloudCommand,
        crate::commands::StockCommand,
        crate::commands::GoogleDocsCommand,
        crate::commands::GoogleMapsCommand,
        crate::commands::GoogleSheetsCommand,
        crate::commands::GoogleSlidesCommand,
        crate::commands::GoogleChatCommand,
        crate::commands::GoogleSearchCommand,
        crate::commands::BrewCommand,
        crate::commands::ChocoCommand,
        crate::commands::DockerhubCommand,
        crate::commands::GodocsCommand,
        crate::commands::GopkgCommand,
        crate::commands::MdnCommand,
        crate::commands::NodeCommand,
        crate::commands::NugetCommand,
        crate::commands::PackagistCommand,
        crate::commands::PypiCommand,
        crate::commands::PythonCommand,
        crate::commands::RubygemsCommand,
        crate::commands::StackOverflowCommand,
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
        BINDINGS_DATA.get_or_init(Self::get_all_commands_impl)
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

        // Verify we have 82+ total bindings (46 commands with multiple aliases each)
        assert!(
            lookup.len() >= 82,
            "Expected at least 82 bindings, got {}",
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
        assert_eq!(commands.len(), 46, "Expected 46 commands");

        // Verify cache returns same pointer (not regenerated)
        let commands2 = BunnylolCommandRegistry::get_all_commands();
        assert!(
            std::ptr::eq(commands, commands2),
            "Cache should return same reference"
        );
    }

    #[test]
    fn test_no_binding_collisions() {
        use std::collections::HashMap;

        let commands = BunnylolCommandRegistry::get_all_commands();
        let mut binding_to_command: HashMap<&str, &str> = HashMap::new();
        let mut collisions: Vec<String> = Vec::new();

        // Check each command's bindings for collisions
        for cmd_info in commands {
            for binding in &cmd_info.bindings {
                if let Some(existing_description) = binding_to_command.get(binding.as_str()) {
                    collisions.push(format!(
                        "Binding '{}' is used by both '{}' and '{}'",
                        binding, existing_description, cmd_info.description
                    ));
                } else {
                    binding_to_command.insert(binding, &cmd_info.description);
                }
            }
        }

        assert!(
            collisions.is_empty(),
            "Found binding collisions:\n{}",
            collisions.join("\n")
        );
    }
}
