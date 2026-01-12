use lfmc::api::models::{ApiResponse, Artist, TopArtists};
use lfmc::config::Period;
use lfmc::formatters::{get_formatter, Format};
use lfmc::{Config, LastFmClient};

#[tokio::test]
async fn test_config_validation() {
    // Test empty API key
    let result = Config::new("".to_string(), "testuser".to_string(), 5, Period::SevenDay);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("API key"));

    // Test empty username
    let result = Config::new("test_key".to_string(), "".to_string(), 5, Period::SevenDay);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Username"));

    // Test zero limit
    let result = Config::new(
        "test_key".to_string(),
        "testuser".to_string(),
        0,
        Period::SevenDay,
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Limit"));
}

#[test]
fn test_formatters_integration() {
    let artists = TopArtists {
        artists: vec![
            Artist {
                name: "Artist 1".to_string(),
                playcount: "100".to_string(),
            },
            Artist {
                name: "Artist 2".to_string(),
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

    // Test all formats
    for format in [Format::Social, Format::Json, Format::Csv] {
        let formatter = get_formatter(format);
        let result = formatter.format(&artists, &config);
        assert!(result.is_ok(), "Format {:?} failed", format);

        let output = result.unwrap();
        assert!(
            !output.is_empty(),
            "Format {:?} produced empty output",
            format
        );

        // Verify each format contains expected data
        match format {
            Format::Social => {
                assert!(output.contains("Artist 1"));
                assert!(output.contains("100"));
                assert!(output.contains("#LastFM"));
            }
            Format::Json => {
                assert!(output.contains("\"name\""));
                assert!(output.contains("\"playcount\""));
            }
            Format::Csv => {
                assert!(output.contains("artist,playcount"));
                assert!(output.contains("Artist 1,100"));
            }
        }
    }
}

#[test]
fn test_client_creation() {
    // Just verify the client can be created
    let client = LastFmClient::new();
    // If this compiles and runs, the client is properly configured
    drop(client);
}

#[test]
fn test_period_api_strings() {
    assert_eq!(Period::Overall.api_string(), "overall");
    assert_eq!(Period::SevenDay.api_string(), "7day");
    assert_eq!(Period::OneMonth.api_string(), "1month");
    assert_eq!(Period::ThreeMonth.api_string(), "3month");
    assert_eq!(Period::SixMonth.api_string(), "6month");
    assert_eq!(Period::TwelveMonth.api_string(), "12month");
}
