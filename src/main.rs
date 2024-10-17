use anyhow::{Context, Result};
use aoc::{actions, cli, constants, providers};
use clap::{Parser, Subcommand};
use log::{info, trace};
use std::cmp::PartialEq;
use std::env;

fn calculate_default_year() -> u32 {
    let provider = providers::date::default_date_provider();
    cli::default_year(&provider)
}

fn calculate_default_day() -> u32 {
    let provider = providers::date::default_date_provider();
    cli::default_day(&provider)
}

struct HTTPAdapter {}

impl providers::http::HTTPProvider for HTTPAdapter {
    fn get(&self, endpoint: &str) -> Result<String> {
        let client = reqwest::blocking::Client::default();
        let aoc_cookie =
            env::var(constants::AOC_COOKIE).with_context(|| {
                format!("Missing {} env variable", constants::AOC_COOKIE)
            })?;

        let response = client
            .get(endpoint)
            .header(reqwest::header::COOKIE, aoc_cookie)
            .send()
            .with_context(|| format!("Unable to GET {}", endpoint))?;

        let result = response.text().with_context(|| {
            format!("Unable to parse response from {}", endpoint)
        })?;

        Ok(result)
    }
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
    Download,
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    providers::http::init(HTTPAdapter {}).unwrap();

    let cli = Cli::parse();

    info!("Application started...");
    trace!("Checking Download requirements are met...");
    if Commands::Download == cli.command {
        env::var(constants::AOC_COOKIE).with_context(|| {
            format!("{} is required to make a download.", constants::AOC_COOKIE)
        })?;
    }

    trace!(
        "Initializing the puzzle with year: {} and day: {}...",
        cli.year,
        cli.day
    );
    let puzzle = aoc::Puzzle::new(cli.year, cli.day)?;

    match cli.command {
        Commands::Download {} => {
            let puzzle_data = actions::download_input(puzzle)?;

            println!("{}", puzzle_data);
        }
    }

    Ok(())
}
