use aoc::{actions, cli, providers, providers::http::HTTPProvider, Execute};
use clap::{Args, Parser, Subcommand, ValueEnum};
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

fn validate_is_file(data: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(data);

    if !path.is_file() {
        return Err(format!("{} is not a file", path.to_str().unwrap()));
    }

    Ok(path)
}

#[derive(Parser)]
#[command(version, author, about)]
struct Cli {
    /// Selected year.
    /// Defaults to current year on December, last year otherwise.
    #[arg(long, short, default_value_t = calculate_default_year(), global = true)]
    year: u32,

    /// Selected day.
    /// Defaults to current day on December between 1-25, 1 otherwise.
    #[arg(long, short, default_value_t = calculate_default_day(), global = true)]
    day: u32,

    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug, PartialEq)]
enum Commands {
    /// Downloads the specified puzzle input from AoC
    Download(DownloadArgs),
    /// Solve the specified puzzle
    Solve(SolveArgs),
}

#[derive(Args, PartialEq, Debug)]
struct DownloadArgs {
    /// AOC_COOKIE required to download the puzzle input. Can be set in an envvar.
    #[arg(long, short, env, hide_env_values = true)]
    aoc_cookie: String,
}

#[derive(Args, PartialEq, Debug)]
struct SolveArgs {
    #[arg(value_parser = validate_is_file)]
    puzzle_input: PathBuf,

    #[arg(value_enum, default_value_t = Execute::ALL)]
    execute: Execute,
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
        Commands::Solve(args) => {
            println!("Solving puzzle... {:?}", args.puzzle_input);
        }
    }

    Ok(())
}
