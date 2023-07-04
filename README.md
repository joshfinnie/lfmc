# lfmc
A command-line application to view your latest artists from Last.fm

## Use

``` sh
$ lfmc --api-key <LAST.FM API KEY> --username <LAST.FM USERNAME> --period 7days --limit 5
```

The API key, Username, Period, and Limit can be passed via `.env` file which is stored in the `$HOME/.config/lfmc/` folder.

## Develop

LFMC uses Rust.
We're currently using Rust 1.66.1.
Run `cargo build --release` to get executible.
