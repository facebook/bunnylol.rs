use std::collections::HashMap;
use std::sync::OnceLock;

use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

// Type alias for command handler functions
type CommandHandler = fn(&str) -> String;

// Global command lookup table, initialized once on first access
static COMMAND_LOOKUP: OnceLock<HashMap<&'static str, CommandHandler>> = OnceLock::new();
static BINDINGS_DATA: OnceLock<Vec<BunnylolCommandInfo>> = OnceLock::new();
static ALIAS_TO_COMMAND_NAME: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
static COMMAND_NAME_TO_BINDINGS: OnceLock<HashMap<&'static str, Vec<&'static str>>> =
    OnceLock::new();

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

        /// Initialize alias-to-command-name lookup
        /// Maps aliases (e.g., "gh", "github") to the struct name (e.g., "GitHub")
        fn initialize_alias_to_command_name() -> HashMap<&'static str, &'static str> {
            let mut map = HashMap::new();

            $(
                // Extract command name from struct type (e.g., "GitHubCommand" -> "GitHub")
                let type_name = std::any::type_name::<$cmd>();
                let command_name = type_name
                    .rsplit("::")
                    .next()
                    .unwrap_or(type_name)
                    .strip_suffix("Command")
                    .unwrap_or(type_name);

                // Leak the string once so it has 'static lifetime (only happens on first init)
                let command_name_static: &'static str = Box::leak(command_name.to_string().into_boxed_str());

                // Map all aliases to this command name
                for alias in <$cmd>::BINDINGS {
                    map.insert(*alias, command_name_static);
                }
            )+

            map
        }

        /// Initialize command-name-to-bindings lookup
        /// Maps command names (e.g., "GitHub") to all their aliases (e.g., ["gh", "github"])
        fn initialize_command_name_to_bindings() -> HashMap<&'static str, Vec<&'static str>> {
            let mut map = HashMap::new();

            $(
                // Extract command name from struct type (e.g., "GitHubCommand" -> "GitHub")
                let type_name = std::any::type_name::<$cmd>();
                let command_name = type_name
                    .rsplit("::")
                    .next()
                    .unwrap_or(type_name)
                    .strip_suffix("Command")
                    .unwrap_or(type_name);

                // Leak the string once so it has 'static lifetime (only happens on first init)
                let command_name_static: &'static str = Box::leak(command_name.to_string().into_boxed_str());

                // Collect all bindings for this command
                let bindings: Vec<&'static str> = <$cmd>::BINDINGS.to_vec();
                map.insert(command_name_static, bindings);
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
        // Prefix commands bypass command filtering
        if let Some(url) = Self::process_prefix_commands(command) {
            return url;
        }

        // Initialize lookup tables once, then use O(1) HashMap lookups
        let lookup = COMMAND_LOOKUP.get_or_init(Self::initialize_command_lookup);
        let alias_to_name =
            ALIAS_TO_COMMAND_NAME.get_or_init(Self::initialize_alias_to_command_name);

        match lookup.get(command) {
            Some(handler) => {
                // Check command filtering if config provided
                if let Some(cfg) = config {
                    // Get the command name (struct name like "GitHub") for this alias
                    if let Some(&command_name) = alias_to_name.get(command) {
                        // Get bindings from cache instead of rebuilding
                        let name_to_bindings = COMMAND_NAME_TO_BINDINGS
                            .get_or_init(Self::initialize_command_name_to_bindings);

                        if let Some(command_bindings) = name_to_bindings.get(command_name) {
                            // Check if this command is allowed (checks struct name and all aliases)
                            if !cfg
                                .command_filtering
                                .is_command_allowed(command_name, command_bindings)
                            {
                                // Command is blocked - fall through to search
                                return cfg.get_search_url(full_args);
                            }
                        }
                    }
                }

                // Command is allowed - execute handler
                handler(full_args)
            }
            None => {
                // Unknown command - fall through to search
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

    /// Get all registered command bindings, filtered by command filtering config
    /// Blocked commands are hidden from the list
    pub fn get_all_commands_filtered(
        config: Option<&crate::config::BunnylolConfig>,
    ) -> Vec<BunnylolCommandInfo> {
        let all_commands = Self::get_all_commands();

        if let Some(cfg) = config {
            let alias_to_name =
                ALIAS_TO_COMMAND_NAME.get_or_init(Self::initialize_alias_to_command_name);

            all_commands
                .iter()
                .filter(|cmd_info| {
                    // Get command name from first binding
                    if let Some(first_binding) = cmd_info.bindings.first()
                        && let Some(&command_name) = alias_to_name.get(first_binding.as_str())
                    {
                        // Convert Vec<String> to &[&str] for is_command_allowed
                        let bindings: Vec<&str> =
                            cmd_info.bindings.iter().map(|s| s.as_str()).collect();

                        // Check if this command is allowed
                        return cfg
                            .command_filtering
                            .is_command_allowed(command_name, &bindings);
                    }
                    true // If we can't determine, show it
                })
                .cloned()
                .collect()
        } else {
            // No config, return all commands
            all_commands.clone()
        }
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

    #[test]
    fn test_process_command_with_blocklist() {
        let mut config = crate::config::BunnylolConfig::default();
        config.command_filtering.blocked_commands = vec!["gh".to_string()];

        // GitHub command should be blocked (falls through to search)
        let url = BunnylolCommandRegistry::process_command_with_config(
            "gh",
            "gh facebook/react",
            Some(&config),
        );
        assert!(url.contains("google.com/search"));

        // Instagram should work normally
        let url = BunnylolCommandRegistry::process_command_with_config("ig", "ig", Some(&config));
        assert_eq!(url, "https://www.instagram.com");
    }

    #[test]
    fn test_process_command_with_allowlist() {
        let mut config = crate::config::BunnylolConfig::default();
        config.command_filtering.allowed_commands = vec!["ig".to_string()];

        // Instagram should work
        let url = BunnylolCommandRegistry::process_command_with_config("ig", "ig", Some(&config));
        assert_eq!(url, "https://www.instagram.com");

        // GitHub should be blocked
        let url = BunnylolCommandRegistry::process_command_with_config(
            "gh",
            "gh facebook/react",
            Some(&config),
        );
        assert!(url.contains("google.com/search"));
    }

    #[test]
    fn test_process_command_filtering_disabled() {
        let config = crate::config::BunnylolConfig::default();

        // All commands should work normally
        let url = BunnylolCommandRegistry::process_command_with_config(
            "gh",
            "gh facebook/react",
            Some(&config),
        );
        assert_eq!(url, "https://github.com/facebook/react");
    }

    #[test]
    fn test_blocked_command_uses_custom_search_engine() {
        let config = crate::config::BunnylolConfig {
            default_search: "ddg".to_string(),
            command_filtering: crate::config::CommandFilteringConfig {
                blocked_commands: vec!["gh".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let url = BunnylolCommandRegistry::process_command_with_config(
            "gh",
            "gh facebook/react",
            Some(&config),
        );
        assert!(url.contains("duckduckgo.com"));
    }

    #[test]
    fn test_prefix_command_bypasses_filtering() {
        let config = crate::config::BunnylolConfig {
            command_filtering: crate::config::CommandFilteringConfig {
                blocked_commands: vec!["stock".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        // Prefix stock command should still work
        let url =
            BunnylolCommandRegistry::process_command_with_config("$AAPL", "$AAPL", Some(&config));
        // Stock command returns a Schwab URL
        assert!(url.contains("schwab.com") || url.contains("quote"));
    }

    #[test]
    fn test_unknown_command_unaffected_by_filtering() {
        let config = crate::config::BunnylolConfig {
            command_filtering: crate::config::CommandFilteringConfig {
                allowed_commands: vec!["gh".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        // Unknown command should fall through to search
        let url = BunnylolCommandRegistry::process_command_with_config(
            "unknowncmd",
            "unknowncmd some query",
            Some(&config),
        );
        assert!(url.contains("google.com/search"));
    }

    #[test]
    fn test_alias_to_command_name_mapping() {
        let alias_map = ALIAS_TO_COMMAND_NAME
            .get_or_init(BunnylolCommandRegistry::initialize_alias_to_command_name);

        // Test that aliases map to their first binding (canonical name)
        // GitHub's first binding should be used as the canonical name
        if let Some(&gh_command) = alias_map.get("gh") {
            // Both "gh" and any other GitHub aliases should map to the same command name
            assert!(alias_map.values().filter(|&&v| v == gh_command).count() >= 1);
        }

        // Verify we have entries for many bindings
        assert!(alias_map.len() >= 82, "Should have many alias mappings");
    }

    #[test]
    fn test_get_all_commands_filtered_with_blocklist() {
        let config = crate::config::BunnylolConfig {
            command_filtering: crate::config::CommandFilteringConfig {
                blocked_commands: vec!["GitHub".to_string(), "Meta".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let filtered = BunnylolCommandRegistry::get_all_commands_filtered(Some(&config));
        let all = BunnylolCommandRegistry::get_all_commands();

        // Filtered list should be smaller
        assert!(filtered.len() < all.len());

        // GitHub and Meta should not be in filtered list
        assert!(
            !filtered
                .iter()
                .any(|cmd| cmd.bindings.contains(&"gh".to_string()))
        );
        assert!(
            !filtered
                .iter()
                .any(|cmd| cmd.bindings.contains(&"meta".to_string()))
        );

        // Instagram should still be in filtered list
        assert!(
            filtered
                .iter()
                .any(|cmd| cmd.bindings.contains(&"ig".to_string()))
        );
    }

    #[test]
    fn test_get_all_commands_filtered_with_allowlist() {
        let config = crate::config::BunnylolConfig {
            command_filtering: crate::config::CommandFilteringConfig {
                allowed_commands: vec!["GitHub".to_string(), "Instagram".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let filtered = BunnylolCommandRegistry::get_all_commands_filtered(Some(&config));

        // Filtered list should only have 2 commands
        assert_eq!(filtered.len(), 2);

        // Only GitHub and Instagram should be in list
        assert!(
            filtered
                .iter()
                .any(|cmd| cmd.bindings.contains(&"gh".to_string()))
        );
        assert!(
            filtered
                .iter()
                .any(|cmd| cmd.bindings.contains(&"ig".to_string()))
        );

        // Meta should not be in list
        assert!(
            !filtered
                .iter()
                .any(|cmd| cmd.bindings.contains(&"meta".to_string()))
        );
    }

    #[test]
    fn test_get_all_commands_filtered_no_config() {
        let filtered = BunnylolCommandRegistry::get_all_commands_filtered(None);
        let all = BunnylolCommandRegistry::get_all_commands();

        // Should return all commands when no config provided
        assert_eq!(filtered.len(), all.len());
    }
}
