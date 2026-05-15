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

    /// User-defined bindings (URL shortcuts and command aliases).
    ///
    /// Each entry is one of:
    ///   - `Url`     { url, description?, override? }      — maps to a URL or URL template
    ///   - `Command` { command, description?, override? } — rewrites to another bunnylol command
    ///
    /// On a name collision with a built-in, the built-in wins by default. A
    /// user binding may opt in to shadowing a built-in with `override = true`.
    ///
    /// `[aliases]` (legacy) is also loaded and folded into this map as
    /// `Command` variants at load time. See `BunnylolConfig::load_from_path`.
    #[serde(default)]
    pub user_bindings: HashMap<String, UserBinding>,

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
            user_bindings: HashMap::new(),
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

/// A user-defined binding from `[user_bindings]` in the config file.
///
/// Two variants are accepted, both as inline tables:
///
/// ```toml
/// [user_bindings]
/// # URL binding: maps a name to a URL (or URL template with {}).
/// cal  = { url = "https://calendar.google.com/calendar/u/1/r" }
/// jira = { url = "https://corp.atlassian.net/browse/{}", description = "Jira ticket" }
///
/// # Command binding: rewrites to another bunnylol command.
/// work = { command = "gh mycompany/repo", description = "Work repo" }
/// ```
///
/// ## Semantics
///
/// - `Url` bindings support `{}` template substitution. At resolution time
///   the command prefix is stripped from the user input, the remainder is
///   URL-encoded, and substituted in. A template with no `{}` is treated as
///   a static URL and any arguments are ignored.
///
/// - `Command` bindings rewrite the input to the bound command verbatim.
///   They do **not** support `{}` templates and do **not** forward extra args.
///   Example: with `work = { command = "gh org/repo" }`, typing `work foo` is
///   equivalent to typing `gh org/repo` — `foo` is dropped.
///
/// - `Command` bindings dispatch into the registry **exactly once**: a
///   `Command` binding may resolve to a built-in or to the search fallback,
///   but it will not re-enter another `[user_bindings]` entry. This avoids
///   cycles like `a = { command = "b" }` / `b = { command = "a" }`.
///
/// - By default, built-ins win on a name collision. Set `override = true`
///   to make a user binding shadow a built-in command.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum UserBinding {
    /// Maps a name to a URL or URL template.
    Url {
        url: String,
        #[serde(default)]
        description: Option<String>,
        #[serde(default, rename = "override")]
        override_builtin: bool,
    },
    /// Rewrites a name to another bunnylol command (dispatched once, no recursion
    /// into other user bindings).
    Command {
        command: String,
        #[serde(default)]
        description: Option<String>,
        #[serde(default, rename = "override")]
        override_builtin: bool,
    },
}

impl UserBinding {
    /// The description shown on the /bindings web page, if any.
    pub fn description(&self) -> Option<&str> {
        match self {
            UserBinding::Url { description, .. } | UserBinding::Command { description, .. } => {
                description.as_deref()
            }
        }
    }

    /// Whether this binding asks to shadow a built-in command of the same name.
    pub fn overrides_builtin(&self) -> bool {
        match self {
            UserBinding::Url {
                override_builtin, ..
            }
            | UserBinding::Command {
                override_builtin, ..
            } => *override_builtin,
        }
    }

    /// Short label for display ("URL" or "CMD").
    pub fn kind_label(&self) -> &'static str {
        match self {
            UserBinding::Url { .. } => "URL",
            UserBinding::Command { .. } => "CMD",
        }
    }

    /// The URL template (for `Url`) or command string (for `Command`) — used
    /// for displaying the binding's target in the /bindings web page and the
    /// CLI `--list` table.
    pub fn display_target(&self) -> &str {
        match self {
            UserBinding::Url { url, .. } => url,
            UserBinding::Command { command, .. } => command,
        }
    }
}

/// Apply a `{}` template substitution to a URL binding. `command` is stripped
/// from the front of `full_args`, the remainder is URL-encoded, and
/// substituted in. A template with no `{}` is returned as-is.
pub(crate) fn apply_url_template(template: &str, command: &str, full_args: &str) -> String {
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

/// Format one `[user_bindings]` entry as its TOML inline-table representation.
fn format_user_binding_toml(name: &str, binding: &UserBinding) -> String {
    let mut parts: Vec<String> = Vec::new();
    match binding {
        UserBinding::Url {
            url,
            description,
            override_builtin,
        } => {
            parts.push(format!("url = \"{}\"", escape_toml_string(url)));
            if let Some(d) = description {
                parts.push(format!("description = \"{}\"", escape_toml_string(d)));
            }
            if *override_builtin {
                parts.push("override = true".to_string());
            }
        }
        UserBinding::Command {
            command,
            description,
            override_builtin,
        } => {
            parts.push(format!("command = \"{}\"", escape_toml_string(command)));
            if let Some(d) = description {
                parts.push(format!("description = \"{}\"", escape_toml_string(d)));
            }
            if *override_builtin {
                parts.push("override = true".to_string());
            }
        }
    }
    format!("{} = {{ {} }}", name, parts.join(", "))
}

fn escape_toml_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Fold `[aliases]` entries into `[user_bindings]` in-memory as `Command`
/// variants. Pure function — does not touch the on-disk file.
///
/// On a name collision between `[aliases]` and `[user_bindings]`,
/// `[user_bindings]` wins (a debug warning is logged).
fn fold_aliases_into_user_bindings(config: &mut BunnylolConfig) {
    for (name, command) in &config.aliases {
        if config.user_bindings.contains_key(name) {
            eprintln!(
                "Warning: '{}' is defined in both [aliases] and [user_bindings]; \
                 [user_bindings] wins.",
                name
            );
            continue;
        }
        config.user_bindings.insert(
            name.clone(),
            UserBinding::Command {
                command: command.clone(),
                description: None,
                override_builtin: false,
            },
        );
    }
}

/// Result of validating a user binding against the built-in command set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingConflict {
    /// The name (TOML key) of the user binding that conflicts.
    pub name: String,
    /// The target string (URL template or command) — kept for diagnostics.
    pub target: String,
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

        let mut config: BunnylolConfig = toml::from_str(&contents)
            .map_err(|e| format!("Failed to parse config file {:?}: {}", config_path, e))?;

        // Fold legacy [aliases] into [user_bindings] in-memory (the on-disk
        // file is left untouched — see Q2 in the plan).
        fold_aliases_into_user_bindings(&mut config);

        Ok(config)
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
        let user_bindings_content = if self.user_bindings.is_empty() {
            // Commented examples for first-time users
            r#"# cal  = { url = "https://calendar.google.com/calendar/u/1/r" }
# jira = { url = "https://corp.atlassian.net/browse/{}", description = "Jira ticket" }
# work = { command = "gh mycompany/repo", description = "Work repo" }"#
                .to_string()
        } else {
            let mut entries: Vec<(&String, &UserBinding)> = self.user_bindings.iter().collect();
            entries.sort_by_key(|(k, _)| k.to_lowercase());
            entries
                .into_iter()
                .map(|(k, v)| format_user_binding_toml(k, v))
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

# Legacy command aliases (DEPRECATED — use [user_bindings] instead).
# Entries here are folded into [user_bindings] at load time as `command = ...`
# bindings. Example: work = "gh mycompany/repo"
[aliases]
{}

# User-defined bindings. Two variants, both as inline tables:
#
#   # URL binding: maps a name to a URL (use {{}} as a placeholder for args).
#   cal  = {{ url = "https://calendar.google.com/calendar/u/1/r" }}
#   jira = {{ url = "https://corp.atlassian.net/browse/{{}}", description = "Jira ticket" }}
#
#   # Command binding: rewrites the input to another bunnylol command.
#   work = {{ command = "gh mycompany/repo", description = "Work repo" }}
#
# By default, built-in commands win on a name collision. Add `override = true`
# to a binding to shadow a built-in (e.g. `gh = {{ command = "...", override = true }}`).
#
# Note: Command bindings do not forward extra args ({{}} is URL-only) and never
# recurse into other user bindings (dispatch once into the registry).
[user_bindings]
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
            user_bindings_content,
            self.history.enabled,
            self.history.max_entries,
            self.server.port,
            self.server.address,
            self.server.log_level,
            server_display_url_line,
        )
    }

    /// Resolve a user binding for `name`, if one exists.
    ///
    /// Returns `Some((resolved, overrides_builtin))`:
    /// - `resolved` is either a final URL (for `Url` bindings, after `{}`
    ///   substitution) or a rewritten command string (for `Command` bindings).
    /// - `overrides_builtin` reflects the binding's `override = true` flag,
    ///   used by `BunnylolCommandRegistry` to decide whether the binding
    ///   shadows a built-in (override = true, tier 2) or yields to it
    ///   (override = false, tier 4).
    pub fn resolve_user_binding(
        &self,
        name: &str,
        full_args: &str,
    ) -> Option<(ResolvedBinding, bool)> {
        let binding = self.user_bindings.get(name)?;
        let resolved = match binding {
            UserBinding::Url { url, .. } => {
                ResolvedBinding::Url(apply_url_template(url, name, full_args))
            }
            UserBinding::Command { command, .. } => ResolvedBinding::Command(command.clone()),
        };
        Some((resolved, binding.overrides_builtin()))
    }

    /// Validate this config's `[user_bindings]` against the set of built-in
    /// command names. Returns the list of bindings that **silently** collide
    /// with a built-in — i.e. bindings without `override = true` that share a
    /// name with a built-in. These bindings are kept in config but shadowed
    /// at runtime.
    ///
    /// Bindings with `override = true` are intentionally shadowing the
    /// built-in and are not reported as conflicts.
    pub fn validate_user_bindings_conflicts(
        &self,
        builtin_names: &std::collections::HashSet<&'static str>,
    ) -> Vec<BindingConflict> {
        let mut conflicts = Vec::new();
        for (name, binding) in &self.user_bindings {
            if builtin_names.contains(name.as_str()) && !binding.overrides_builtin() {
                conflicts.push(BindingConflict {
                    name: name.clone(),
                    target: binding.display_target().to_string(),
                });
            }
        }
        // Stable, deterministic output for logs and tests
        conflicts.sort_by(|a, b| a.name.cmp(&b.name));
        conflicts
    }
}

/// Outcome of resolving a user binding. The registry interprets these:
/// `Url` is returned to the caller as the final URL; `Command` is a rewritten
/// command string that the registry dispatches once (and never recurses back
/// into user bindings).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedBinding {
    Url(String),
    Command(String),
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
    fn test_aliases_fold_into_user_bindings_as_command_variant() {
        let mut config = BunnylolConfig::default();
        config
            .aliases
            .insert("work".to_string(), "gh mycompany".to_string());
        fold_aliases_into_user_bindings(&mut config);

        match config.user_bindings.get("work") {
            Some(UserBinding::Command {
                command,
                description,
                override_builtin,
            }) => {
                assert_eq!(command, "gh mycompany");
                assert_eq!(description, &None);
                assert!(!override_builtin);
            }
            other => panic!("Expected Command binding, got {:?}", other),
        }
    }

    #[test]
    fn test_aliases_fold_user_bindings_wins_on_conflict() {
        let mut config = BunnylolConfig::default();
        config
            .aliases
            .insert("work".to_string(), "gh from-aliases".to_string());
        config.user_bindings.insert(
            "work".to_string(),
            UserBinding::Command {
                command: "gh from-user-bindings".to_string(),
                description: None,
                override_builtin: false,
            },
        );
        fold_aliases_into_user_bindings(&mut config);

        // user_bindings wins; the aliases entry is dropped on the floor (the
        // [aliases] HashMap still exists, but it's not folded over the top).
        match config.user_bindings.get("work") {
            Some(UserBinding::Command { command, .. }) => {
                assert_eq!(command, "gh from-user-bindings");
            }
            other => panic!("Expected user_bindings entry to win, got {:?}", other),
        }
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
    // [user_bindings] table tests
    // -----------------------------------------------------------------

    use std::collections::HashSet;

    #[test]
    fn test_default_user_bindings_empty() {
        let config = BunnylolConfig::default();
        assert!(config.user_bindings.is_empty());
    }

    #[test]
    fn test_parse_user_bindings_url_with_description() {
        let toml_str = r#"
            [user_bindings]
            jira = { url = "https://corp.atlassian.net/browse/{}", description = "Jira ticket" }
        "#;
        let config: BunnylolConfig = toml::from_str(toml_str).unwrap();
        match config.user_bindings.get("jira") {
            Some(UserBinding::Url {
                url,
                description,
                override_builtin,
            }) => {
                assert_eq!(url, "https://corp.atlassian.net/browse/{}");
                assert_eq!(description.as_deref(), Some("Jira ticket"));
                assert!(!override_builtin);
            }
            other => panic!("Expected Url binding, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_user_bindings_url_without_description() {
        let toml_str = r#"
            [user_bindings]
            cal = { url = "https://calendar.google.com/calendar/u/1/r" }
        "#;
        let config: BunnylolConfig = toml::from_str(toml_str).unwrap();
        match config.user_bindings.get("cal") {
            Some(UserBinding::Url {
                url, description, ..
            }) => {
                assert_eq!(url, "https://calendar.google.com/calendar/u/1/r");
                assert_eq!(description, &None);
            }
            other => panic!("Expected Url binding, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_user_bindings_command_variant() {
        let toml_str = r#"
            [user_bindings]
            work = { command = "gh mycompany/repo", description = "Work repo" }
        "#;
        let config: BunnylolConfig = toml::from_str(toml_str).unwrap();
        match config.user_bindings.get("work") {
            Some(UserBinding::Command {
                command,
                description,
                override_builtin,
            }) => {
                assert_eq!(command, "gh mycompany/repo");
                assert_eq!(description.as_deref(), Some("Work repo"));
                assert!(!override_builtin);
            }
            other => panic!("Expected Command binding, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_user_bindings_mixed_url_and_command() {
        let toml_str = r#"
            [user_bindings]
            jira = { url = "https://corp.atlassian.net/browse/{}" }
            work = { command = "gh mycompany/repo" }
        "#;
        let config: BunnylolConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.user_bindings.len(), 2);
        assert!(matches!(
            config.user_bindings.get("jira"),
            Some(UserBinding::Url { .. })
        ));
        assert!(matches!(
            config.user_bindings.get("work"),
            Some(UserBinding::Command { .. })
        ));
    }

    #[test]
    fn test_parse_user_bindings_override_flag() {
        let toml_str = r#"
            [user_bindings]
            gh = { url = "https://example.com/my-fork", override = true }
        "#;
        let config: BunnylolConfig = toml::from_str(toml_str).unwrap();
        match config.user_bindings.get("gh") {
            Some(b) => assert!(b.overrides_builtin()),
            None => panic!("Expected gh binding to be present"),
        }
    }

    #[test]
    fn test_parse_user_bindings_rejects_short_form() {
        // Structured form is required (Q3). A bare string value must NOT
        // deserialize as a binding.
        let toml_str = r#"
            [user_bindings]
            cal = "https://calendar.google.com/calendar/u/1/r"
        "#;
        let result: Result<BunnylolConfig, _> = toml::from_str(toml_str);
        assert!(
            result.is_err(),
            "Short form (bare URL string) must be rejected. Got: {:?}",
            result
        );
    }

    #[test]
    fn test_resolve_user_binding_url_static() {
        let mut config = BunnylolConfig::default();
        config.user_bindings.insert(
            "cal".to_string(),
            UserBinding::Url {
                url: "https://calendar.google.com/calendar/u/1/r".to_string(),
                description: None,
                override_builtin: false,
            },
        );
        let resolved = config.resolve_user_binding("cal", "cal");
        assert_eq!(
            resolved,
            Some((
                ResolvedBinding::Url("https://calendar.google.com/calendar/u/1/r".to_string()),
                false
            ))
        );
    }

    #[test]
    fn test_resolve_user_binding_url_template_substitution() {
        let mut config = BunnylolConfig::default();
        config.user_bindings.insert(
            "jira".to_string(),
            UserBinding::Url {
                url: "https://corp.atlassian.net/browse/{}".to_string(),
                description: None,
                override_builtin: false,
            },
        );
        assert_eq!(
            config.resolve_user_binding("jira", "jira PROJ-123"),
            Some((
                ResolvedBinding::Url("https://corp.atlassian.net/browse/PROJ-123".to_string()),
                false
            ))
        );
    }

    #[test]
    fn test_resolve_user_binding_url_template_encodes_args() {
        let mut config = BunnylolConfig::default();
        config.user_bindings.insert(
            "wiki".to_string(),
            UserBinding::Url {
                url: "https://example.com/?q={}".to_string(),
                description: None,
                override_builtin: false,
            },
        );
        assert_eq!(
            config.resolve_user_binding("wiki", "wiki hello world"),
            Some((
                ResolvedBinding::Url("https://example.com/?q=hello%20world".to_string()),
                false
            ))
        );
    }

    #[test]
    fn test_resolve_user_binding_command_returns_rewritten_string() {
        let mut config = BunnylolConfig::default();
        config.user_bindings.insert(
            "work".to_string(),
            UserBinding::Command {
                command: "gh mycompany/repo".to_string(),
                description: None,
                override_builtin: false,
            },
        );
        // Command bindings do not substitute or forward args; the registry's
        // dispatch_resolved consumes the rewritten string verbatim.
        assert_eq!(
            config.resolve_user_binding("work", "work extra args dropped"),
            Some((
                ResolvedBinding::Command("gh mycompany/repo".to_string()),
                false
            ))
        );
    }

    #[test]
    fn test_resolve_user_binding_returns_none_when_missing() {
        let config = BunnylolConfig::default();
        assert_eq!(config.resolve_user_binding("nope", "nope"), None);
    }

    #[test]
    fn test_resolve_user_binding_reports_override_flag() {
        let mut config = BunnylolConfig::default();
        config.user_bindings.insert(
            "gh".to_string(),
            UserBinding::Url {
                url: "https://example.com/my-fork".to_string(),
                description: None,
                override_builtin: true,
            },
        );
        let resolved = config.resolve_user_binding("gh", "gh").unwrap();
        assert!(resolved.1, "override flag must propagate");
    }

    #[test]
    fn test_validate_user_bindings_conflicts_reports_silent_shadows_only() {
        let mut config = BunnylolConfig::default();
        // Silently shadowed — should be reported.
        config.user_bindings.insert(
            "gh".to_string(),
            UserBinding::Url {
                url: "https://example.com/my-fork".to_string(),
                description: None,
                override_builtin: false,
            },
        );
        // Intentional override — must NOT be reported.
        config.user_bindings.insert(
            "ig".to_string(),
            UserBinding::Url {
                url: "https://example.com/insta".to_string(),
                description: None,
                override_builtin: true,
            },
        );
        // No collision — irrelevant.
        config.user_bindings.insert(
            "cal".to_string(),
            UserBinding::Url {
                url: "https://calendar.google.com".to_string(),
                description: None,
                override_builtin: false,
            },
        );

        let builtins: HashSet<&'static str> = ["gh", "ig", "yt"].into_iter().collect();
        let conflicts = config.validate_user_bindings_conflicts(&builtins);
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].name, "gh");
        assert_eq!(conflicts[0].target, "https://example.com/my-fork");
    }

    #[test]
    fn test_validate_user_bindings_conflicts_sorted_deterministic() {
        let mut config = BunnylolConfig::default();
        for name in ["zsh", "abc", "mno", "gh"] {
            config.user_bindings.insert(
                name.to_string(),
                UserBinding::Url {
                    url: "https://example.com".to_string(),
                    description: None,
                    override_builtin: false,
                },
            );
        }
        let builtins: HashSet<&'static str> = ["zsh", "abc", "mno", "gh"].into_iter().collect();
        let conflicts = config.validate_user_bindings_conflicts(&builtins);
        let names: Vec<&str> = conflicts.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, vec!["abc", "gh", "mno", "zsh"]);
    }

    #[test]
    #[cfg(feature = "cli")]
    fn test_write_then_parse_roundtrip_with_user_bindings() {
        let mut config = BunnylolConfig::default();
        config.user_bindings.insert(
            "cal".to_string(),
            UserBinding::Url {
                url: "https://calendar.google.com/calendar/u/1/r".to_string(),
                description: None,
                override_builtin: false,
            },
        );
        config.user_bindings.insert(
            "jira".to_string(),
            UserBinding::Url {
                url: "https://corp.atlassian.net/browse/{}".to_string(),
                description: Some("Jira".to_string()),
                override_builtin: false,
            },
        );
        config.user_bindings.insert(
            "work".to_string(),
            UserBinding::Command {
                command: "gh mycompany/repo".to_string(),
                description: Some("Work repo".to_string()),
                override_builtin: false,
            },
        );

        let toml_text = config.to_toml_with_comments();
        let parsed: BunnylolConfig =
            toml::from_str(&toml_text).expect("Generated config must be parseable as TOML");
        assert_eq!(parsed.user_bindings, config.user_bindings);
    }

    #[test]
    fn test_generated_config_drops_restart_note_and_documents_user_bindings() {
        // After PR #48, hot reload is supported. The generated default config
        // must NOT advertise a restart-required surface, and SHOULD mention
        // that [aliases] is deprecated.
        let config = BunnylolConfig::default();
        let toml_text = config.to_toml_with_comments();
        assert!(
            !toml_text.contains("Hot-reload is not supported"),
            "Restart-required surface must be removed now that hot reload works"
        );
        assert!(
            toml_text.contains("[user_bindings]"),
            "Generated config must contain a [user_bindings] section"
        );
        assert!(
            toml_text.contains("DEPRECATED"),
            "Generated config must mark [aliases] as deprecated"
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
