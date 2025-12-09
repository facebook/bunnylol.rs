/// Command module exports
///
/// This module re-exports all the individual command implementations
/// for easy importing in the registry.
pub mod bindings;
pub mod cargo;
pub mod chatgpt;
pub mod claude;
pub mod devbunny;
pub mod facebook;
pub mod github;
pub mod gmail;
pub mod google;
pub mod instagram;
pub mod meta;
pub mod npm;
pub mod reddit;
pub mod rei;
pub mod threads;
pub mod twitter;
pub mod whatsapp;

// Re-export the command structs for convenience
pub use bindings::BindingsCommand;
pub use cargo::CargoCommand;
pub use chatgpt::ChatGPTCommand;
pub use claude::ClaudeCommand;
pub use devbunny::DevBunnyCommand;
pub use facebook::FacebookCommand;
pub use github::GitHubCommand;
pub use gmail::GmailCommand;
pub use google::GoogleCommand;
pub use instagram::InstagramCommand;
pub use meta::MetaCommand;
pub use npm::NpmCommand;
pub use reddit::RedditCommand;
pub use rei::REICommand;
pub use threads::ThreadsCommand;
pub use twitter::TwitterCommand;
pub use whatsapp::WhatsAppCommand;
