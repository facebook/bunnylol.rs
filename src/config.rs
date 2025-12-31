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

/// Configuration for bunnylol CLI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BunnylolConfig {
    /// Browser to open URLs in (optional)
    /// Examples: "firefox", "chrome", "chromium", "safari"
    #[serde(default)]
    pub browser: Option<String>,

    /// Default search engine when command not recognized (optional)
    /// Options: "google" (default), "ddg", "bing"
    #[serde(default = "default_search_engine")]
    pub default_search: String,

    /// Custom command aliases
    #[serde(default)]
    pub aliases: HashMap<String, String>,

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
            aliases: HashMap::new(),
            history: HistoryConfig::default(),
            server: ServerConfig::default(),
        }
    }
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

    /// Get the full path to the config file
    /// Returns: /etc/bunnylol/config.toml (system-wide, preferred)
    ///       or $XDG_CONFIG_HOME/bunnylol/config.toml (user-specific fallback)
    pub fn get_config_path() -> Option<PathBuf> {
        // Check system-wide config first
        let system_config = PathBuf::from("/etc/bunnylol/config.toml");
        let user_config = Self::get_config_dir().map(|dir| dir.join("config.toml"));

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

        // Fall back to user config
        user_config
    }

    /// Get the full path to the config file for writing
    /// Returns: /etc/bunnylol/config.toml if writable (running as root)
    ///       or $XDG_CONFIG_HOME/bunnylol/config.toml otherwise
    pub fn get_config_path_for_writing() -> Option<PathBuf> {
        // If running as root (or /etc/bunnylol exists and is writable), use system config
        let system_config_dir = PathBuf::from("/etc/bunnylol");
        if system_config_dir.exists() || std::fs::create_dir_all(&system_config_dir).is_ok() {
            return Some(system_config_dir.join("config.toml"));
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
                        println!("Created default config file at: {}", write_path.display());
                    }
                    return Ok(default_config);
                }
                return Ok(Self::default());
            }
        };

        // Config exists, read it
        let contents = fs::read_to_string(&config_path)
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
        format!(
            r#"# Bunnylol Configuration File
# https://github.com/facebook/bunnylol.rs
#
# NOTE: Configuration is loaded once at server startup.
#       You must restart the server (bunnylol serve) to apply changes.

# Browser to open URLs in (optional)
# Examples: "firefox", "chrome", "chromium", "safari"
# If not set, uses system default browser
{}

# Default search engine when command not recognized
# Options: "google" (default), "ddg", "bing"
default_search = "{}"

# Custom command aliases
# Example: work = "gh mycompany/repo"
[aliases]
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
            if let Some(browser) = &self.browser {
                format!("browser = \"{}\"", browser)
            } else {
                "# browser = \"firefox\"".to_string()
            },
            self.default_search,
            if self.aliases.is_empty() {
                "# my-alias = \"gh username/repo\"".to_string()
            } else {
                self.aliases
                    .iter()
                    .map(|(k, v)| format!("{} = \"{}\"", k, v))
                    .collect::<Vec<_>>()
                    .join("\n")
            },
            self.history.enabled,
            self.history.max_entries,
            self.server.port,
            self.server.address,
            self.server.log_level,
            if let Some(url) = &self.server.server_display_url {
                format!("server_display_url = \"{}\"", url)
            } else {
                "# server_display_url = \"bunny.example.com\"".to_string()
            },
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

    /// Get the search engine URL for a query
    pub fn get_search_url(&self, query: &str) -> String {
        let encoded_query =
            percent_encoding::utf8_percent_encode(query, percent_encoding::NON_ALPHANUMERIC)
                .to_string();

        match self.default_search.as_str() {
            "ddg" | "duckduckgo" => format!("https://duckduckgo.com/?q={}", encoded_query),
            "bing" => format!("https://www.bing.com/search?q={}", encoded_query),
            _ => format!("https://www.google.com/search?q={}", encoded_query), // Default to Google
        }
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
    fn test_get_search_url_google() {
        let config = BunnylolConfig::default();
        let url = config.get_search_url("hello world");
        assert!(url.starts_with("https://www.google.com/search?q="));
        assert!(url.contains("hello"));
        assert!(url.contains("world"));
    }

    #[test]
    fn test_get_search_url_ddg() {
        let mut config = BunnylolConfig::default();
        config.default_search = "ddg".to_string();
        let url = config.get_search_url("test query");
        assert!(url.starts_with("https://duckduckgo.com/?q="));
    }

    #[test]
    fn test_get_search_url_bing() {
        let mut config = BunnylolConfig::default();
        config.default_search = "bing".to_string();
        let url = config.get_search_url("test query");
        assert!(url.starts_with("https://www.bing.com/search?q="));
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
}
