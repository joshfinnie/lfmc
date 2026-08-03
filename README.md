# lfmc

A command-line application to view your latest artists from Last.fm, written in Rust.

## Features

- 🎵 Fetch your top artists from Last.fm
- 📊 Multiple output formats: Social media text (default), JSON, and CSV
- ⚡ Async HTTP with automatic retry logic
- 🔒 Secure HTTPS API calls
- 📝 Structured logging with adjustable verbosity
- ⚙️ Configuration via environment file or CLI arguments
- ✅ Comprehensive test coverage

## Installation

### Build from Source

```sh
cargo build --release
```

The executable will be located at `target/release/lfmc`.

### Prerequisites

- Rust 1.66.1 or later
- A Last.fm API key (get one at https://www.last.fm/api)

## Usage

### Basic Usage

```sh
lfmc --api-key <YOUR_API_KEY> --username <YOUR_USERNAME>
```

### With All Options

```sh
lfmc \
  --api-key <API_KEY> \
  --username <USERNAME> \
  --period 7day \
  --limit 5 \
  --format social
```

### Output Formats

#### Social (Default)
Perfect for sharing on social media:
```sh
lfmc -k <API_KEY> -u <USERNAME> --format social
```
Output:
```
♫ My Top 5 played artists in the past week: Artist 1 (100), Artist 2 (90), Artist 3 (80), Artist 4 (70), & Artist 5 (60). Via #LastFM ♫
```

#### JSON
For programmatic processing:
```sh
lfmc -k <API_KEY> -u <USERNAME> --format json
```

#### CSV
For spreadsheet analysis:
```sh
lfmc -k <API_KEY> -u <USERNAME> --format csv
```

### Configuration File

`lfmc` reads settings from the first of these it finds:

1. `./.env` — project-local, handy when running from a checkout
2. `$HOME/.config/lfmc/.env` — user-level install

```sh
API_KEY=your_lastfm_api_key
USERNAME=your_lastfm_username
PERIOD=7day
LIMIT=5
```

Real environment variables and CLI flags both override the config files.
If a required setting is missing, `lfmc` lists the locations it searched.

You can use the provided `.env.sample` as a template:

```sh
mkdir -p ~/.config/lfmc
cp .env.sample ~/.config/lfmc/.env
# Edit the file with your credentials
```

> **Note:** the username is read from the `USERNAME` variable, which some shells
> and Windows set automatically. If `lfmc` reports an unexpected user, an
> inherited `USERNAME` is the likely cause — pass `--username` explicitly.

Then run without arguments:
```sh
lfmc
```

## Options

| Option | Short | Environment Variable | Default | Description |
|--------|-------|---------------------|---------|-------------|
| `--api-key` | `-k` | `API_KEY` | Required | Your Last.fm API key |
| `--username` | `-u` | `USERNAME` | Required | Your Last.fm username |
| `--limit` | `-l` | `LIMIT` | 5 | Number of artists to fetch (1-50) |
| `--period` | `-p` | `PERIOD` | 7day | Time period (see below) |
| `--format` | `-f` | - | social | Output format (social, json, csv) |
| `--verbose` | `-v` | - | - | Increase logging verbosity (-v, -vv, -vvv) |

### Valid Periods

- `overall` - All time
- `7day` - Last 7 days
- `1month` - Last month
- `3month` - Last 3 months
- `6month` - Last 6 months
- `12month` - Last 12 months

## Examples

### Most played artists this week (default)
```sh
lfmc -k YOUR_KEY -u YOUR_USERNAME
```

### Top 10 artists of all time as JSON
```sh
lfmc -k YOUR_KEY -u YOUR_USERNAME --period overall --limit 10 --format json
```

### Last month's artists with debug logging
```sh
lfmc -k YOUR_KEY -u YOUR_USERNAME --period 1month -vv
```

### Export to CSV for analysis
```sh
lfmc -k YOUR_KEY -u YOUR_USERNAME --format csv > my_artists.csv
```

## Development

### Building

```sh
cargo build
```

### Running Tests

```sh
cargo test
```

### Linting

```sh
cargo clippy
```

### Formatting

```sh
cargo fmt
```

## Architecture

LFMC follows a modular architecture:

- **API Client**: Async HTTP client with retry logic and exponential backoff
- **Configuration**: Type-safe config with validation
- **Formatters**: Pluggable output formatters (social, JSON, CSV)
- **Error Handling**: Custom error types with helpful messages
- **Testing**: Unit, integration, and CLI end-to-end tests

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
