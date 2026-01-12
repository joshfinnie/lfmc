use crate::api::models::TopArtists;
use crate::config::Config;
use crate::error::{Error, Result};
use crate::formatters::Formatter;
use csv::Writer;

pub struct CsvFormatter;

impl Formatter for CsvFormatter {
    fn format(&self, artists: &TopArtists, _config: &Config) -> Result<String> {
        let mut wtr = Writer::from_writer(vec![]);

        // Write header
        wtr.write_record(["artist", "playcount"])
            .map_err(|e| Error::format_error(format!("Failed to write CSV header: {}", e)))?;

        // Write artists
        for artist in &artists.artists {
            wtr.write_record([&artist.name, &artist.playcount])
                .map_err(|e| Error::format_error(format!("Failed to write CSV record: {}", e)))?;
        }

        wtr.flush()
            .map_err(|e| Error::format_error(format!("Failed to flush CSV writer: {}", e)))?;

        let data = wtr
            .into_inner()
            .map_err(|e| Error::format_error(format!("Failed to get CSV data: {}", e)))?;

        String::from_utf8(data)
            .map_err(|e| Error::format_error(format!("Failed to convert CSV to string: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::models::Artist;
    use crate::config::Period;

    #[test]
    fn test_csv_format() {
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

        let formatter = CsvFormatter;
        let result = formatter.format(&artists, &config).unwrap();

        // Check header
        assert!(result.contains("artist,playcount"));
        // Check data
        assert!(result.contains("Test Artist 1,100"));
        assert!(result.contains("Test Artist 2,50"));

        // Verify it's valid CSV by parsing it back
        let mut rdr = csv::Reader::from_reader(result.as_bytes());
        let records: Vec<_> = rdr.records().collect();
        assert_eq!(records.len(), 2);
    }

    #[test]
    fn test_csv_format_with_commas_in_name() {
        let artists = TopArtists {
            artists: vec![Artist {
                name: "Test, Artist".to_string(),
                playcount: "100".to_string(),
            }],
        };

        let config = Config::builder()
            .api_key("test_key")
            .username("test_user")
            .limit(1)
            .period(Period::SevenDay)
            .build()
            .unwrap();

        let formatter = CsvFormatter;
        let result = formatter.format(&artists, &config).unwrap();

        // CSV should properly escape the comma
        assert!(result.contains("\"Test, Artist\""));

        // Verify parsing works correctly
        let mut rdr = csv::Reader::from_reader(result.as_bytes());
        let record = rdr.records().next().unwrap().unwrap();
        assert_eq!(record.get(0).unwrap(), "Test, Artist");
        assert_eq!(record.get(1).unwrap(), "100");
    }

    #[test]
    fn test_csv_format_empty() {
        let artists = TopArtists { artists: vec![] };

        let config = Config::builder()
            .api_key("test_key")
            .username("test_user")
            .limit(5)
            .period(Period::SevenDay)
            .build()
            .unwrap();

        let formatter = CsvFormatter;
        let result = formatter.format(&artists, &config).unwrap();

        // Should have header but no data rows
        assert!(result.contains("artist,playcount"));
        let mut rdr = csv::Reader::from_reader(result.as_bytes());
        assert_eq!(rdr.records().count(), 0);
    }
}
