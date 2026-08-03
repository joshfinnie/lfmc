use crate::api::models::{ApiError, ApiResponse};
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
    base_url: String,
}

impl LastFmClient {
    pub fn new() -> Self {
        Self::with_base_url(BASE_URL)
    }

    /// Builds a client against an alternate base URL. Used by tests to point at
    /// a mock server; production code should use [`LastFmClient::new`].
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        let retry_policy = ExponentialBackoff::builder()
            .retry_bounds(
                std::time::Duration::from_secs(1),
                std::time::Duration::from_secs(60),
            )
            .build_with_max_retries(3);

        let client = ClientBuilder::new(Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Self {
            client,
            base_url: base_url.into(),
        }
    }

    #[instrument(skip(self, config), fields(username = %config.username, period = %config.period))]
    pub async fn get_top_artists(&self, config: &Config) -> Result<ApiResponse> {
        let encoded_username = urlencoding::encode(&config.username);

        // Note: api_key is included as a query parameter per the Last.fm API spec.
        // Avoid enabling reqwest debug logging (RUST_LOG=reqwest=debug) in production
        // as it will log the full URL including the key.
        let url = format!(
            "{}?method={}&user={}&api_key={}&format=json&period={}&limit={}",
            self.base_url,
            API_METHOD,
            encoded_username,
            config.api_key,
            config.period.api_string(),
            config.limit
        );

        info!("Fetching top artists from Last.fm API");

        // Errors are scrubbed of their URL throughout: it carries the API key.
        let response = self.client.get(&url).send().await.map_err(scrub_url)?;

        let status = response.status();
        let body = response.text().await.map_err(|e| scrub_url(e.into()))?;

        // Last.fm reports failures as a JSON envelope, both on 4xx and
        // occasionally alongside a 200. Prefer its message over the bare status.
        if let Ok(api_error) = serde_json::from_str::<ApiError>(&body) {
            let mut msg = format!(
                "Last.fm API error {} ({}): {}",
                api_error.error, status, api_error.message
            );
            if let Some(hint) = api_error.hint() {
                msg.push_str("\n  ");
                msg.push_str(hint);
            }
            return Err(Error::ApiResponseError(msg));
        }

        if !status.is_success() {
            return Err(Error::ApiResponseError(format!(
                "Last.fm returned {} with an unrecognized body: {}",
                status,
                truncate(&body, 200)
            )));
        }

        let api_response = serde_json::from_str::<ApiResponse>(&body).map_err(|e| {
            Error::parse_error(format!(
                "Failed to parse API response: {} (body: {})",
                e,
                truncate(&body, 200)
            ))
        })?;

        info!(
            "Successfully fetched {} artists",
            api_response.top_artists.artists.len()
        );

        Ok(api_response)
    }
}

/// Strips the request URL from an error before it is ever displayed —
/// the Last.fm URL embeds the API key as a query parameter.
fn scrub_url(err: reqwest_middleware::Error) -> Error {
    match err {
        reqwest_middleware::Error::Reqwest(e) => Error::from(e.without_url()),
        reqwest_middleware::Error::Middleware(e) => {
            Error::ApiResponseError(format!("Request failed: {}", e))
        }
    }
}

fn truncate(s: &str, max: usize) -> String {
    let trimmed = s.trim();
    if trimmed.chars().count() <= max {
        return trimmed.to_string();
    }
    let head: String = trimmed.chars().take(max).collect();
    format!("{}…", head)
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

    fn test_config() -> Config {
        Config::builder()
            .api_key("secret_key_do_not_leak")
            .username("testuser")
            .limit(5)
            .period(Period::SevenDay)
            .build()
            .unwrap()
    }

    async fn mock_responding_with(status: u16, body: &str) -> (wiremock::MockServer, Config) {
        use wiremock::matchers::method;
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(status).set_body_raw(body.to_string(), "application/json"),
            )
            .mount(&server)
            .await;

        (server, test_config())
    }

    #[tokio::test]
    async fn test_invalid_api_key_surfaces_lastfm_message() {
        let body = r#"{"message":"Invalid API key - You must be granted a valid key by last.fm","error":10}"#;
        let (server, config) = mock_responding_with(403, body).await;

        let client = LastFmClient::with_base_url(server.uri());
        let err = client.get_top_artists(&config).await.unwrap_err();
        let msg = err.to_string();

        assert!(msg.contains("Invalid API key"), "got: {}", msg);
        assert!(msg.contains("error 10"), "should include the code: {}", msg);
        assert!(
            msg.contains("last.fm/api/accounts"),
            "should include a hint: {}",
            msg
        );
    }

    #[tokio::test]
    async fn test_user_not_found_surfaces_lastfm_message() {
        let body = r#"{"message":"User not found","error":6}"#;
        let (server, config) = mock_responding_with(404, body).await;

        let client = LastFmClient::with_base_url(server.uri());
        let err = client.get_top_artists(&config).await.unwrap_err();
        let msg = err.to_string();

        assert!(msg.contains("User not found"), "got: {}", msg);
        assert!(msg.contains("--username"), "should include a hint: {}", msg);
    }

    #[tokio::test]
    async fn test_error_envelope_on_http_200_is_still_an_error() {
        // Last.fm sometimes returns its error envelope with a 200 status; that
        // must not be parsed as a successful (empty) result.
        let body = r#"{"message":"Rate limit exceeded","error":29}"#;
        let (server, config) = mock_responding_with(200, body).await;

        let client = LastFmClient::with_base_url(server.uri());
        let err = client.get_top_artists(&config).await.unwrap_err();
        assert!(err.to_string().contains("Rate limit exceeded"));
    }

    #[tokio::test]
    async fn test_errors_never_leak_the_api_key() {
        // The request URL embeds the API key, so no error may render it.
        for (status, body) in [
            (403, r#"{"message":"Invalid API key","error":10}"#),
            (500, "upstream exploded"),
            (200, "not json at all"),
        ] {
            let (server, config) = mock_responding_with(status, body).await;
            let client = LastFmClient::with_base_url(server.uri());
            let err = client.get_top_artists(&config).await.unwrap_err();

            assert!(
                !err.to_string().contains("secret_key_do_not_leak"),
                "status {} leaked the API key: {}",
                status,
                err
            );
        }
    }

    #[tokio::test]
    async fn test_unparseable_body_reports_a_snippet() {
        let (server, config) = mock_responding_with(200, "<html>maintenance</html>").await;

        let client = LastFmClient::with_base_url(server.uri());
        let err = client.get_top_artists(&config).await.unwrap_err();
        let msg = err.to_string();

        assert!(msg.contains("Failed to parse"), "got: {}", msg);
        assert!(msg.contains("maintenance"), "should show the body: {}", msg);
    }

    #[tokio::test]
    async fn test_empty_artist_list_parses_as_success() {
        // A transient empty chart is valid data, not an error.
        let body = r#"{"topartists":{"artist":[],"@attr":{"total":"0"}}}"#;
        let (server, config) = mock_responding_with(200, body).await;

        let client = LastFmClient::with_base_url(server.uri());
        let response = client.get_top_artists(&config).await.unwrap();
        assert!(response.top_artists.artists.is_empty());
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
