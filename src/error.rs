use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("API request failed: {0}")]
    ApiError(#[from] reqwest::Error),

    #[error("API request middleware error: {0}")]
    MiddlewareError(#[from] reqwest_middleware::Error),

    #[error("Failed to parse API response: {0}")]
    ParseError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error(
        "Invalid period: {0}. Valid periods are: overall, 7day, 1month, 3month, 6month, 12month"
    )]
    InvalidPeriod(String),

    #[error("Formatting error: {0}")]
    FormatError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

impl Error {
    pub fn parse_error<T: std::fmt::Display>(msg: T) -> Self {
        Error::ParseError(msg.to_string())
    }

    pub fn config_error<T: std::fmt::Display>(msg: T) -> Self {
        Error::ConfigError(msg.to_string())
    }

    pub fn format_error<T: std::fmt::Display>(msg: T) -> Self {
        Error::FormatError(msg.to_string())
    }
}
