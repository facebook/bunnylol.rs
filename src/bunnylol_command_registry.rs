/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::config::get_global_config;

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
        crate::commands::ProtonMailCommand,
        crate::commands::ProtonDriveCommand,
        crate::commands::WaybackCommand,
    }

    /// Process commands that use special prefixes (like $ for stock tickers)
    fn process_prefix_commands(command: &str, full_args: &str) -> Option<String> {
        use crate::commands::*;

        if command.starts_with('$') {
            // Don't process bare $ - let it fall through to default search
            if command.len() <= 1 {
                return None;
            }
            return Some(StockCommand::process_ticker(command));
        }

        if command.starts_with("r/") && command.len() > 2 {
            return Some(RedditCommand::process_subreddit_prefix(full_args));
        }

        None
    }

    /// Process a command string and return the appropriate URL.
    ///
    /// Resolution order (first match wins):
    ///   1. Special prefix handlers (`$TICKER`, `r/sub`)
    ///   2. Built-in registered commands
    ///   3. User-defined `[bindings]` from the loaded config
    ///   4. Default search engine fallback
    ///
    /// Step 3 is intentionally **after** step 2 — built-in commands always
    /// win on a name collision. Conflicts are surfaced as warnings at
    /// startup via [`Self::validate_user_bindings`] so users notice that a
    /// `[bindings]` entry is being shadowed.
    pub fn process_command(command: &str, full_args: &str) -> String {
        // Check for prefix commands first (special case)
        if let Some(url) = Self::process_prefix_commands(command, full_args) {
            return url;
        }

        // Initialize lookup table once, then use O(1) HashMap lookup
        let lookup = COMMAND_LOOKUP.get_or_init(Self::initialize_command_lookup);

        if let Some(handler) = lookup.get(command) {
            return handler(full_args);
        }

        // Try user-defined custom bindings before falling through to search.
        if let Some(cfg) = get_global_config()
            && let Some(url) = cfg.resolve_custom_binding(command, full_args)
        {
            return url;
        }

        let engine = get_global_config()
            .map(|cfg| cfg.default_search)
            .unwrap_or_else(|| "google".to_string());
        crate::commands::search_url(&engine, full_args)
    }

    /// Get all registered command bindings
    pub fn get_all_commands() -> &'static Vec<BunnylolCommandInfo> {
        BINDINGS_DATA.get_or_init(Self::get_all_commands_impl)
    }

    /// All built-in command alias names. Used to detect conflicts with
    /// user-defined `[bindings]` at startup.
    pub fn builtin_binding_names() -> std::collections::HashSet<&'static str> {
        let lookup = COMMAND_LOOKUP.get_or_init(Self::initialize_command_lookup);
        lookup.keys().copied().collect()
    }

    /// Validate user-defined `[bindings]` against the built-in command set
    /// and return any conflicts. Built-ins always win at runtime — conflicts
    /// here exist only for startup logging.
    pub fn validate_user_bindings(
        config: &crate::config::BunnylolConfig,
    ) -> Vec<crate::config::BindingConflict> {
        config.validate_custom_bindings(&Self::builtin_binding_names())
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

        // Verify we have 84+ total bindings (47 commands with multiple aliases each)
        assert!(
            lookup.len() >= 86,
            "Expected at least 86 bindings, got {}",
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
        assert_eq!(commands.len(), 49, "Expected 49 commands");

        // Verify cache returns same pointer (not regenerated)
        let commands2 = BunnylolCommandRegistry::get_all_commands();
        assert!(
            std::ptr::eq(commands, commands2),
            "Cache should return same reference"
        );
    }

    #[test]
    fn test_reddit_subreddit_prefix_via_process_command() {
        assert_eq!(
            BunnylolCommandRegistry::process_command("r/myog", "r/myog"),
            "https://www.reddit.com/r/myog/"
        );
        assert_eq!(
            BunnylolCommandRegistry::process_command("r/rust", "r/rust async await"),
            "https://www.reddit.com/r/rust/search/?q=async%20await"
        );
    }

    // ---------------- Custom [bindings] regression tests ----------------
    //
    // These tests exercise process_command's interaction with the global
    // config's user-defined bindings. They run after other tests may have
    // initialized the global config, so they only assert behavior that is
    // valid regardless of whether GLOBAL_CONFIG is set:
    //   - validate_user_bindings is a pure function over a passed-in config
    //   - apply_binding_template / resolve_custom_binding are pure
    // Tests that need a populated GLOBAL_CONFIG are placed in a separate
    // integration-style test to avoid OnceLock contention between unit tests.

    #[test]
    fn test_builtin_binding_names_contains_known_aliases() {
        let names = BunnylolCommandRegistry::builtin_binding_names();
        assert!(names.contains("gh"));
        assert!(names.contains("ig"));
        assert!(names.contains("yt"));
        assert!(!names.contains("definitely-not-a-binding-xyz"));
    }

    #[test]
    fn test_validate_user_bindings_flags_conflicts_with_builtins() {
        use crate::config::{BunnylolConfig, CustomBinding};

        let mut cfg = BunnylolConfig::default();
        cfg.bindings.insert(
            "gh".to_string(),
            CustomBinding::Simple("https://example.com".to_string()),
        );
        cfg.bindings.insert(
            "cal-not-a-builtin".to_string(),
            CustomBinding::Simple("https://example.com".to_string()),
        );
        let conflicts = BunnylolCommandRegistry::validate_user_bindings(&cfg);
        let names: Vec<&str> = conflicts.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, vec!["gh"]);
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
