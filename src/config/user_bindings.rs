/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use serde::{Deserialize, Serialize};

use super::BunnylolConfig;

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
///   equivalent to typing `gh org/repo`; `foo` is dropped.
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

    /// The URL template (for `Url`) or command string (for `Command`), used
    /// for displaying the binding's target in the /bindings web page and the
    /// CLI `--list` table.
    pub fn display_target(&self) -> &str {
        match self {
            UserBinding::Url { url, .. } => url,
            UserBinding::Command { command, .. } => command,
        }
    }
}

impl BunnylolConfig {
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
    /// with a built-in: bindings without `override = true` that share a
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
        conflicts.sort_by(|a, b| a.name.cmp(&b.name));
        conflicts
    }
}

/// Apply a `{}` template substitution to a URL binding. `command` is stripped
/// from the front of `full_args`, the remainder is URL-encoded, and
/// substituted in. A template with no `{}` is returned as-is.
fn apply_url_template(template: &str, command: &str, full_args: &str) -> String {
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
pub(super) fn format_user_binding_toml(name: &str, binding: &UserBinding) -> String {
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
    format!("{} = {{ {} }}", format_toml_key(name), parts.join(", "))
}

fn format_toml_key(key: &str) -> String {
    if !key.is_empty()
        && key
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        key.to_string()
    } else {
        format!("\"{}\"", escape_toml_string(key))
    }
}

fn escape_toml_string(s: &str) -> String {
    let mut escaped = String::new();
    for ch in s.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '\u{08}' => escaped.push_str("\\b"),
            '\u{0C}' => escaped.push_str("\\f"),
            ch if ch.is_control() => escaped.push_str(&format!("\\u{:04X}", ch as u32)),
            ch => escaped.push(ch),
        }
    }
    escaped
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

/// Result of validating a user binding against the built-in command set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingConflict {
    /// The name (TOML key) of the user binding that conflicts.
    pub name: String,
    /// The target string (URL template or command), kept for diagnostics.
    pub target: String,
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn url_binding(url: &str, override_builtin: bool) -> UserBinding {
        UserBinding::Url {
            url: url.to_string(),
            description: None,
            override_builtin,
        }
    }

    fn command_binding(command: &str) -> UserBinding {
        UserBinding::Command {
            command: command.to_string(),
            description: None,
            override_builtin: false,
        }
    }

    #[test]
    fn test_parse_user_bindings_variants_and_override() {
        let toml_str = r#"
            [user_bindings]
            jira = { url = "https://corp.atlassian.net/browse/{}", description = "Jira ticket" }
            cal = { url = "https://calendar.google.com/calendar/u/1/r" }
            work = { command = "gh mycompany/repo", description = "Work repo" }
            gh = { url = "https://example.com/my-fork", override = true }
        "#;
        let config: BunnylolConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.user_bindings.len(), 4);

        let jira = config.user_bindings.get("jira").unwrap();
        assert!(matches!(jira, UserBinding::Url { .. }));
        assert_eq!(
            jira.display_target(),
            "https://corp.atlassian.net/browse/{}"
        );
        assert_eq!(jira.description(), Some("Jira ticket"));
        assert!(!jira.overrides_builtin());

        let cal = config.user_bindings.get("cal").unwrap();
        assert!(matches!(cal, UserBinding::Url { .. }));
        assert_eq!(
            cal.display_target(),
            "https://calendar.google.com/calendar/u/1/r"
        );
        assert_eq!(cal.description(), None);

        let work = config.user_bindings.get("work").unwrap();
        assert!(matches!(work, UserBinding::Command { .. }));
        assert_eq!(work.display_target(), "gh mycompany/repo");
        assert_eq!(work.description(), Some("Work repo"));

        let gh = config.user_bindings.get("gh").unwrap();
        assert!(matches!(gh, UserBinding::Url { .. }));
        assert!(gh.overrides_builtin());
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
    fn test_resolve_user_bindings_url_variants_and_missing_entries() {
        let mut config = BunnylolConfig::default();
        config.user_bindings.insert(
            "cal".to_string(),
            url_binding("https://calendar.google.com/calendar/u/1/r", false),
        );
        config.user_bindings.insert(
            "jira".to_string(),
            url_binding("https://corp.atlassian.net/browse/{}", false),
        );
        config.user_bindings.insert(
            "wiki".to_string(),
            url_binding("https://example.com/?q={}", false),
        );
        config.user_bindings.insert(
            "gh".to_string(),
            url_binding("https://example.com/my-fork", true),
        );

        assert_eq!(
            config.resolve_user_binding("cal", "cal"),
            Some((
                ResolvedBinding::Url("https://calendar.google.com/calendar/u/1/r".to_string()),
                false
            ))
        );
        assert_eq!(
            config.resolve_user_binding("jira", "jira PROJ-123"),
            Some((
                ResolvedBinding::Url("https://corp.atlassian.net/browse/PROJ-123".to_string()),
                false
            ))
        );
        assert_eq!(
            config.resolve_user_binding("wiki", "wiki hello world"),
            Some((
                ResolvedBinding::Url("https://example.com/?q=hello%20world".to_string()),
                false
            ))
        );
        assert_eq!(
            config.resolve_user_binding("gh", "gh").unwrap(),
            (
                ResolvedBinding::Url("https://example.com/my-fork".to_string()),
                true
            )
        );
        assert_eq!(config.resolve_user_binding("nope", "nope"), None);
    }

    #[test]
    fn test_resolve_user_binding_command_returns_rewritten_string() {
        let mut config = BunnylolConfig::default();
        config
            .user_bindings
            .insert("work".to_string(), command_binding("gh mycompany/repo"));
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
    fn test_validate_user_bindings_conflicts_filters_and_sorts() {
        let mut config = BunnylolConfig::default();
        for name in ["zsh", "abc", "mno", "gh"] {
            config.user_bindings.insert(
                name.to_string(),
                UserBinding::Url {
                    url: format!("https://example.com/{}", name),
                    description: None,
                    override_builtin: false,
                },
            );
        }
        // Intentional override; must NOT be reported.
        config.user_bindings.insert(
            "ig".to_string(),
            url_binding("https://example.com/insta", true),
        );
        // No collision; irrelevant.
        config.user_bindings.insert(
            "cal".to_string(),
            url_binding("https://calendar.google.com", false),
        );

        let builtins: HashSet<&'static str> = ["zsh", "abc", "mno", "gh", "ig", "yt"]
            .into_iter()
            .collect();
        let conflicts = config.validate_user_bindings_conflicts(&builtins);
        let actual: Vec<(&str, &str)> = conflicts
            .iter()
            .map(|conflict| (conflict.name.as_str(), conflict.target.as_str()))
            .collect();
        assert_eq!(
            actual,
            vec![
                ("abc", "https://example.com/abc"),
                ("gh", "https://example.com/gh"),
                ("mno", "https://example.com/mno"),
                ("zsh", "https://example.com/zsh"),
            ]
        );
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
}
