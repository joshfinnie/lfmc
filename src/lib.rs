pub mod api;
pub mod bluesky;
pub mod cli;
pub mod config;
pub mod error;
pub mod formatters;

// Re-export main types for convenience
pub use api::client::LastFmClient;
pub use api::models::{Artist, TopArtists};
pub use cli::Cli;
pub use config::{Config, Period};
pub use error::{Error, Result};
pub use formatters::{Format, Formatter};
