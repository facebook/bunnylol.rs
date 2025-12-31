/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::{BunnylolConfig, History, utils};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CommandStats {
    pub command: String,
    pub count: usize,
    pub percentage: f64,
}

#[derive(Debug, Clone)]
pub struct UsageStats {
    pub total_commands: usize,
    pub unique_commands: usize,
    pub top_commands: Vec<CommandStats>,
    pub least_used_commands: Vec<CommandStats>,
    pub all_commands: Vec<CommandStats>,
}

impl UsageStats {
    /// Generate usage statistics from history
    pub fn from_history(config: &BunnylolConfig) -> Option<Self> {
        let history = History::new(config)?;
        let entries = history.read_all().ok()?;

        if entries.is_empty() {
            return Some(UsageStats {
                total_commands: 0,
                unique_commands: 0,
                top_commands: vec![],
                least_used_commands: vec![],
                all_commands: vec![],
            });
        }

        // Count command usage
        let mut command_counts: HashMap<String, usize> = HashMap::new();
        for entry in &entries {
            let command = utils::get_command_from_query_string(&entry.command);
            *command_counts.entry(command.to_string()).or_insert(0) += 1;
        }

        let total_commands = entries.len();
        let unique_commands = command_counts.len();

        // Convert to CommandStats
        let mut all_commands: Vec<CommandStats> = command_counts
            .into_iter()
            .map(|(command, count)| CommandStats {
                command,
                count,
                percentage: (count as f64 / total_commands as f64) * 100.0,
            })
            .collect();

        // Sort by count (descending)
        all_commands.sort_by(|a, b| b.count.cmp(&a.count));

        // Get top 10
        let top_commands = all_commands.iter().take(10).cloned().collect();

        // Get least used (bottom 10, but reversed to show least first)
        let mut least_used_commands: Vec<CommandStats> =
            all_commands.iter().rev().take(10).cloned().collect();
        least_used_commands.reverse();

        Some(UsageStats {
            total_commands,
            unique_commands,
            top_commands,
            least_used_commands,
            all_commands,
        })
    }
}
