use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Artist {
    pub name: String,
    pub playcount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopArtists {
    #[serde(rename = "artist")]
    pub artists: Vec<Artist>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "topartists")]
    pub top_artists: TopArtists,
}

/// Last.fm's error envelope, e.g. `{"error": 6, "message": "User not found"}`.
///
/// Returned both with 4xx statuses and, for some failures, alongside HTTP 200.
#[derive(Debug, Clone, Deserialize)]
pub struct ApiError {
    pub error: u32,
    pub message: String,
}

impl ApiError {
    /// Adds actionable context to the codes a user of this tool can actually hit.
    /// See https://www.last.fm/api/errorcodes
    pub fn hint(&self) -> Option<&'static str> {
        match self.error {
            6 => Some("Check the --username value; it must be an existing Last.fm account."),
            10 | 26 => Some("Check the --api-key value at https://www.last.fm/api/accounts"),
            17 => {
                Some("This user's recent listening is private in their Last.fm privacy settings.")
            }
            29 => Some("Rate limit exceeded — wait a moment before retrying."),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_artist() {
        let json = r#"{"name": "Test Artist", "playcount": "100"}"#;
        let artist: Artist = serde_json::from_str(json).unwrap();
        assert_eq!(artist.name, "Test Artist");
        assert_eq!(artist.playcount, "100");
    }

    #[test]
    fn test_deserialize_top_artists() {
        let json = r#"{
            "artist": [
                {"name": "Artist 1", "playcount": "100"},
                {"name": "Artist 2", "playcount": "50"}
            ]
        }"#;
        let top_artists: TopArtists = serde_json::from_str(json).unwrap();
        assert_eq!(top_artists.artists.len(), 2);
        assert_eq!(top_artists.artists[0].name, "Artist 1");
        assert_eq!(top_artists.artists[1].playcount, "50");
    }

    #[test]
    fn test_deserialize_api_response() {
        let json = r#"{
            "topartists": {
                "artist": [
                    {"name": "Test Artist", "playcount": "100"}
                ]
            }
        }"#;
        let response: ApiResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.top_artists.artists.len(), 1);
        assert_eq!(response.top_artists.artists[0].name, "Test Artist");
    }
}
