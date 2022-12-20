use anyhow::{anyhow, Result};
use clap::Parser;
use reqwest;
use serde_json::Value;

/// An application to view your latest artists from Last.fm
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Your Last.fm API Key
    #[arg(short = 'k', long)]
    api_key: String,

    /// Your Last.fm Username
    #[arg(short, long)]
    username: String,

    /// The limit of Artists
    #[arg(short, long)]
    limit: u16,

    /// The lookback period
    #[arg(short, long, default_value = "7day")]
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

    let mut f = format!(
        "♫ My Top {} played artists in the past{}:",
        config.limit.to_string(),
        period
    );

    let artists = json["topartists"]["artist"].as_array().unwrap();
    for (i, artist) in artists.iter().enumerate() {
        let ending = match i {
            x if x <= (config.limit as usize - 3) => ",",
            x if x == (config.limit as usize - 2) => ", &",
            _ => "",
        };

        f = format!(
            " {} {} ({}){}",
            f,
            artist["name"].as_str().unwrap(),
            artist["playcount"].as_str().unwrap(),
            ending
        );
    }
    f = format!("{}. Via #LastFM ♫", f);
    Ok(f.to_string())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let c = Config::new(args.api_key, args.username, args.limit, args.period);

    let r: Result<_, reqwest::Error> = reqwest::blocking::get(c.get_uri())?.json::<Value>();

    if let Ok(j) = r {
        let output = construct_output(c, j)?;
        println!("{}", output);
    } else {
        return Err(anyhow!("Could not convert response to JSON."))
    }

    Ok(())
}
