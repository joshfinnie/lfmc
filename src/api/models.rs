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
