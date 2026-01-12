use clap::Parser;
use lfmc::{Cli, Config, LastFmClient};
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

    // Create config with validation
    let config = Config::new(args.api_key, args.username, args.limit, args.period)?;

    // Create API client and fetch data
    let client = LastFmClient::new();
    let response = client.get_top_artists(&config).await?;

    // Format and print output
    let formatter = lfmc::formatters::get_formatter(args.format);
    let output = formatter.format(&response.top_artists, &config)?;

    println!("\n{}\n", output);

    Ok(())
}
