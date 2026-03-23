use crate::api::models::TopArtists;
use crate::config::Config;
use crate::error::Result;
use crate::formatters::Formatter;

pub struct SocialFormatter;

impl Formatter for SocialFormatter {
    fn format(&self, artists: &TopArtists, config: &Config) -> Result<String> {
        let period = config.period.display_suffix();

        let artist_list = &artists.artists;
        let mut output = format!(
            "♫ My Top {} played artists in the past{}:",
            artist_list.len(),
            period
        );
        for (i, artist) in artist_list.iter().enumerate() {
            let ending = if artist_list.len() < 2 {
                ""
            } else if i < artist_list.len() - 2 {
                ","
            } else if i == artist_list.len() - 2 {
                ", &"
            } else {
                ""
            };

            output = format!(
                "{} {} ({}){}",
                output, artist.name, artist.playcount, ending
            );
        }

        Ok(format!("{}. Via #LastFM ♫", output))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::models::Artist;
    use crate::config::Period;

    #[test]
    fn test_social_format_with_five_artists() {
        let artists = TopArtists {
            artists: vec![
                Artist {
                    name: "Artist 1".to_string(),
                    playcount: "100".to_string(),
                },
                Artist {
                    name: "Artist 2".to_string(),
                    playcount: "90".to_string(),
                },
                Artist {
                    name: "Artist 3".to_string(),
                    playcount: "80".to_string(),
                },
                Artist {
                    name: "Artist 4".to_string(),
                    playcount: "70".to_string(),
                },
                Artist {
                    name: "Artist 5".to_string(),
                    playcount: "60".to_string(),
                },
            ],
        };

        let config = Config::builder()
            .api_key("test_key")
            .username("test_user")
            .limit(5)
            .period(Period::SevenDay)
            .build()
            .unwrap();

        let formatter = SocialFormatter;
        let result = formatter.format(&artists, &config).unwrap();

        assert!(result.contains("My Top 5 played artists"));
        assert!(result.contains("past week"));
        assert!(result.contains("Artist 1 (100)"));
        assert!(result.contains("Artist 4 (70), &"));
        assert!(result.contains("Artist 5 (60)"));
        assert!(result.contains("Via #LastFM ♫"));
        assert!(result.contains(", &")); // Check ampersand before last item
    }

    #[test]
    fn test_social_format_with_two_artists() {
        let artists = TopArtists {
            artists: vec![
                Artist {
                    name: "First".to_string(),
                    playcount: "100".to_string(),
                },
                Artist {
                    name: "Second".to_string(),
                    playcount: "50".to_string(),
                },
            ],
        };

        let config = Config::builder()
            .api_key("test_key")
            .username("test_user")
            .limit(2)
            .period(Period::OneMonth)
            .build()
            .unwrap();

        let formatter = SocialFormatter;
        let result = formatter.format(&artists, &config).unwrap();

        assert!(result.contains("My Top 2 played artists"));
        assert!(result.contains("past month"));
        // With only 2 artists, first should have no ending, second should have "&" before it
        assert!(result.contains("First (100), &"));
        assert!(result.contains("Second (50)"));
    }

    #[test]
    fn test_social_format_single_artist() {
        let artists = TopArtists {
            artists: vec![Artist {
                name: "Only Artist".to_string(),
                playcount: "500".to_string(),
            }],
        };

        let config = Config::builder()
            .api_key("test_key")
            .username("test_user")
            .limit(1)
            .period(Period::SevenDay)
            .build()
            .unwrap();

        let formatter = SocialFormatter;
        let result = formatter.format(&artists, &config).unwrap();

        assert!(result.contains("My Top 1 played artists"));
        assert!(result.contains("Only Artist (500)"));
        assert!(!result.contains(","));
        assert!(!result.contains("&"));
    }

    #[test]
    fn test_social_count_uses_actual_artist_count() {
        // Regression test: count in header should reflect actual artists returned,
        // not config.limit (they can differ if the API returns fewer than requested).
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
            .limit(10) // requested 10, but only 2 returned
            .period(Period::SevenDay)
            .build()
            .unwrap();

        let formatter = SocialFormatter;
        let result = formatter.format(&artists, &config).unwrap();

        assert!(result.contains("My Top 2 played artists"), "expected '2', got: {}", result);
        assert!(!result.contains("My Top 10"));
    }

    #[test]
    fn test_social_format_empty_artists() {
        let artists = TopArtists { artists: vec![] };

        let config = Config::builder()
            .api_key("test_key")
            .username("test_user")
            .limit(5)
            .period(Period::SevenDay)
            .build()
            .unwrap();

        let formatter = SocialFormatter;
        let result = formatter.format(&artists, &config).unwrap();

        assert!(result.contains("My Top 0 played artists"));
        assert!(result.contains("Via #LastFM ♫"));
    }

    #[test]
    fn test_social_format_overall_period() {
        let artists = TopArtists {
            artists: vec![Artist {
                name: "Test".to_string(),
                playcount: "1000".to_string(),
            }],
        };

        let config = Config::builder()
            .api_key("test_key")
            .username("test_user")
            .limit(1)
            .period(Period::Overall)
            .build()
            .unwrap();

        let formatter = SocialFormatter;
        let result = formatter.format(&artists, &config).unwrap();

        // Overall period should have no suffix
        assert!(result.contains("My Top 1 played artists in the past:"));
        assert!(!result.contains("week"));
        assert!(!result.contains("month"));
    }
}
