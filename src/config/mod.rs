pub mod period;

pub use period::Period;

use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub username: String,
    pub limit: u16,
    pub period: Period,
}

impl Config {
    pub fn new(api_key: String, username: String, limit: u16, period: Period) -> Result<Self> {
        if api_key.is_empty() {
            return Err(Error::config_error(
                "API key cannot be empty. Set via --api-key or API_KEY env var",
            ));
        }

        if username.is_empty() {
            return Err(Error::config_error(
                "Username cannot be empty. Set via --username or USERNAME env var",
            ));
        }

        if limit == 0 {
            return Err(Error::config_error("Limit must be greater than 0"));
        }

        Ok(Config {
            api_key,
            username,
            limit,
            period,
        })
    }

    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct ConfigBuilder {
    api_key: Option<String>,
    username: Option<String>,
    limit: Option<u16>,
    period: Option<Period>,
}

impl ConfigBuilder {
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn period(mut self, period: Period) -> Self {
        self.period = Some(period);
        self
    }

    pub fn build(self) -> Result<Config> {
        Config::new(
            self.api_key.unwrap_or_default(),
            self.username.unwrap_or_default(),
            self.limit.unwrap_or(5),
            self.period.unwrap_or_default(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_config() {
        let config = Config::new(
            "test_key".to_string(),
            "test_user".to_string(),
            5,
            Period::SevenDay,
        );
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.api_key, "test_key");
        assert_eq!(config.username, "test_user");
        assert_eq!(config.limit, 5);
    }

    #[test]
    fn test_empty_api_key() {
        let config = Config::new("".to_string(), "test_user".to_string(), 5, Period::SevenDay);
        assert!(config.is_err());
        let err = config.unwrap_err();
        assert!(format!("{}", err).contains("API key cannot be empty"));
    }

    #[test]
    fn test_empty_username() {
        let config = Config::new("test_key".to_string(), "".to_string(), 5, Period::SevenDay);
        assert!(config.is_err());
        let err = config.unwrap_err();
        assert!(format!("{}", err).contains("Username cannot be empty"));
    }

    #[test]
    fn test_zero_limit() {
        let config = Config::new(
            "test_key".to_string(),
            "test_user".to_string(),
            0,
            Period::SevenDay,
        );
        assert!(config.is_err());
        let err = config.unwrap_err();
        assert!(format!("{}", err).contains("Limit must be greater than 0"));
    }

    #[test]
    fn test_builder_pattern() {
        let config = Config::builder()
            .api_key("test_key")
            .username("test_user")
            .limit(10)
            .period(Period::OneMonth)
            .build();
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.api_key, "test_key");
        assert_eq!(config.username, "test_user");
        assert_eq!(config.limit, 10);
    }

    #[test]
    fn test_builder_defaults() {
        let config = Config::builder()
            .api_key("test_key")
            .username("test_user")
            .build();
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.limit, 5);
        assert_eq!(config.period.api_string(), "7day");
    }
}
