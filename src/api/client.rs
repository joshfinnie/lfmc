use crate::api::models::ApiResponse;
use crate::config::Config;
use crate::error::{Error, Result};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use tracing::{info, instrument};

const BASE_URL: &str = "https://ws.audioscrobbler.com/2.0/";
const API_METHOD: &str = "user.gettopartists";

pub struct LastFmClient {
    client: ClientWithMiddleware,
}

impl LastFmClient {
    pub fn new() -> Self {
        let retry_policy = ExponentialBackoff::builder()
            .retry_bounds(
                std::time::Duration::from_secs(1),
                std::time::Duration::from_secs(60),
            )
            .build_with_max_retries(3);

        let client = ClientBuilder::new(Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Self { client }
    }

    #[instrument(skip(self, config), fields(username = %config.username, period = %config.period))]
    pub async fn get_top_artists(&self, config: &Config) -> Result<ApiResponse> {
        let encoded_username = urlencoding::encode(&config.username);

        // Note: api_key is included as a query parameter per the Last.fm API spec.
        // Avoid enabling reqwest debug logging (RUST_LOG=reqwest=debug) in production
        // as it will log the full URL including the key.
        let url = format!(
            "{}?method={}&user={}&api_key={}&format=json&period={}&limit={}",
            BASE_URL,
            API_METHOD,
            encoded_username,
            config.api_key,
            config.period.api_string(),
            config.limit
        );

        info!("Fetching top artists from Last.fm API");

        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()
            .map_err(|e| {
                if e.status().is_some_and(|s| s.as_u16() == 404) {
                    Error::config_error(format!("User '{}' not found", config.username))
                } else if e.status().is_some_and(|s| s.as_u16() == 403) {
                    Error::config_error("Invalid API key")
                } else {
                    Error::from(e)
                }
            })?;

        let api_response = response
            .json::<ApiResponse>()
            .await
            .map_err(|e| Error::parse_error(format!("Failed to parse API response: {}", e)))?;

        info!(
            "Successfully fetched {} artists",
            api_response.top_artists.artists.len()
        );

        Ok(api_response)
    }
}

impl Default for LastFmClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Period;

    #[test]
    fn test_client_creation() {
        let client = LastFmClient::new();
        assert!(client.client.get(BASE_URL).build().is_ok());
    }

    #[test]
    fn test_url_encoding() {
        let username_with_space = "test user";
        let encoded = urlencoding::encode(username_with_space);
        assert_eq!(encoded, "test%20user");
    }

    #[test]
    fn test_base_url_is_https() {
        assert!(BASE_URL.starts_with("https://"));
    }

    #[tokio::test]
    #[ignore = "requires network access"]
    async fn test_client_with_invalid_key() {
        let client = LastFmClient::new();
        let config = Config::builder()
            .api_key("invalid_key")
            .username("testuser")
            .limit(5)
            .period(Period::SevenDay)
            .build()
            .unwrap();

        let result = client.get_top_artists(&config).await;
        assert!(result.is_err());
    }
}
