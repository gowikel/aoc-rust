use anyhow::{Context, Result};
use aoc::{cli, constants, http};
use chrono::Datelike;
use clap::{Parser, Subcommand};
use log::{info, trace};
use std::cmp::PartialEq;
use std::env;

struct DateAdapter {}

impl cli::DateInfoProvider for DateAdapter {
    fn current_year(&self) -> u32 {
        chrono::Local::now().year() as u32
    }

    fn current_month(&self) -> cli::Month {
        match chrono::Local::now().month() {
            1 => cli::Month::January,
            2 => cli::Month::February,
            3 => cli::Month::March,
            4 => cli::Month::April,
            5 => cli::Month::May,
            6 => cli::Month::June,
            7 => cli::Month::July,
            8 => cli::Month::August,
            9 => cli::Month::September,
            10 => cli::Month::October,
            11 => cli::Month::November,
            12 => cli::Month::December,
            _ => unreachable!("Month should never get here"),
        }
    }

    fn current_day(&self) -> u32 {
        chrono::Local::now().day()
    }
}

fn calculate_default_year() -> u32 {
    let provider = DateAdapter {};
    cli::default_year(&provider)
}

fn calculate_default_day() -> u32 {
    let provider = DateAdapter {};
    cli::default_day(&provider)
}

struct HTTPAdapter {}

impl http::HTTPProvider for HTTPAdapter {
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
    let puzzle = aoc::Puzzle::new(cli.year, cli.day, HTTPAdapter {})?;

    match cli.command {
        Commands::Download {} => {
            let puzzle_data = puzzle.download()?;

            println!("{}", puzzle_data);
        }
    }

    Ok(())
}
