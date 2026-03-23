use crate::config::Period;
use crate::formatters::Format;
use clap::Parser;

/// A command-line application to view your latest artists from Last.fm
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Your Last.fm API Key
    #[arg(short = 'k', long, env = "API_KEY")]
    pub api_key: String,

    /// Your Last.fm Username
    #[arg(short, long, env = "USERNAME")]
    pub username: String,

    /// The limit of Artists
    #[arg(short, long, default_value = "5", env = "LIMIT")]
    pub limit: u16,

    /// The lookback period
    #[arg(short, long, default_value = "7day", env = "PERIOD")]
    pub period: Period,

    /// Output format
    #[arg(short, long, default_value = "social", value_enum)]
    pub format: Format,

    /// Post output to Bluesky
    #[arg(long, default_value = "false")]
    pub post_bluesky: bool,

    /// Bluesky handle (e.g. user.bsky.social)
    #[arg(long, env = "BLUESKY_HANDLE")]
    pub bluesky_handle: Option<String>,

    /// Bluesky app password
    #[arg(long, env = "BLUESKY_APP_PASSWORD")]
    pub bluesky_app_password: Option<String>,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let args = Cli::parse_from(&["lfmc", "--api-key", "test_key", "--username", "test_user"]);
        assert_eq!(args.api_key, "test_key");
        assert_eq!(args.username, "test_user");
        assert_eq!(args.limit, 5); // default
        assert_eq!(args.period.api_string(), "7day"); // default
    }

    #[test]
    fn test_cli_with_all_args() {
        let args = Cli::parse_from(&[
            "lfmc", "-k", "key123", "-u", "user123", "-l", "10", "-p", "1month", "-f", "json",
            "-vv",
        ]);
        assert_eq!(args.api_key, "key123");
        assert_eq!(args.username, "user123");
        assert_eq!(args.limit, 10);
        assert_eq!(args.period.api_string(), "1month");
        assert_eq!(args.verbose, 2);
    }

    #[test]
    fn test_cli_verbose_counting() {
        let args = Cli::parse_from(&["lfmc", "-k", "key", "-u", "user"]);
        assert_eq!(args.verbose, 0);

        let args = Cli::parse_from(&["lfmc", "-k", "key", "-u", "user", "-v"]);
        assert_eq!(args.verbose, 1);

        let args = Cli::parse_from(&["lfmc", "-k", "key", "-u", "user", "-vvv"]);
        assert_eq!(args.verbose, 3);
    }

    #[test]
    fn test_cli_post_bluesky_flags() {
        let args = Cli::parse_from(&[
            "lfmc",
            "-k", "key",
            "-u", "user",
            "--post-bluesky",
            "--bluesky-handle", "user.bsky.social",
            "--bluesky-app-password", "xxxx-xxxx-xxxx-xxxx",
        ]);
        assert!(args.post_bluesky);
        assert_eq!(args.bluesky_handle.unwrap(), "user.bsky.social");
        assert_eq!(args.bluesky_app_password.unwrap(), "xxxx-xxxx-xxxx-xxxx");
    }

    #[test]
    fn test_cli_post_bluesky_defaults_to_false() {
        let args = Cli::parse_from(&["lfmc", "-k", "key", "-u", "user"]);
        assert!(!args.post_bluesky);
        assert!(args.bluesky_handle.is_none());
        assert!(args.bluesky_app_password.is_none());
    }

    #[test]
    fn test_cli_format_values() {
        let args = Cli::parse_from(&["lfmc", "-k", "key", "-u", "user", "--format", "csv"]);
        assert!(matches!(args.format, Format::Csv));

        let args = Cli::parse_from(&["lfmc", "-k", "key", "-u", "user", "--format", "json"]);
        assert!(matches!(args.format, Format::Json));
    }
}
