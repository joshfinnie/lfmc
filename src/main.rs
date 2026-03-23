use clap::Parser;
use lfmc::{bluesky::BlueskyClient, formatters::Format, Cli, Config, LastFmClient};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from config file
    if let Some(home_dir) = dirs::home_dir() {
        dotenv::from_filename(format!("{}/.config/lfmc/.env", home_dir.to_string_lossy())).ok();
    }

    // Parse CLI arguments
    let args = Cli::parse();

    // Initialize tracing based on verbosity
    let level = match args.verbose {
        0 => Level::ERROR,
        1 => Level::WARN,
        2 => Level::INFO,
        3 => Level::DEBUG,
        _ => Level::TRACE,
    };

    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Validate Bluesky credentials before making any API calls
    let bluesky_creds = if args.post_bluesky {
        let handle = args.bluesky_handle.ok_or_else(|| {
            anyhow::anyhow!(
                "--bluesky-handle (or BLUESKY_HANDLE env var) is required when using --post-bluesky"
            )
        })?;
        let password = args.bluesky_app_password.ok_or_else(|| {
            anyhow::anyhow!(
                "--bluesky-app-password (or BLUESKY_APP_PASSWORD env var) is required when using --post-bluesky"
            )
        })?;
        Some((handle, password))
    } else {
        None
    };

    // Create config with validation
    let config = Config::new(args.api_key, args.username, args.limit, args.period)?;

    // Create API client and fetch data
    let client = LastFmClient::new();
    let response = client.get_top_artists(&config).await?;

    // Format and print output
    let formatter = lfmc::formatters::get_formatter(args.format);
    let output = formatter.format(&response.top_artists, &config)?;

    println!("\n{}\n", output);

    if let Some((handle, password)) = bluesky_creds {
        // Always post using social format regardless of --format
        let post_text = if matches!(args.format, Format::Social) {
            output.clone()
        } else {
            let social = lfmc::formatters::get_formatter(Format::Social);
            social.format(&response.top_artists, &config)?
        };

        let bsky = BlueskyClient::new();
        bsky.post(&handle, &password, &post_text).await?;
        println!("Posted to Bluesky.");
    }

    Ok(())
}
