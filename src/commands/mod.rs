/// Command module exports
///
/// This module re-exports all the individual command implementations
/// for easy importing in the registry.
pub mod devbunny;
pub mod github;
pub mod gmail;
pub mod google;
pub mod instagram;
pub mod reddit;
pub mod rei;
pub mod twitter;
pub mod youtube;

// Re-export the command structs for convenience
pub use devbunny::DevBunnyCommand;
pub use github::GitHubCommand;
pub use gmail::GmailCommand;
pub use google::GoogleCommand;
pub use instagram::InstagramCommand;
pub use reddit::RedditCommand;
pub use rei::ReiCommand;
pub use twitter::TwitterCommand;
pub use youtube::YouTubeCommand;
