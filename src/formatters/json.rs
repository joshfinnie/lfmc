use crate::api::models::TopArtists;
use crate::config::Config;
use crate::error::{Error, Result};
use crate::formatters::Formatter;

pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format(&self, artists: &TopArtists, _config: &Config) -> Result<String> {
        serde_json::to_string_pretty(artists)
            .map_err(|e| Error::format_error(format!("Failed to serialize to JSON: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::models::Artist;
    use crate::config::Period;

    #[test]
    fn test_json_format() {
        let artists = TopArtists {
            artists: vec![
                Artist {
                    name: "Test Artist 1".to_string(),
                    playcount: "100".to_string(),
                },
                Artist {
                    name: "Test Artist 2".to_string(),
                    playcount: "50".to_string(),
                },
            ],
        };

        let config = Config::builder()
            .api_key("test_key")
            .username("test_user")
            .limit(2)
            .period(Period::SevenDay)
            .build()
            .unwrap();

        let formatter = JsonFormatter;
        let result = formatter.format(&artists, &config).unwrap();

        // Verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert!(parsed["artist"].is_array());
        assert_eq!(parsed["artist"].as_array().unwrap().len(), 2);
        assert_eq!(parsed["artist"][0]["name"], "Test Artist 1");
        assert_eq!(parsed["artist"][0]["playcount"], "100");
    }

    #[test]
    fn test_json_format_empty() {
        let artists = TopArtists { artists: vec![] };

        let config = Config::builder()
            .api_key("test_key")
            .username("test_user")
            .limit(5)
            .period(Period::SevenDay)
            .build()
            .unwrap();

        let formatter = JsonFormatter;
        let result = formatter.format(&artists, &config).unwrap();

        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert!(parsed["artist"].is_array());
        assert_eq!(parsed["artist"].as_array().unwrap().len(), 0);
    }
}
