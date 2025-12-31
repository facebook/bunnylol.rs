/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub mod bunnylol_command_registry;
pub mod commands;
pub mod config;
pub mod history;
pub mod utils;

// Server module is needed for both server runtime and CLI service management
#[cfg(any(feature = "server", feature = "cli"))]
pub mod server;

// Re-export service from server module for CLI feature
#[cfg(feature = "cli")]
pub use server::service;

pub use bunnylol_command_registry::BunnylolCommandRegistry;
pub use commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
pub use config::BunnylolConfig;
pub use history::{History, HistoryEntry};
