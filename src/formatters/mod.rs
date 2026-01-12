pub mod csv;
pub mod json;
pub mod social;

use crate::api::models::TopArtists;
use crate::config::Config;
use crate::error::Result;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum Format {
    /// Social media format (default)
    #[default]
    Social,
    /// JSON format
    Json,
    /// CSV format
    Csv,
}

pub trait Formatter {
    fn format(&self, artists: &TopArtists, config: &Config) -> Result<String>;
}

pub fn get_formatter(format: Format) -> Box<dyn Formatter> {
    match format {
        Format::Social => Box::new(social::SocialFormatter),
        Format::Json => Box::new(json::JsonFormatter),
        Format::Csv => Box::new(csv::CsvFormatter),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::models::Artist;
    use crate::config::Period;

    fn create_test_data() -> (TopArtists, Config) {
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

        (artists, config)
    }

    #[test]
    fn test_all_formatters_produce_output() {
        let (artists, config) = create_test_data();

        for format in [Format::Social, Format::Json, Format::Csv] {
            let formatter = get_formatter(format);
            let result = formatter.format(&artists, &config);
            assert!(result.is_ok(), "Format {:?} should produce output", format);
            assert!(!result.unwrap().is_empty());
        }
    }
}
