use clap::Parser;
use lfmc::{bluesky::BlueskyClient, formatters::Format, Cli, Config, LastFmClient};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from config files (project-local, then user-level)
    let loaded = lfmc::config::load_config_files();

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

    if loaded.is_empty() {
        tracing::debug!("no config file found; relying on environment and CLI flags");
    } else {
        for path in &loaded {
            tracing::debug!("loaded config from {}", path.display());
        }
    }

    // Resolve credentials, reporting *where* we looked when they are missing.
    let api_key = require_setting(args.api_key, "API key", "--api-key", "API_KEY")?;
    let username = require_setting(args.username, "username", "--username", "USERNAME")?;

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
    let config = Config::new(api_key, username, args.limit, args.period)?;

    // Create API client and fetch data
    let client = LastFmClient::new();
    let response = client.get_top_artists(&config).await?;

    // Format and print output
    let formatter = lfmc::formatters::get_formatter(args.format);
    let output = formatter.format(&response.top_artists, &config)?;

    println!("\n{}\n", output);

    // An empty result is valid data (nothing scrobbled in the window), but it is
    // never worth posting, and silence here is what makes it look like a failure.
    if response.top_artists.artists.is_empty() {
        eprintln!(
            "Note: Last.fm returned no artists for '{}' over the '{}' period \
             — nothing was scrobbled in that window. Try a longer period, e.g. --period 1month.",
            config.username, config.period
        );

        if bluesky_creds.is_some() {
            eprintln!("Skipping Bluesky post: refusing to post an empty artist list.");
        }

        return Ok(());
    }

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

/// Resolves a required setting, or explains exactly how to supply it.
fn require_setting(
    value: Option<String>,
    label: &str,
    flag: &str,
    env_var: &str,
) -> anyhow::Result<String> {
    value.ok_or_else(|| {
        anyhow::anyhow!(
            "Missing Last.fm {label}.\n\n\
             Provide it with {flag} <VALUE>, the {env_var} environment variable, \
             or a '{env_var}=<VALUE>' line in one of these config files:\n{}\n\n\
             None of those were set. See .env.sample for the expected format.",
            lfmc::config::config_paths_hint()
        )
    })
}
