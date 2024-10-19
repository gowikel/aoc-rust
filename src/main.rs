use aoc::{actions, cli, constants, providers, providers::http::HTTPProvider};
use clap::{ArgAction, Args, Parser, Subcommand};
use log::{info, trace};
use std::{error::Error, path::PathBuf};

fn calculate_default_year() -> u32 {
    let provider = providers::date::default_date_provider();
    cli::default_year(&provider)
}

fn calculate_default_day() -> u32 {
    let provider = providers::date::default_date_provider();
    cli::default_day(&provider)
}

#[derive(Parser)]
#[command(version, author, about)]
struct Cli {
    /// Selected year.
    /// Defaults to current year on December, last year otherwise.
    #[arg(long, short, default_value_t = calculate_default_year())]
    year: u32,

    /// Selected day.
    /// Defaults to current day on December between 1-25, 1 otherwise.
    #[arg(long, short, default_value_t = calculate_default_day())]
    day: u32,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Commands {
    /// Downloads the specified puzzle input from AoC
    Download(DownloadArgs),

#[derive(Args, PartialEq, Debug)]
struct DownloadArgs {
    /// AOC_COOKIE required to download the puzzle input. Can be set in an envvar.
    #[arg(long, short, env, hide_env_values = true)]
    aoc_cookie: String,
}
}

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let cli = Cli::parse();

    info!("Application started...");

    trace!(
        "Initializing the puzzle with year: {} and day: {}...",
        cli.year,
        cli.day
    );
    let puzzle = aoc::Puzzle::new(cli.year, cli.day)?;

    match cli.command {
        Commands::Download(args) => {
            let mut http_provider =
                providers::http::get_default_http_provider();

            http_provider.set_cookie(args.aoc_cookie);

            let puzzle_data = actions::download_input(&http_provider, puzzle)?;

            println!("{}", puzzle_data);
        }
    }

    Ok(())
}
