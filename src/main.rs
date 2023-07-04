use anyhow::{anyhow, Result};
use clap::Parser;
use dotenv;
use reqwest;
use serde_json::Value;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Your Last.fm API Key
    #[arg(short = 'k', long, env = "API_KEY")]
    api_key: String,

    /// Your Last.fm Username
    #[arg(short, long, env = "USERNAME")]
    username: String,

    /// The limit of Artists
    #[arg(short, long, default_value = "5", env = "LIMIT")]
    limit: u16,

    /// The lookback period
    #[arg(short, long, default_value = "7day", env = "PERIOD")]
    period: String,
}

struct Config {
    api_key: String,
    username: String,
    limit: u16,
    period: String,
}

impl Config {
    fn new(api_key: String, username: String, limit: u16, period: String) -> Self {
        Config {
            api_key,
            username,
            limit,
            period,
        }
    }

    fn get_uri(&self) -> String {
        format!(
            "http://ws.audioscrobbler.com/{}/?method={}&user={}&api_key={}&format={}&period={}&limit={}",
            "2.0",
            "user.gettopartists",
            &self.username,
            &self.api_key,
            "json",
            &self.period,
            &self.limit,
        )
    }
}

fn construct_output(config: Config, json: Value) -> Result<String> {
    let period: &str = match config.period.as_str() {
        "overall" => "",
        "7day" => " week",
        "1month" => " month",
        "3month" => " 3 months",
        "6month" => " 6 months",
        "12month" => " year",
        _ => return Err(anyhow!("Period {} not allowed. Only allow \"overall\", \"7day\", \"1month\", \"3month\", \"6month\", or \"12month\".", config.period))
    };

    let mut output: String = format!(
        "♫ My Top {} played artists in the past{}:",
        config.limit.to_string(),
        period
    );

    let artists = json["topartists"]["artist"]
        .as_array()
        .ok_or(anyhow!("Error parsing JSON."))?;

    for (i, artist) in artists.iter().enumerate() {
        let ending = match i {
            x if x <= (config.limit as usize - 3) => ",",
            x if x == (config.limit as usize - 2) => ", &",
            _ => "",
        };

        let name = artist["name"]
            .as_str()
            .ok_or(anyhow!("Artist not found."))?;
        let playcount = artist["playcount"]
            .as_str()
            .ok_or(anyhow!("Playcount not found."))?;

        output = format!(" {} {} ({}){}", output, name, playcount, ending);
    }

    Ok(format!("{}. Via #LastFM ♫", output))
}

fn main() -> Result<()> {
    if let Some(home_dir) = dirs::home_dir() {
        dotenv::from_filename(format!("{}/.config/lfmc/.env", home_dir.to_string_lossy())).ok();
    }
    let args = Args::parse();
    let config = Config::new(args.api_key, args.username, args.limit, args.period);
    let resp: Result<_, reqwest::Error> = reqwest::blocking::get(config.get_uri())?.json::<Value>();

    if let Ok(json) = resp {
        let output = construct_output(config, json)?;
        println!("\n{}\n", output);
    } else {
        return Err(anyhow!("Could not convert response to JSON."));
    }

    Ok(())
}
