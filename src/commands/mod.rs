/// Command module exports
///
/// This module re-exports all the individual command implementations
/// for easy importing in the registry.
pub mod amazon;
pub mod bindings;
pub mod cargo;
pub mod chatgpt;
pub mod claude;
pub mod devbunny;
pub mod facebook;
pub mod github;
pub mod gmail;
pub mod google;
pub mod googlechat;
pub mod googledocs;
pub mod googlesheets;
pub mod googleslides;
pub mod hack;
pub mod instagram;
pub mod meta;
pub mod npm;
pub mod reddit;
pub mod rei;
pub mod rust;
pub mod threads;
pub mod twitter;
pub mod whatsapp;

// Re-export the command structs for convenience
pub use amazon::AmazonCommand;
pub use bindings::BindingsCommand;
pub use cargo::CargoCommand;
pub use chatgpt::ChatGPTCommand;
pub use claude::ClaudeCommand;
pub use devbunny::DevBunnyCommand;
pub use facebook::FacebookCommand;
pub use github::GitHubCommand;
pub use gmail::GmailCommand;
pub use google::GoogleSearchCommand;
pub use googlechat::GoogleChatCommand;
pub use googledocs::GoogleDocsCommand;
pub use googlesheets::GoogleSheetsCommand;
pub use googleslides::GoogleSlidesCommand;
pub use hack::HackCommand;
pub use instagram::InstagramCommand;
pub use meta::MetaCommand;
pub use npm::NpmCommand;
pub use reddit::RedditCommand;
pub use rei::REICommand;
pub use rust::RustCommand;
pub use threads::ThreadsCommand;
pub use twitter::TwitterCommand;
pub use whatsapp::WhatsAppCommand;
