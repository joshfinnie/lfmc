use crate::error::{Error, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const BSKY_HOST: &str = "https://bsky.social";

#[derive(Serialize)]
struct CreateSessionRequest<'a> {
    identifier: &'a str,
    password: &'a str,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateSessionResponse {
    access_jwt: String,
    did: String,
}

#[derive(Serialize)]
struct PostRecord<'a> {
    #[serde(rename = "$type")]
    record_type: &'a str,
    text: &'a str,
    #[serde(rename = "createdAt")]
    created_at: String,
}

#[derive(Serialize)]
struct CreateRecordRequest<'a> {
    repo: &'a str,
    collection: &'a str,
    record: PostRecord<'a>,
}

pub struct BlueskyClient {
    client: Client,
}

impl BlueskyClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn post(&self, handle: &str, app_password: &str, text: &str) -> Result<()> {
        let char_count = text.chars().count();
        if char_count > 300 {
            return Err(Error::BlueskyError(format!(
                "Post is {} characters, exceeding Bluesky's 300 character limit. Try reducing --limit.",
                char_count
            )));
        }

        let session = self.authenticate(handle, app_password).await?;
        self.create_post(&session.access_jwt, &session.did, text)
            .await
    }

    async fn authenticate(&self, handle: &str, password: &str) -> Result<CreateSessionResponse> {
        let url = format!("{}/xrpc/com.atproto.server.createSession", BSKY_HOST);
        let body = CreateSessionRequest {
            identifier: handle,
            password,
        };

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::BlueskyError(format!("Authentication request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(Error::BlueskyError(format!(
                "Authentication failed ({}): {}",
                status, text
            )));
        }

        response.json::<CreateSessionResponse>().await.map_err(|e| {
            Error::BlueskyError(format!("Failed to parse authentication response: {}", e))
        })
    }

    async fn create_post(&self, access_jwt: &str, did: &str, text: &str) -> Result<()> {
        let url = format!("{}/xrpc/com.atproto.repo.createRecord", BSKY_HOST);
        let created_at = chrono::Utc::now().to_rfc3339();

        let body = CreateRecordRequest {
            repo: did,
            collection: "app.bsky.feed.post",
            record: PostRecord {
                record_type: "app.bsky.feed.post",
                text,
                created_at,
            },
        };

        let response = self
            .client
            .post(&url)
            .bearer_auth(access_jwt)
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::BlueskyError(format!("Post request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(Error::BlueskyError(format!(
                "Failed to create post ({}): {}",
                status, text
            )));
        }

        Ok(())
    }
}

impl Default for BlueskyClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_post_rejects_text_over_300_chars() {
        let client = BlueskyClient::new();
        let long_text = "a".repeat(301);
        let result = client.post("handle", "password", &long_text).await;
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("301"), "error should include character count: {}", msg);
        assert!(msg.contains("300"), "error should mention the limit: {}", msg);
    }

    #[tokio::test]
    async fn test_post_accepts_text_at_300_chars() {
        // 300 chars should pass the local validation (will then fail on auth, not char count)
        let client = BlueskyClient::new();
        let text = "a".repeat(300);
        let result = client.post("handle", "password", &text).await;
        // Should fail on network/auth, not on character count
        if let Err(e) = result {
            assert!(
                !e.to_string().contains("300 character limit"),
                "should not fail with char limit error: {}",
                e
            );
        }
    }
}
