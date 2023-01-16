use anyhow::{anyhow, Result};
use clap::Parser;
use dotenv::dotenv;
use reqwest;
use serde_json::Value;

#[derive(Parser, Debug)]
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

#[derive(Debug)]
struct Config {
    api_key: String,
    username: String,
    limit: u16,
    period: String,
}

impl Config {
    fn new(api_key: String, username: String, limit: u16, period: String) -> Result<Self> {
        Ok(Config {
            api_key,
            username,
            limit,
            period,
        })
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
        "6month" => r#" 6 months"#,
        "12month" => " year",
        _ => return Err(anyhow!("Period {} not allowed. Only allow \"overall\", \"7day\", \"1month\", \"3month\", \"6month\", or \"12month\".", config.period))
    };

    let mut output: String = format!(
        "♫ My Top {} played artists in the past{}:",
        config.limit.to_string(),
        period
    );

    let artists = match json["topartists"]["artist"].as_array() {
        Some(a) => a,
        None => return Err(anyhow!("Error parsing json.")),
    };

    for (i, artist) in artists.iter().enumerate() {
        let ending = match i {
            x if x <= (config.limit as usize - 3) => ",",
            x if x == (config.limit as usize - 2) => ", &",
            _ => "",
        };

        let name = match artist["name"].as_str() {
            Some(n) => n,
            None => return Err(anyhow!("Artist not found.")),
        };

        let playcount = match artist["playcount"].as_str() {
            Some(p) => p,
            None => return Err(anyhow!("Playcount not found.")),
        };

        output = format!(" {} {} ({}){}", output, name, playcount, ending);
    }

    Ok(format!("{}. Via #LastFM ♫", output))
}

fn main() -> Result<()> {
    dotenv().ok();
    let args = Args::parse();

    let c = Config::new(args.api_key, args.username, args.limit, args.period)?;

    let r: Result<_, reqwest::Error> = reqwest::blocking::get(c.get_uri())?.json::<Value>();

    if let Ok(j) = r {
        let output = construct_output(c, j)?;
        println!("{}", output);
    } else {
        return Err(anyhow!("Could not convert response to JSON."));
    }

    Ok(())
}
