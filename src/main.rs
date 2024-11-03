use aoc::{
    services::{
        DateAdapter, DateService, FSService, HTTPAdapter, HTTPService,
        LocalFSAdapter,
    },
    solvers, Execute, Puzzle,
};
use clap::{Args, Parser, Subcommand};
use human_panic::setup_panic;
use lazy_static::lazy_static;
use log::{info, trace};
use std::{cell::LazyCell, error::Error, path::PathBuf, sync::Mutex};

const DATE_SERVICE: LazyCell<DateService<DateAdapter>> =
    LazyCell::new(|| DateService::default());

const FS_SERVICE: LazyCell<FSService<LocalFSAdapter>> =
    LazyCell::new(|| FSService::default());

lazy_static! {
    static ref HTTP_SERVICE: Mutex<HTTPService<HTTPAdapter>> =
        Mutex::new(HTTPService::default());
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
    #[arg(long, short, default_value_t = DATE_SERVICE.default_year(), global = true)]
    year: u32,

    /// Selected day.
    /// Defaults to current day on December between 1-25, 1 otherwise.
    #[arg(long, short, default_value_t = DATE_SERVICE.default_day(), global = true)]
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
    /// Generate the boilerplate code to solve the aforementioned challenge
    Generate,
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
    setup_panic!();
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
            trace!("Download command executing...");
            let mut service = HTTP_SERVICE.lock().unwrap();
            service.set_cookie(args.aoc_cookie);

            let puzzle_data = service.download_input(&puzzle)?;

            print!("{}", puzzle_data);
        }
        Commands::Solve(args) => {
            trace!("Solve command executing for year {}...", puzzle.year());

            match puzzle.year() {
                2023 => {
                    run_y2023_solver(puzzle, args.execute, &args.puzzle_input)
                }
                _ => {
                    eprintln!("{} not implemented", cli.year);
                    std::process::exit(exitcode::DATAERR);
                }
            }
        }
        Commands::Generate => {
            trace!("Generate command executing...");

            FS_SERVICE.extract_template_for(&puzzle)?;
        }
    }

    Ok(())
}

fn run_y2023_solver(puzzle: Puzzle, execute: Execute, input_path: &PathBuf) {
    trace!("Running y2023 solver...");

    let solutions = match puzzle.day() {
        1 => solvers::y2023::day01::solve(execute, input_path),
        2 => solvers::y2023::day02::solve(execute, input_path),
        3 => solvers::y2023::day03::solve(execute, input_path),
        4 => solvers::y2023::day04::solve(execute, input_path),
        5 => solvers::y2023::day05::solve(execute, input_path),
        _ => {
            eprintln!("Day {:02} is not implemented!", puzzle.day());
            std::process::exit(exitcode::UNAVAILABLE)
        }
    };

    solvers::print_results(puzzle, &solutions);
}
