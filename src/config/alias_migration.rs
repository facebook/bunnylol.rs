/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::io::Write;
use std::path::Path;

use super::user_bindings::format_user_binding_toml;
use super::{BunnylolConfig, UserBinding};

pub(super) enum AliasMigrationError {
    Validation(String),
    Write(String),
}

pub(super) fn migrate_aliases_to_user_bindings(
    config_path: &Path,
    contents: &str,
    config: &BunnylolConfig,
) -> Result<Option<BunnylolConfig>, AliasMigrationError> {
    let Some(migrated_contents) = migrate_aliases_to_user_bindings_toml(contents, config) else {
        return Ok(None);
    };

    let migrated_config = toml::from_str(&migrated_contents)
        .map_err(|e| AliasMigrationError::Validation(e.to_string()))?;
    write_config_atomically(config_path, &migrated_contents).map_err(AliasMigrationError::Write)?;

    Ok(Some(migrated_config))
}

#[derive(Debug, Clone, Copy)]
struct TomlTableSection {
    header_start: usize,
    body_start: usize,
    end: usize,
}

fn find_toml_table_section(contents: &str, table_name: &str) -> Option<TomlTableSection> {
    let sections = toml_table_headers(contents);

    sections
        .iter()
        .enumerate()
        .find(|(_, (name, _, _))| *name == table_name)
        .map(|(idx, (_, header_start, body_start))| TomlTableSection {
            header_start: *header_start,
            body_start: *body_start,
            end: sections
                .get(idx + 1)
                .map(|(_, next_header_start, _)| *next_header_start)
                .unwrap_or(contents.len()),
        })
}

fn toml_table_headers(contents: &str) -> Vec<(&str, usize, usize)> {
    let mut sections = Vec::new();
    let mut offset = 0;

    for line in contents.split_inclusive('\n') {
        if let Some(name) = parse_toml_table_header(line) {
            sections.push((name, offset, offset + line.len()));
        }
        offset += line.len();
    }

    if offset < contents.len() {
        let line = &contents[offset..];
        if let Some(name) = parse_toml_table_header(line) {
            sections.push((name, offset, contents.len()));
        }
    }

    sections
}

fn parse_toml_table_header(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    if let Some(rest) = trimmed.strip_prefix("[[") {
        let close = rest.find("]]")?;
        return Some(rest[..close].trim());
    }
    if !trimmed.starts_with('[') {
        return None;
    }

    let close = trimmed.find(']')?;
    Some(trimmed[1..close].trim())
}

fn find_top_level_aliases_key(contents: &str) -> Option<(usize, usize)> {
    let first_table_start = toml_table_headers(contents)
        .first()
        .map(|(_, start, _)| *start)
        .unwrap_or(contents.len());
    let root = &contents[..first_table_start];
    let mut offset = 0;

    for line in root.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if !trimmed.starts_with('#') && toml_line_key_is(trimmed, "aliases") {
            return Some((offset, offset + line.len()));
        }
        offset += line.len();
    }

    if offset < root.len() {
        let line = &root[offset..];
        let trimmed = line.trim_start();
        if !trimmed.starts_with('#') && toml_line_key_is(trimmed, "aliases") {
            return Some((offset, root.len()));
        }
    }

    None
}

fn toml_line_key_is(trimmed_line: &str, expected: &str) -> bool {
    let Some((raw_key, _)) = trimmed_line.split_once('=') else {
        return false;
    };
    let raw_key = raw_key.trim();
    raw_key == expected || raw_key == format!("\"{}\"", expected).as_str()
}

fn toml_line_is_comment_or_blank(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.is_empty() || trimmed.starts_with('#')
}

fn toml_section_lines(contents: &str, section: TomlTableSection) -> Vec<(usize, usize, &str)> {
    let body = &contents[section.body_start..section.end];
    let mut offset = section.body_start;
    let mut lines = Vec::new();

    for line in body.split_inclusive('\n') {
        let start = offset;
        let end = offset + line.len();
        lines.push((start, end, line));
        offset = end;
    }

    lines
}

fn toml_section_last_entry_end(contents: &str, section: TomlTableSection) -> Option<usize> {
    toml_section_lines(contents, section)
        .into_iter()
        .filter(|(_, _, line)| !toml_line_is_comment_or_blank(line))
        .map(|(_, end, _)| end)
        .last()
}

fn aliases_section_replacement_end(contents: &str, section: TomlTableSection) -> usize {
    if let Some(end) = toml_section_last_entry_end(contents, section) {
        return end;
    }

    let lines = toml_section_lines(contents, section);
    lines
        .iter()
        .enumerate()
        .filter_map(|(idx, (start, _, line))| {
            if !line.trim().is_empty() {
                return None;
            }
            let has_following_comment = lines[idx + 1..].iter().any(|(_, _, following)| {
                let trimmed = following.trim();
                !trimmed.is_empty() && trimmed.starts_with('#')
            });
            has_following_comment.then_some(*start)
        })
        .last()
        .unwrap_or(section.end)
}

fn separator_before_preserved_suffix(suffix: &str) -> &'static str {
    if suffix.starts_with('\n') {
        "\n"
    } else {
        "\n\n"
    }
}

fn migrate_aliases_to_user_bindings_toml(
    contents: &str,
    config: &BunnylolConfig,
) -> Option<String> {
    let aliases_section = find_toml_table_section(contents, "aliases");
    let aliases_key = find_top_level_aliases_key(contents);
    if aliases_section.is_none() && aliases_key.is_none() {
        return None;
    }
    let user_bindings_section = find_toml_table_section(contents, "user_bindings");

    let mut migrated_aliases: Vec<(&String, &String)> = config
        .aliases
        .iter()
        .filter(|(name, _)| !config.user_bindings.contains_key(*name))
        .collect();
    migrated_aliases.sort_by_key(|(name, _)| name.to_lowercase());

    let migrated_entries = migrated_aliases
        .into_iter()
        .map(|(name, command)| {
            format_user_binding_toml(
                name,
                &UserBinding::Command {
                    command: command.clone(),
                    description: None,
                    override_builtin: false,
                },
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut replacements: Vec<(usize, usize, String)> = Vec::new();
    if let Some(aliases_section) = aliases_section {
        let replacement_end = aliases_section_replacement_end(contents, aliases_section);
        let preserved_suffix = &contents[replacement_end..aliases_section.end];
        let aliases_replacement = if user_bindings_section.is_none() && !migrated_entries.is_empty()
        {
            format!(
                "[user_bindings]\n{}{}",
                migrated_entries,
                separator_before_preserved_suffix(preserved_suffix)
            )
        } else {
            String::new()
        };
        replacements.push((
            aliases_section.header_start,
            replacement_end,
            aliases_replacement,
        ));
    } else if let Some((start, end)) = aliases_key {
        replacements.push((start, end, String::new()));
    }

    let needs_new_user_bindings_section = user_bindings_section.is_none()
        && aliases_section.is_none()
        && !migrated_entries.is_empty();
    if needs_new_user_bindings_section {
        let first_table_start = toml_table_headers(contents)
            .first()
            .map(|(_, start, _)| *start)
            .unwrap_or(contents.len());
        let separator = if first_table_start == 0 || contents[..first_table_start].ends_with('\n') {
            ""
        } else {
            "\n"
        };
        replacements.push((
            first_table_start,
            first_table_start,
            format!("{}[user_bindings]\n{}\n\n", separator, migrated_entries),
        ));
    } else if let Some(section) = user_bindings_section
        && !migrated_entries.is_empty()
    {
        let insertion_point =
            toml_section_last_entry_end(contents, section).unwrap_or(section.body_start);
        let preserved_suffix = &contents[insertion_point..section.end];
        let prefix = if insertion_point == section.body_start
            || contents[..insertion_point].ends_with('\n')
        {
            ""
        } else {
            "\n"
        };
        replacements.push((
            insertion_point,
            insertion_point,
            format!(
                "{}{}{}",
                prefix,
                migrated_entries,
                separator_before_preserved_suffix(preserved_suffix)
            ),
        ));
    }

    replacements.sort_by(|a, b| b.0.cmp(&a.0));
    let mut migrated = contents.to_string();
    for (start, end, replacement) in replacements {
        migrated.replace_range(start..end, &replacement);
    }
    Some(migrated)
}

fn write_config_atomically(config_path: &Path, contents: &str) -> Result<(), String> {
    let parent = config_path
        .parent()
        .ok_or_else(|| format!("Config path {:?} has no parent directory", config_path))?;
    let mut temp_file = tempfile::NamedTempFile::new_in(parent)
        .map_err(|e| format!("Failed to create temporary config file: {}", e))?;
    temp_file
        .write_all(contents.as_bytes())
        .map_err(|e| format!("Failed to write temporary config file: {}", e))?;
    temp_file
        .flush()
        .map_err(|e| format!("Failed to flush temporary config file: {}", e))?;

    temp_file
        .persist(config_path)
        .map(|_| ())
        .map_err(|e| format!("Failed to replace config file atomically: {}", e.error))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::*;

    fn write_migration_test_config(test_name: &str, contents: &str) -> (PathBuf, PathBuf) {
        let mut dir = std::env::temp_dir();
        dir.push(format!(
            "bunnylol-alias-migration-test-{}-{}-{}",
            test_name,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("config.toml");
        fs::write(&path, contents).unwrap();
        (dir, path)
    }

    #[test]
    fn test_load_from_path_migration_removes_empty_aliases_section() {
        let (dir, path) = write_migration_test_config(
            "empty-aliases",
            r#"# top-level comment
default_search = "google"

[aliases]
# old example alias

[history]
enabled = true
"#,
        );

        let result = (|| {
            let config = BunnylolConfig::load_from_path(&path).unwrap();
            assert!(config.aliases.is_empty());
            assert!(config.user_bindings.is_empty());

            let migrated = fs::read_to_string(&path).unwrap();
            assert!(migrated.contains("# top-level comment"));
            assert!(migrated.contains("[history]\nenabled = true"));
            assert!(!migrated.contains("[aliases]"));
            assert!(!migrated.contains("old example alias"));
            assert!(!migrated.contains("[user_bindings]"));
        })();

        fs::remove_dir_all(&dir).ok();
        result
    }

    #[test]
    fn test_load_from_path_migrates_aliases_preserving_non_alias_comments() {
        let (dir, path) = write_migration_test_config(
            "preserve-comments",
            r#"# top-level comment
default_search = "ddg"

[aliases]
# personal alias comment
work = "gh mycompany/repo"
blog = "gh username/blog"

[user_bindings]
# keep user binding comment
cal = { url = "https://calendar.google.com/calendar/u/1/r" }

# keep history heading comment
[history]
# keep history comment
enabled = false
max_entries = 12
"#,
        );

        let result = (|| {
            let config = BunnylolConfig::load_from_path(&path).unwrap();
            assert!(config.aliases.is_empty());
            assert!(matches!(
                config.user_bindings.get("work"),
                Some(UserBinding::Command { command, .. }) if command == "gh mycompany/repo"
            ));
            assert!(matches!(
                config.user_bindings.get("blog"),
                Some(UserBinding::Command { command, .. }) if command == "gh username/blog"
            ));
            assert!(matches!(
                config.user_bindings.get("cal"),
                Some(UserBinding::Url { .. })
            ));

            let migrated = fs::read_to_string(&path).unwrap();
            assert!(migrated.contains("# top-level comment"));
            assert!(migrated.contains("# keep user binding comment"));
            assert!(migrated.contains("# keep history heading comment"));
            assert!(migrated.contains("# keep history comment"));
            assert!(!migrated.contains("[aliases]"));
            assert!(!migrated.contains("# personal alias comment"));
            assert!(!migrated.contains("work = \"gh mycompany/repo\""));
            assert!(migrated.contains("work = { command = \"gh mycompany/repo\" }"));
            assert!(migrated.contains("blog = { command = \"gh username/blog\" }"));
            assert!(migrated.contains(
                "work = { command = \"gh mycompany/repo\" }\n\n# keep history heading comment"
            ));
        })();

        fs::remove_dir_all(&dir).ok();
        result
    }

    #[test]
    fn test_load_from_path_migration_creates_user_bindings_section() {
        let (dir, path) = write_migration_test_config(
            "create-user-bindings",
            r#"default_search = "google"

[aliases]
work = "gh mycompany/repo"

# Command history settings
[history]
enabled = false
"#,
        );

        let result = (|| {
            let config = BunnylolConfig::load_from_path(&path).unwrap();
            assert!(config.aliases.is_empty());
            assert!(matches!(
                config.user_bindings.get("work"),
                Some(UserBinding::Command { command, .. }) if command == "gh mycompany/repo"
            ));

            let migrated = fs::read_to_string(&path).unwrap();
            assert!(
                migrated.contains("[user_bindings]\nwork = { command = \"gh mycompany/repo\" }")
            );
            assert!(migrated.contains(
                "work = { command = \"gh mycompany/repo\" }\n\n# Command history settings"
            ));
            assert!(!migrated.contains("[aliases]"));
            assert!(migrated.contains("# Command history settings"));
            assert!(migrated.contains("[history]\nenabled = false"));
        })();

        fs::remove_dir_all(&dir).ok();
        result
    }

    #[test]
    fn test_load_from_path_migration_keeps_user_bindings_on_alias_conflict() {
        let (dir, path) = write_migration_test_config(
            "user-bindings-wins",
            r#"[aliases]
work = "gh from-aliases"

[user_bindings]
work = { command = "gh from-user-bindings" }
"#,
        );

        let result = (|| {
            let config = BunnylolConfig::load_from_path(&path).unwrap();
            assert!(config.aliases.is_empty());
            assert!(matches!(
                config.user_bindings.get("work"),
                Some(UserBinding::Command { command, .. }) if command == "gh from-user-bindings"
            ));

            let migrated = fs::read_to_string(&path).unwrap();
            assert!(migrated.contains("work = { command = \"gh from-user-bindings\" }"));
            assert!(!migrated.contains("from-aliases"));
        })();

        fs::remove_dir_all(&dir).ok();
        result
    }

    #[test]
    fn test_load_from_path_migration_handles_inline_aliases_without_reparenting_root_keys() {
        let (dir, path) = write_migration_test_config(
            "inline-aliases",
            r#"browser = "firefox"
aliases = { work = "gh mycompany/repo" }
default_search = "ddg"

[history]
enabled = false
"#,
        );

        let result = (|| {
            let config = BunnylolConfig::load_from_path(&path).unwrap();
            assert!(config.aliases.is_empty());
            assert_eq!(config.browser.as_deref(), Some("firefox"));
            assert_eq!(config.default_search, "ddg");
            assert!(matches!(
                config.user_bindings.get("work"),
                Some(UserBinding::Command { command, .. }) if command == "gh mycompany/repo"
            ));

            let migrated = fs::read_to_string(&path).unwrap();
            assert!(!migrated.contains("aliases ="));
            assert!(
                migrated.contains("[user_bindings]\nwork = { command = \"gh mycompany/repo\" }")
            );
            assert!(migrated.contains("default_search = \"ddg\""));
            assert!(migrated.contains("[history]\nenabled = false"));
        })();

        fs::remove_dir_all(&dir).ok();
        result
    }

    #[test]
    fn test_load_from_path_migration_escapes_alias_control_characters() {
        let (dir, path) = write_migration_test_config(
            "escaped-control-characters",
            r#"[aliases]
multi = "gh mycompany/line\nbreak"
"#,
        );

        let result = (|| {
            let config = BunnylolConfig::load_from_path(&path).unwrap();
            assert!(matches!(
                config.user_bindings.get("multi"),
                Some(UserBinding::Command { command, .. }) if command == "gh mycompany/line\nbreak"
            ));

            let migrated = fs::read_to_string(&path).unwrap();
            assert!(migrated.contains("command = \"gh mycompany/line\\nbreak\""));
            let reparsed: BunnylolConfig = toml::from_str(&migrated).unwrap();
            assert!(matches!(
                reparsed.user_bindings.get("multi"),
                Some(UserBinding::Command { command, .. }) if command == "gh mycompany/line\nbreak"
            ));
        })();

        fs::remove_dir_all(&dir).ok();
        result
    }
}
