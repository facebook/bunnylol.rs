/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{OnceLock, RwLock};
use std::time::SystemTime;

/// Global config snapshot used by command handlers that read config directly.
static GLOBAL_CONFIG: OnceLock<RwLock<BunnylolConfig>> = OnceLock::new();

/// Initialize or replace the global config snapshot.
pub fn init_global_config(config: BunnylolConfig) {
    if let Some(global) = GLOBAL_CONFIG.get() {
        *global.write().expect("global config lock poisoned") = config;
    } else {
        let _ = GLOBAL_CONFIG.set(RwLock::new(config));
    }
}

/// Get a reference to the global config, after initialized.
pub fn get_global_config() -> Option<BunnylolConfig> {
    GLOBAL_CONFIG
        .get()
        .map(|config| config.read().expect("global config lock poisoned").clone())
}

/// Configuration for bunnylol CLI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BunnylolConfig {
    /// Browser to open URLs in (optional)
    /// Examples: "firefox", "chrome", "chromium", "safari"
    /// If not set, uses the OS default browser
    #[serde(default)]
    pub browser: Option<String>,

    /// Default search engine when command not recognized (optional)
    /// Options: "google" (default), "ddg", "bing"
    #[serde(default = "default_search_engine")]
    pub default_search: String,

    /// Stock website provider
    /// Options: "yahoo" (default), "finviz", "tradingview", "google", "investing"
    #[serde(default = "default_stock_provider")]
    pub stock_provider: String,

    /// Custom command aliases
    #[serde(default)]
    pub aliases: HashMap<String, String>,

    /// Custom URL bindings (user-defined)
    ///
    /// Maps a name to a URL or URL template. Unlike `aliases` (which renames
    /// other commands), a binding maps directly to a URL. Templates may use
    /// `{}` as a placeholder; arguments are URL-encoded before substitution.
    ///
    /// Custom bindings never override built-in commands — on conflict, the
    /// built-in wins and a warning is logged at startup.
    ///
    /// NOTE: Hot-reload is not supported in this release. Edit `config.toml`
    /// and restart `bunnylol serve` (or the CLI) for changes to apply.
    #[serde(default)]
    pub bindings: HashMap<String, CustomBinding>,

    /// Command history settings
    #[serde(default)]
    pub history: HistoryConfig,

    /// Server configuration (for bunnylol serve)
    #[serde(default)]
    pub server: ServerConfig,
}

impl Default for BunnylolConfig {
    fn default() -> Self {
        Self {
            browser: None,
            default_search: default_search_engine(),
            stock_provider: default_stock_provider(),
            aliases: HashMap::new(),
            bindings: HashMap::new(),
            history: HistoryConfig::default(),
            server: ServerConfig::default(),
        }
    }
}

/// Reloads `config.toml` when its modified time changes.
#[derive(Debug)]
pub struct ConfigReloader {
    config: RwLock<BunnylolConfig>,
    config_path: Option<PathBuf>,
    modified: RwLock<Option<SystemTime>>,
}

impl ConfigReloader {
    pub fn new(config: BunnylolConfig) -> Self {
        let config_path = BunnylolConfig::get_config_path();
        Self::with_path(config, config_path)
    }

    fn with_path(config: BunnylolConfig, config_path: Option<PathBuf>) -> Self {
        let modified = config_path
            .as_ref()
            .and_then(|path| fs::metadata(path).ok())
            .and_then(|metadata| metadata.modified().ok());

        Self {
            config: RwLock::new(config),
            config_path,
            modified: RwLock::new(modified),
        }
    }

    #[cfg(test)]
    fn new_for_path(config: BunnylolConfig, config_path: PathBuf) -> Self {
        Self::with_path(config, Some(config_path))
    }

    pub fn current(&self) -> BunnylolConfig {
        if let Err(e) = self.reload_if_changed() {
            eprintln!("Warning: Failed to reload config.toml: {}", e);
        }

        self.config
            .read()
            .expect("config reloader lock poisoned")
            .clone()
    }

    fn reload_if_changed(&self) -> Result<(), String> {
        let Some(path) = &self.config_path else {
            return Ok(());
        };

        let modified = fs::metadata(path)
            .map_err(|e| format!("Failed to stat config file {:?}: {}", path, e))?
            .modified()
            .map_err(|e| format!("Failed to read config modified time {:?}: {}", path, e))?;

        {
            let last_modified = self.modified.read().expect("config reloader lock poisoned");
            if *last_modified == Some(modified) {
                return Ok(());
            }
        }

        let config = BunnylolConfig::load_from_path(path)?;
        *self.config.write().expect("config reloader lock poisoned") = config.clone();
        *self
            .modified
            .write()
            .expect("config reloader lock poisoned") = Some(modified);
        init_global_config(config);

        println!("Reloaded config from {}", path.display());
        Ok(())
    }
}

/// A user-defined URL binding from `[bindings]` in the config file.
///
/// Two TOML forms are accepted:
///
/// ```toml
/// [bindings]
/// # Short form: name = "url"
/// cal = "https://calendar.google.com/calendar/u/1/r"
///
/// # Detailed form: adds a description shown on the /bindings web page.
/// jira = { url = "https://corp.atlassian.net/browse/{}", description = "Jira ticket" }
/// ```
///
/// Templates may include `{}` as a placeholder. At resolution time the
/// command prefix is stripped from the user input, the remainder is
/// URL-encoded, and substituted in. A template with no `{}` is treated
/// as a static URL and any arguments are ignored.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum CustomBinding {
    /// Short form: just a URL or URL template.
    Simple(String),
    /// Detailed form with optional description for the bindings page.
    Detailed {
        url: String,
        #[serde(default)]
        description: Option<String>,
    },
}

impl CustomBinding {
    /// The URL or URL template for this binding.
    pub fn url_template(&self) -> &str {
        match self {
            CustomBinding::Simple(url) => url,
            CustomBinding::Detailed { url, .. } => url,
        }
    }

    /// The description shown on the /bindings web page, if any.
    pub fn description(&self) -> Option<&str> {
        match self {
            CustomBinding::Simple(_) => None,
            CustomBinding::Detailed { description, .. } => description.as_deref(),
        }
    }
}

/// Apply a `{}` template substitution. `command` is stripped from the front
/// of `full_args` (mirroring the convention used by built-in command handlers),
/// and the remainder is URL-encoded before being substituted.
///
/// A template with no `{}` is returned as-is.
pub(crate) fn apply_binding_template(template: &str, command: &str, full_args: &str) -> String {
    if !template.contains("{}") {
        return template.to_string();
    }
    let remainder = full_args
        .strip_prefix(command)
        .map(|s| s.trim_start())
        .unwrap_or(full_args);
    let encoded = crate::utils::url_encoding::encode_url(remainder);
    template.replace("{}", &encoded)
}

/// Result of validating a custom binding against the built-in command set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingConflict {
    /// The name (TOML key) of the user binding that conflicts.
    pub name: String,
    /// The URL template the user defined (kept for diagnostics).
    pub user_url: String,
}


/// Configuration for command history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryConfig {
    /// Whether history tracking is enabled
    #[serde(default = "default_history_enabled")]
    pub enabled: bool,

    /// Maximum number of history entries to keep
    #[serde(default = "default_max_entries")]
    pub max_entries: usize,
}

impl Default for HistoryConfig {
    fn default() -> Self {
        Self {
            enabled: default_history_enabled(),
            max_entries: default_max_entries(),
        }
    }
}

/// Configuration for bunnylol server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Port to bind the server to
    #[serde(default = "default_port")]
    pub port: u16,

    /// Address to bind to (127.0.0.1 for localhost, 0.0.0.0 for network)
    #[serde(default = "default_address")]
    pub address: String,

    /// Rocket log level (normal, debug, critical, off)
    #[serde(default = "default_log_level")]
    pub log_level: String,

    /// Public-facing URL for display in the bindings page
    ///
    /// Smart defaults when protocol is omitted:
    /// - "bunny.example.com" → "https://bunny.example.com"
    /// - "localhost" or "127.0.0.1" or "0.0.0.0" → "http://localhost" (or IP)
    ///
    /// You can also specify the full URL to override:
    /// - "https://bunny.example.com" → used as-is
    /// - "http://bunny.local" → used as-is
    ///
    /// If not set, defaults to http://localhost:{port}
    #[serde(default)]
    pub server_display_url: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: default_port(),
            address: default_address(),
            log_level: default_log_level(),
            server_display_url: None,
        }
    }
}

impl ServerConfig {
    /// Get the display URL for the server, normalized with protocol
    ///
    /// Smart defaults when protocol is omitted:
    /// - Public domains (e.g., "bunny.example.com") → "https://bunny.example.com"
    /// - Local addresses (localhost, 127.0.0.1, 0.0.0.0) → "http://localhost" (or IP)
    ///
    /// If server_display_url is not set, returns "http://localhost:{port}"
    pub fn get_display_url(&self) -> String {
        match &self.server_display_url {
            Some(url) => {
                let url = url.trim();
                // If URL already has a protocol, use as-is
                if url.starts_with("http://") || url.starts_with("https://") {
                    url.to_string()
                } else {
                    // Bare domain/IP - apply smart defaults
                    if url.starts_with("localhost")
                        || url.starts_with("127.0.0.1")
                        || url.starts_with("0.0.0.0")
                    {
                        // Local addresses default to http://
                        format!("http://{}", url)
                    } else {
                        // Public domains default to https://
                        format!("https://{}", url)
                    }
                }
            }
            None => {
                // Fallback to localhost
                format!("http://localhost:{}", self.port)
            }
        }
    }
}

fn default_search_engine() -> String {
    "google".to_string()
}

fn default_stock_provider() -> String {
    "yahoo".to_string()
}

fn default_history_enabled() -> bool {
    true
}

fn default_max_entries() -> usize {
    1000
}

fn default_port() -> u16 {
    8000
}

fn default_address() -> String {
    "127.0.0.1".to_string()
}

fn default_log_level() -> String {
    "normal".to_string()
}

impl BunnylolConfig {
    /// Get the XDG base directories for bunnylol
    fn get_xdg_dirs() -> Option<xdg::BaseDirectories> {
        Some(xdg::BaseDirectories::with_prefix("bunnylol"))
    }

    /// Get the XDG config directory path for bunnylol
    /// Returns: $XDG_CONFIG_HOME/bunnylol (defaults to ~/.config/bunnylol)
    pub fn get_config_dir() -> Option<PathBuf> {
        Self::get_xdg_dirs().and_then(|xdg| xdg.get_config_home())
    }

    /// Get the XDG data directory path for bunnylol
    /// Returns: $XDG_DATA_HOME/bunnylol (defaults to ~/.local/share/bunnylol)
    pub fn get_data_dir() -> Option<PathBuf> {
        Self::get_xdg_dirs().and_then(|xdg| xdg.get_data_home())
    }

    /// Get the XDG cache directory path for bunnylol
    /// Returns: $XDG_CACHE_HOME/bunnylol (defaults to ~/.cache/bunnylol)
    pub fn get_cache_dir() -> Option<PathBuf> {
        Self::get_xdg_dirs().and_then(|xdg| xdg.get_cache_home())
    }

    /// Get the full path to an existing config file.
    /// Returns: /etc/bunnylol/config.toml (system-wide, preferred on Unix)
    ///       or $XDG_CONFIG_HOME/bunnylol/config.toml (user-specific fallback)
    pub fn get_config_path() -> Option<PathBuf> {
        let user_config = Self::get_config_dir().map(|dir| dir.join("config.toml"));

        // Check system-wide config first on Unix platforms.
        #[cfg(unix)]
        {
            let system_config = PathBuf::from("/etc/bunnylol/config.toml");
            if system_config.exists() {
                // Warn if both configs exist
                if let Some(ref user_path) = user_config
                    && user_path.exists()
                {
                    eprintln!("Warning: Found config files at both locations:");
                    eprintln!("  - {}", system_config.display());
                    eprintln!("  - {}", user_path.display());
                    eprintln!("Using system config: {}", system_config.display());
                }
                return Some(system_config);
            }
        }

        // Fall back to user config if it exists
        user_config.filter(|path| path.exists())
    }

    /// Get the full path to the config file for writing
    /// Returns: /etc/bunnylol/config.toml on Unix if writable (running as root)
    ///       or $XDG_CONFIG_HOME/bunnylol/config.toml otherwise
    pub fn get_config_path_for_writing() -> Option<PathBuf> {
        // If running as root (or /etc/bunnylol exists and is writable), use system config
        #[cfg(unix)]
        {
            let system_config_dir = PathBuf::from("/etc/bunnylol");
            if system_config_dir.exists() || std::fs::create_dir_all(&system_config_dir).is_ok() {
                return Some(system_config_dir.join("config.toml"));
            }
        }

        // Otherwise use user config
        Self::get_config_dir().map(|dir| dir.join("config.toml"))
    }

    /// Get the full path to the history file
    /// Returns: $XDG_DATA_HOME/bunnylol/history
    pub fn get_history_path() -> Option<PathBuf> {
        Self::get_data_dir().map(|dir| dir.join("history"))
    }

    /// Load configuration from the config file
    /// If the file doesn't exist, creates it with default configuration
    /// If the file exists but is invalid, returns an error
    pub fn load() -> Result<Self, String> {
        let config_path = match Self::get_config_path() {
            Some(path) => path,
            None => {
                // No config exists, try to create one
                if let Some(write_path) = Self::get_config_path_for_writing() {
                    let default_config = Self::default();
                    if let Err(e) = default_config.write_to_file(&write_path) {
                        eprintln!("Warning: Failed to write default config file: {}", e);
                        eprintln!("Continuing with default configuration...");
                    } else {
                        eprintln!("Created default config file at: {}", write_path.display());
                    }
                    return Ok(default_config);
                }
                return Ok(Self::default());
            }
        };

        // Config exists, read it
        Self::load_from_path(&config_path)
    }

    fn load_from_path(config_path: &PathBuf) -> Result<Self, String> {
        let contents = fs::read_to_string(config_path)
            .map_err(|e| format!("Failed to read config file {:?}: {}", config_path, e))?;

        toml::from_str(&contents)
            .map_err(|e| format!("Failed to parse config file {:?}: {}", config_path, e))
    }

    /// Write configuration to a file
    pub fn write_to_file(&self, path: &PathBuf) -> Result<(), String> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        // Serialize config to TOML with comments
        let toml_content = self.to_toml_with_comments();

        // Write to file
        fs::write(path, toml_content).map_err(|e| format!("Failed to write config file: {}", e))
    }

    /// Convert config to TOML string with helpful comments
    fn to_toml_with_comments(&self) -> String {
        let browser_line = match &self.browser {
            Some(b) => format!("browser = \"{}\"", b),
            None => "# browser = \"firefox\"".to_string(),
        };
        let aliases_content = if self.aliases.is_empty() {
            "# my-alias = \"gh username/repo\"".to_string()
        } else {
            self.aliases
                .iter()
                .map(|(k, v)| format!("{} = \"{}\"", k, v))
                .collect::<Vec<_>>()
                .join("\n")
        };
        let bindings_content = if self.bindings.is_empty() {
            // Commented examples for first-time users
            r#"# cal = "https://calendar.google.com/calendar/u/1/r"
# jira = "https://corp.atlassian.net/browse/{}"
# notion = { url = "https://notion.so/{}", description = "Notion page" }"#
                .to_string()
        } else {
            let mut entries: Vec<(&String, &CustomBinding)> = self.bindings.iter().collect();
            entries.sort_by_key(|(k, _)| k.to_lowercase());
            entries
                .into_iter()
                .map(|(k, v)| match v {
                    CustomBinding::Simple(url) => format!("{} = \"{}\"", k, url),
                    CustomBinding::Detailed { url, description } => match description {
                        Some(d) => {
                            format!("{} = {{ url = \"{}\", description = \"{}\" }}", k, url, d)
                        }
                        None => format!("{} = {{ url = \"{}\" }}", k, url),
                    },
                })
                .collect::<Vec<_>>()
                .join("\n")
        };
        let server_display_url_line = match &self.server.server_display_url {
            Some(url) => format!("server_display_url = \"{}\"", url),
            None => "# server_display_url = \"bunny.example.com\"".to_string(),
        };

        format!(
            r#"# Bunnylol Configuration File
# https://github.com/facebook/bunnylol.rs
#
# NOTE: The CLI reads this file on each run. The server reloads it when the
#       file's modified time changes.

# Browser to open URLs in (optional)
# Examples: "firefox", "chrome", "chromium", "safari"
# If not set, uses the OS default browser
{}

# Default search engine when command not recognized
# Options: "google" (default), "ddg", "bing"
default_search = "{}"

# Stock website provider
# Options: "yahoo" (default), "finviz", "tradingview", "google", "investing"
stock_provider = "{}"

# Custom command aliases
# Renames an existing built-in command (the value is re-parsed as a command).
# Example: work = "gh mycompany/repo"
[aliases]
{}

# Custom URL bindings
# Map a name directly to a URL. Use `{{}}` as a placeholder for arguments
# (URL-encoded at runtime). A template without `{{}}` is treated as a static URL.
#
# Two forms are supported:
#   name = "https://example.com/{{}}"
#   name = {{ url = "https://example.com/{{}}", description = "Shown on /bindings" }}
#
# NOTE: Hot-reload is not supported in this release.
#       Edit this file and restart bunnylol (serve or CLI) to apply changes.
# NOTE: Custom bindings never override built-in commands. On conflict, the
#       built-in wins and a warning is logged at startup.
[bindings]
{}

# Command history settings
[history]
enabled = {}
max_entries = {}

# Server configuration (for bunnylol serve)
# server_display_url: Public-facing URL shown in the bindings page
#   Smart defaults when protocol is omitted:
#     - "bunny.example.com" → "https://bunny.example.com"
#     - "localhost" or "127.0.0.1" or "0.0.0.0" → "http://localhost" (or IP)
#   You can also specify the full URL:
#     - "https://bunny.example.com" → used as-is
#     - "http://bunny.local" → used as-is
#   If not set, defaults to http://localhost:{{port}}
[server]
port = {}
address = "{}"
log_level = "{}"
{}
"#,
            browser_line,
            self.default_search,
            self.stock_provider,
            aliases_content,
            bindings_content,
            self.history.enabled,
            self.history.max_entries,
            self.server.port,
            self.server.address,
            self.server.log_level,
            server_display_url_line,
        )
    }

    /// Resolve a command, checking aliases first
    /// Returns the resolved command (either from alias or original)
    pub fn resolve_command(&self, command: &str) -> String {
        self.aliases
            .get(command)
            .cloned()
            .unwrap_or_else(|| command.to_string())
    }

    /// Resolve a user-defined custom binding, if one exists for `command`.
    ///
    /// Returns `None` when no binding matches — callers should fall back to
    /// the built-in registry or the default search. Built-in commands are
    /// checked **before** this method by `BunnylolCommandRegistry`, so a
    /// binding that shadows a built-in is silently ignored at runtime
    /// (and produces a warning at startup via [`validate_custom_bindings`]).
    pub fn resolve_custom_binding(&self, command: &str, full_args: &str) -> Option<String> {
        let binding = self.bindings.get(command)?;
        Some(apply_binding_template(
            binding.url_template(),
            command,
            full_args,
        ))
    }

    /// Validate this config's `[bindings]` against the set of built-in
    /// command names. Returns the list of user bindings that collide with a
    /// built-in (these are kept in config but will be shadowed at runtime).
    ///
    /// Note: TOML parsing already rejects duplicate keys within `[bindings]`
    /// itself, so this only needs to check against the built-in set.
    pub fn validate_custom_bindings(
        &self,
        builtin_names: &std::collections::HashSet<&'static str>,
    ) -> Vec<BindingConflict> {
        let mut conflicts = Vec::new();
        for (name, binding) in &self.bindings {
            if builtin_names.contains(name.as_str()) {
                conflicts.push(BindingConflict {
                    name: name.clone(),
                    user_url: binding.url_template().to_string(),
                });
            }
        }
        // Stable, deterministic output for logs and tests
        conflicts.sort_by(|a, b| a.name.cmp(&b.name));
        conflicts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = BunnylolConfig::default();
        assert_eq!(config.browser, None);
        assert_eq!(config.default_search, "google");
        assert_eq!(config.stock_provider, "yahoo");
        assert!(config.aliases.is_empty());
        assert!(config.history.enabled);
        assert_eq!(config.history.max_entries, 1000);
        assert_eq!(config.server.port, 8000);
        assert_eq!(config.server.address, "127.0.0.1");
        assert_eq!(config.server.log_level, "normal");
        assert_eq!(config.server.server_display_url, None);
    }

    #[test]
    fn test_resolve_command_with_alias() {
        let mut config = BunnylolConfig::default();
        config
            .aliases
            .insert("work".to_string(), "gh mycompany".to_string());

        assert_eq!(config.resolve_command("work"), "gh mycompany");
        assert_eq!(config.resolve_command("ig"), "ig"); // No alias
    }

    #[test]
    fn test_resolved_alias_produces_correct_redirect() {
        let mut config = BunnylolConfig::default();
        config
            .aliases
            .insert("work".to_string(), "gh mycompany".to_string());

        let resolved = config.resolve_command("work");
        let command = crate::utils::get_command_from_query_string(&resolved);
        let url = crate::BunnylolCommandRegistry::process_command(command, &resolved);
        assert_eq!(
            url,
            "https://github.com/search?q=mycompany&type=repositories"
        );
    }

    #[test]
    fn test_server_config_defaults() {
        let config = ServerConfig::default();
        assert_eq!(config.port, 8000);
        assert_eq!(config.address, "127.0.0.1");
        assert_eq!(config.log_level, "normal");
    }

    #[test]
    #[cfg(feature = "cli")]
    fn test_parse_valid_toml() {
        let toml_str = r#"
            browser = "firefox"
            default_search = "ddg"

            [aliases]
            work = "gh mycompany"
            blog = "gh username/blog"

            [history]
            enabled = false
            max_entries = 500

            [server]
            port = 9000
            address = "0.0.0.0"
            log_level = "debug"
        "#;

        let config: BunnylolConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.browser, Some("firefox".to_string()));
        assert_eq!(config.default_search, "ddg");
        assert_eq!(
            config.aliases.get("work"),
            Some(&"gh mycompany".to_string())
        );
        assert_eq!(
            config.aliases.get("blog"),
            Some(&"gh username/blog".to_string())
        );
        assert!(!config.history.enabled);
        assert_eq!(config.history.max_entries, 500);
        assert_eq!(config.server.port, 9000);
        assert_eq!(config.server.address, "0.0.0.0");
        assert_eq!(config.server.log_level, "debug");
    }

    #[test]
    fn test_server_display_url_defaults() {
        let config = ServerConfig::default();
        assert_eq!(config.server_display_url, None);
    }

    #[test]
    fn test_get_display_url_with_domain() {
        let mut config = ServerConfig::default();
        config.server_display_url = Some("bunny.alichtman.com".to_string());
        assert_eq!(config.get_display_url(), "https://bunny.alichtman.com");
    }

    #[test]
    fn test_get_display_url_with_https() {
        let mut config = ServerConfig::default();
        config.server_display_url = Some("https://bunny.example.com".to_string());
        assert_eq!(config.get_display_url(), "https://bunny.example.com");
    }

    #[test]
    fn test_get_display_url_with_http() {
        let mut config = ServerConfig::default();
        config.server_display_url = Some("http://localhost:8000".to_string());
        assert_eq!(config.get_display_url(), "http://localhost:8000");
    }

    #[test]
    fn test_get_display_url_fallback() {
        let config = ServerConfig::default();
        assert_eq!(config.get_display_url(), "http://localhost:8000");

        let mut config2 = ServerConfig::default();
        config2.port = 9000;
        assert_eq!(config2.get_display_url(), "http://localhost:9000");
    }

    #[test]
    fn test_get_display_url_with_whitespace() {
        let mut config = ServerConfig::default();
        config.server_display_url = Some("  bunny.example.com  ".to_string());
        assert_eq!(config.get_display_url(), "https://bunny.example.com");
    }

    #[test]
    fn test_get_display_url_localhost_bare() {
        let mut config = ServerConfig::default();
        config.server_display_url = Some("localhost".to_string());
        assert_eq!(config.get_display_url(), "http://localhost");
    }

    #[test]
    fn test_get_display_url_localhost_with_port() {
        let mut config = ServerConfig::default();
        config.server_display_url = Some("localhost:8000".to_string());
        assert_eq!(config.get_display_url(), "http://localhost:8000");
    }

    #[test]
    fn test_get_display_url_127_0_0_1() {
        let mut config = ServerConfig::default();
        config.server_display_url = Some("127.0.0.1".to_string());
        assert_eq!(config.get_display_url(), "http://127.0.0.1");
    }

    #[test]
    fn test_get_display_url_127_0_0_1_with_port() {
        let mut config = ServerConfig::default();
        config.server_display_url = Some("127.0.0.1:8000".to_string());
        assert_eq!(config.get_display_url(), "http://127.0.0.1:8000");
    }

    #[test]
    fn test_get_display_url_0_0_0_0() {
        let mut config = ServerConfig::default();
        config.server_display_url = Some("0.0.0.0:8000".to_string());
        assert_eq!(config.get_display_url(), "http://0.0.0.0:8000");
    }

    // -----------------------------------------------------------------
    // Custom bindings ([bindings] table) tests
    // -----------------------------------------------------------------

    use std::collections::HashSet;

    #[test]
    fn test_default_bindings_empty() {
        let config = BunnylolConfig::default();
        assert!(config.bindings.is_empty());
    }

    #[test]
    #[cfg(feature = "cli")]
    fn test_parse_bindings_short_form() {
        let toml_str = r#"
            [bindings]
            cal = "https://calendar.google.com/calendar/u/1/r"
        "#;
        let config: BunnylolConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(
            config.bindings.get("cal"),
            Some(&CustomBinding::Simple(
                "https://calendar.google.com/calendar/u/1/r".to_string()
            ))
        );
    }

    #[test]
    #[cfg(feature = "cli")]
    fn test_parse_bindings_detailed_form() {
        let toml_str = r#"
            [bindings]
            jira = { url = "https://corp.atlassian.net/browse/{}", description = "Jira ticket" }
        "#;
        let config: BunnylolConfig = toml::from_str(toml_str).unwrap();
        match config.bindings.get("jira") {
            Some(CustomBinding::Detailed { url, description }) => {
                assert_eq!(url, "https://corp.atlassian.net/browse/{}");
                assert_eq!(description.as_deref(), Some("Jira ticket"));
            }
            other => panic!("Expected detailed binding, got {:?}", other),
        }
    }

    #[test]
    #[cfg(feature = "cli")]
    fn test_parse_bindings_detailed_form_without_description() {
        let toml_str = r#"
            [bindings]
            corp = { url = "https://example.com" }
        "#;
        let config: BunnylolConfig = toml::from_str(toml_str).unwrap();
        match config.bindings.get("corp") {
            Some(CustomBinding::Detailed { url, description }) => {
                assert_eq!(url, "https://example.com");
                assert_eq!(description, &None);
            }
            other => panic!("Expected detailed binding, got {:?}", other),
        }
    }

    #[test]
    #[cfg(feature = "cli")]
    fn test_parse_bindings_mixed_forms() {
        let toml_str = r#"
            [bindings]
            cal = "https://calendar.google.com/calendar/u/1/r"
            jira = { url = "https://corp.atlassian.net/browse/{}", description = "Jira" }
        "#;
        let config: BunnylolConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.bindings.len(), 2);
        assert!(matches!(
            config.bindings.get("cal"),
            Some(CustomBinding::Simple(_))
        ));
        assert!(matches!(
            config.bindings.get("jira"),
            Some(CustomBinding::Detailed { .. })
        ));
    }

    #[test]
    fn test_resolve_custom_binding_static() {
        let mut config = BunnylolConfig::default();
        config.bindings.insert(
            "cal".to_string(),
            CustomBinding::Simple("https://calendar.google.com/calendar/u/1/r".to_string()),
        );
        assert_eq!(
            config.resolve_custom_binding("cal", "cal"),
            Some("https://calendar.google.com/calendar/u/1/r".to_string())
        );
    }

    #[test]
    fn test_resolve_custom_binding_static_ignores_extra_args() {
        // A template with no `{}` is a static URL; trailing args are ignored.
        let mut config = BunnylolConfig::default();
        config.bindings.insert(
            "cal".to_string(),
            CustomBinding::Simple("https://calendar.google.com/".to_string()),
        );
        assert_eq!(
            config.resolve_custom_binding("cal", "cal tomorrow lunch"),
            Some("https://calendar.google.com/".to_string())
        );
    }

    #[test]
    fn test_resolve_custom_binding_templated() {
        let mut config = BunnylolConfig::default();
        config.bindings.insert(
            "jira".to_string(),
            CustomBinding::Simple("https://corp.atlassian.net/browse/{}".to_string()),
        );
        assert_eq!(
            config.resolve_custom_binding("jira", "jira PROJ-123"),
            Some("https://corp.atlassian.net/browse/PROJ-123".to_string())
        );
    }

    #[test]
    fn test_resolve_custom_binding_templated_url_encodes_args() {
        let mut config = BunnylolConfig::default();
        config.bindings.insert(
            "wiki".to_string(),
            CustomBinding::Simple("https://example.com/?q={}".to_string()),
        );
        // Spaces become %20, special characters in the FRAGMENT set are escaped.
        assert_eq!(
            config.resolve_custom_binding("wiki", "wiki hello world"),
            Some("https://example.com/?q=hello%20world".to_string())
        );
    }

    #[test]
    fn test_resolve_custom_binding_templated_empty_args() {
        let mut config = BunnylolConfig::default();
        config.bindings.insert(
            "jira".to_string(),
            CustomBinding::Simple("https://corp.atlassian.net/browse/{}".to_string()),
        );
        // Bare command with no args: {} substituted with empty string.
        assert_eq!(
            config.resolve_custom_binding("jira", "jira"),
            Some("https://corp.atlassian.net/browse/".to_string())
        );
    }

    #[test]
    fn test_resolve_custom_binding_returns_none_when_missing() {
        let config = BunnylolConfig::default();
        assert_eq!(config.resolve_custom_binding("nope", "nope"), None);
    }

    #[test]
    fn test_resolve_custom_binding_detailed_form_uses_url() {
        let mut config = BunnylolConfig::default();
        config.bindings.insert(
            "jira".to_string(),
            CustomBinding::Detailed {
                url: "https://corp.atlassian.net/browse/{}".to_string(),
                description: Some("Jira ticket".to_string()),
            },
        );
        assert_eq!(
            config.resolve_custom_binding("jira", "jira PROJ-1"),
            Some("https://corp.atlassian.net/browse/PROJ-1".to_string())
        );
    }

    #[test]
    fn test_validate_custom_bindings_reports_conflicts() {
        let mut config = BunnylolConfig::default();
        config.bindings.insert(
            "gh".to_string(),
            CustomBinding::Simple("https://example.com/my-fork".to_string()),
        );
        config.bindings.insert(
            "cal".to_string(),
            CustomBinding::Simple("https://calendar.google.com/calendar/u/1/r".to_string()),
        );
        let builtins: HashSet<&'static str> = ["gh", "ig", "yt"].into_iter().collect();
        let conflicts = config.validate_custom_bindings(&builtins);
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].name, "gh");
        assert_eq!(conflicts[0].user_url, "https://example.com/my-fork");
    }

    #[test]
    fn test_validate_custom_bindings_empty_when_no_conflicts() {
        let mut config = BunnylolConfig::default();
        config.bindings.insert(
            "cal".to_string(),
            CustomBinding::Simple("https://calendar.google.com/calendar/u/1/r".to_string()),
        );
        let builtins: HashSet<&'static str> = ["gh", "ig"].into_iter().collect();
        let conflicts = config.validate_custom_bindings(&builtins);
        assert!(conflicts.is_empty());
    }

    #[test]
    fn test_validate_custom_bindings_sorted_deterministic() {
        // Multiple conflicts should be returned in a stable, alphabetical order
        // so log lines and tests don't flake on HashMap iteration order.
        let mut config = BunnylolConfig::default();
        for name in ["zsh", "abc", "mno", "gh"] {
            config.bindings.insert(
                name.to_string(),
                CustomBinding::Simple("https://example.com".to_string()),
            );
        }
        let builtins: HashSet<&'static str> = ["zsh", "abc", "mno", "gh"].into_iter().collect();
        let conflicts = config.validate_custom_bindings(&builtins);
        let names: Vec<&str> = conflicts.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, vec!["abc", "gh", "mno", "zsh"]);
    }

    #[test]
    fn test_existing_aliases_unaffected_by_bindings() {
        // Regression guard: the new [bindings] field must not break the
        // existing [aliases] resolution path.
        let mut config = BunnylolConfig::default();
        config
            .aliases
            .insert("work".to_string(), "gh mycompany".to_string());
        config.bindings.insert(
            "cal".to_string(),
            CustomBinding::Simple("https://calendar.google.com".to_string()),
        );
        assert_eq!(config.resolve_command("work"), "gh mycompany");
        assert_eq!(config.resolve_command("ig"), "ig");
        assert_eq!(
            config.resolve_custom_binding("cal", "cal"),
            Some("https://calendar.google.com".to_string())
        );
    }

    #[test]
    #[cfg(feature = "cli")]
    fn test_write_then_parse_roundtrip_with_bindings() {
        // Writing a config with bindings and parsing it back must yield the same data.
        let mut config = BunnylolConfig::default();
        config.bindings.insert(
            "cal".to_string(),
            CustomBinding::Simple("https://calendar.google.com/calendar/u/1/r".to_string()),
        );
        config.bindings.insert(
            "jira".to_string(),
            CustomBinding::Detailed {
                url: "https://corp.atlassian.net/browse/{}".to_string(),
                description: Some("Jira".to_string()),
            },
        );

        let toml_text = config.to_toml_with_comments();
        let parsed: BunnylolConfig =
            toml::from_str(&toml_text).expect("Generated config must be parseable as TOML");
        assert_eq!(parsed.bindings, config.bindings);
    }

    #[test]
    fn test_generated_config_includes_restart_note() {
        // Surface #1: the generated default config must surface the
        // restart-required note next to [bindings].
        let config = BunnylolConfig::default();
        let toml_text = config.to_toml_with_comments();
        let bindings_idx = toml_text
            .find("[bindings]")
            .expect("[bindings] section must be present");
        let preamble = &toml_text[..bindings_idx];
        assert!(
            preamble.contains("Hot-reload is not supported"),
            "Restart note must appear in the comment block above [bindings]"
        );
    }

    #[test]
    #[cfg(feature = "cli")]
    fn test_parse_server_display_url_from_toml() {
        let toml_str = r#"
            [server]
            port = 8000
            address = "0.0.0.0"
            log_level = "normal"
            server_display_url = "bunny.alichtman.com"
        "#;

        let config: BunnylolConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(
            config.server.server_display_url,
            Some("bunny.alichtman.com".to_string())
        );
        assert_eq!(
            config.server.get_display_url(),
            "https://bunny.alichtman.com"
        );
    }

    #[test]
    #[cfg(feature = "cli")]
    fn test_config_reloader_reloads_when_config_mtime_changes() {
        let mut dir = std::env::temp_dir();
        dir.push(format!(
            "bunnylol-reloader-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let config_dir = dir.join("bunnylol");
        fs::create_dir_all(&config_dir).unwrap();
        let config_path = config_dir.join("config.toml");
        fs::write(&config_path, "default_search = \"google\"\n").unwrap();

        let result = (|| {
            let initial = BunnylolConfig::load_from_path(&config_path).unwrap();
            let reloader = ConfigReloader::new_for_path(initial, config_path.clone());
            assert_eq!(reloader.current().default_search, "google");

            std::thread::sleep(std::time::Duration::from_millis(1100));
            fs::write(&config_path, "default_search = \"ddg\"\n").unwrap();

            assert_eq!(reloader.current().default_search, "ddg");
        })();

        fs::remove_dir_all(&dir).ok();
        result
    }

    #[test]
    #[cfg(feature = "cli")]
    fn test_config_reloader_keeps_previous_config_when_reload_is_invalid() {
        let mut dir = std::env::temp_dir();
        dir.push(format!(
            "bunnylol-reloader-invalid-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let config_dir = dir.join("bunnylol");
        fs::create_dir_all(&config_dir).unwrap();
        let config_path = config_dir.join("config.toml");
        fs::write(&config_path, "default_search = \"google\"\n").unwrap();

        let result = (|| {
            let initial = BunnylolConfig::load_from_path(&config_path).unwrap();
            let reloader = ConfigReloader::new_for_path(initial, config_path.clone());
            assert_eq!(reloader.current().default_search, "google");

            std::thread::sleep(std::time::Duration::from_millis(1100));
            fs::write(&config_path, "default_search = [").unwrap();

            assert_eq!(reloader.current().default_search, "google");
        })();

        fs::remove_dir_all(&dir).ok();
        result
    }
}
