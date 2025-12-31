/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use crate::config::BunnylolConfig;

/// Command history entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryEntry {
    pub command: String,
    pub timestamp: String,
    pub user: String,
}

impl HistoryEntry {
    /// Create a new history entry with current timestamp
    pub fn new(command: String, user: String) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        Self {
            command,
            timestamp,
            user,
        }
    }

    /// Parse a history entry from a line in the history file
    /// Format: timestamp|user|command
    pub fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.splitn(3, '|').collect();
        if parts.len() == 3 {
            Some(Self {
                timestamp: parts[0].to_string(),
                user: parts[1].to_string(),
                command: parts[2].to_string(),
            })
        } else {
            None
        }
    }

    /// Convert entry to a line for the history file
    /// Format: timestamp|user|command
    pub fn to_line(&self) -> String {
        format!("{}|{}|{}", self.timestamp, self.user, self.command)
    }
}

/// Command history manager
pub struct History {
    path: PathBuf,
    max_entries: usize,
}

impl History {
    /// Create a new history manager
    pub fn new(config: &BunnylolConfig) -> Option<Self> {
        let path = BunnylolConfig::get_history_path()?;
        Some(Self {
            path,
            max_entries: config.history.max_entries,
        })
    }

    /// Ensure the parent directory exists
    fn ensure_parent_dir(&self) -> Result<(), String> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create history directory: {}", e))?;
        }
        Ok(())
    }

    /// Add a command to history
    pub fn add(&self, command: &str, user: &str) -> Result<(), String> {
        if command.trim().is_empty() {
            return Ok(());
        }

        self.ensure_parent_dir()?;

        // Read existing history
        let mut entries = self.read_all()?;

        // Add new entry
        entries.push(HistoryEntry::new(command.to_string(), user.to_string()));

        // Trim to max_entries
        if entries.len() > self.max_entries {
            let skip_count = entries.len() - self.max_entries;
            entries = entries.into_iter().skip(skip_count).collect();
        }

        // Write back to file
        self.write_all(&entries)?;

        Ok(())
    }

    /// Read all history entries
    pub fn read_all(&self) -> Result<Vec<HistoryEntry>, String> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let file = fs::File::open(&self.path)
            .map_err(|e| format!("Failed to read history file: {}", e))?;

        let reader = BufReader::new(file);
        let entries: Vec<HistoryEntry> = reader
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| HistoryEntry::from_line(&line))
            .collect();

        Ok(entries)
    }

    /// Write all history entries to file
    fn write_all(&self, entries: &[HistoryEntry]) -> Result<(), String> {
        let mut file = fs::File::create(&self.path)
            .map_err(|e| format!("Failed to create history file: {}", e))?;

        for entry in entries {
            writeln!(file, "{}", entry.to_line())
                .map_err(|e| format!("Failed to write to history file: {}", e))?;
        }

        Ok(())
    }

    /// Get the last N commands from history
    pub fn get_recent(&self, n: usize) -> Result<Vec<HistoryEntry>, String> {
        let entries = self.read_all()?;
        Ok(entries.into_iter().rev().take(n).collect())
    }

    /// Clear all history
    pub fn clear(&self) -> Result<(), String> {
        if self.path.exists() {
            fs::remove_file(&self.path).map_err(|e| format!("Failed to clear history: {}", e))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_entry_new() {
        let entry = HistoryEntry::new("ig reels".to_string(), "testuser".to_string());
        assert_eq!(entry.command, "ig reels");
        assert_eq!(entry.user, "testuser");
        assert!(!entry.timestamp.is_empty());
    }

    #[test]
    fn test_history_entry_from_line() {
        let line = "1234567890|testuser|gh facebook/react";
        let entry = HistoryEntry::from_line(line).unwrap();
        assert_eq!(entry.timestamp, "1234567890");
        assert_eq!(entry.user, "testuser");
        assert_eq!(entry.command, "gh facebook/react");
    }

    #[test]
    fn test_history_entry_to_line() {
        let entry = HistoryEntry {
            timestamp: "1234567890".to_string(),
            user: "testuser".to_string(),
            command: "ig reels".to_string(),
        };
        assert_eq!(entry.to_line(), "1234567890|testuser|ig reels");
    }

    #[test]
    fn test_history_entry_from_line_invalid() {
        let line = "invalid";
        assert!(HistoryEntry::from_line(line).is_none());
    }

    #[test]
    fn test_history_entry_roundtrip() {
        let original = HistoryEntry {
            timestamp: "1234567890".to_string(),
            user: "testuser".to_string(),
            command: "test command".to_string(),
        };
        let line = original.to_line();
        let parsed = HistoryEntry::from_line(&line).unwrap();
        assert_eq!(original, parsed);
    }
}
