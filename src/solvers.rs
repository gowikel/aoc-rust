//! This contains the solutions to the different challenges

use crate::{Execute, Puzzle};
use derive_more::Display;
use num_format::{Buffer, Locale};
use owo_colors::OwoColorize;
use std::path::Path;
use tabled::{
    builder::Builder,
    settings::{
        object::Rows, style::BorderSpanCorrection, Alignment, Settings, Span,
        Style,
    },
};

pub mod y2023;

/// Solution returned by the solver
#[derive(Debug, Display, PartialEq, Eq, Clone)]
pub enum Solution {
    Value(u64),
    Err(String),
    NotExecuted,
    NotImplemented,
}

#[derive(Debug, Display, PartialEq, Eq, Clone)]
pub enum SolutionExecution {
    Value(u64),
    NotImplemented,
}

impl From<Result<SolutionExecution, String>> for Solution {
    fn from(value: Result<SolutionExecution, String>) -> Self {
        match value {
            Err(s) => Solution::Err(s),
            Ok(execution_value) => match execution_value {
                SolutionExecution::NotImplemented => Solution::NotImplemented,
                SolutionExecution::Value(value) => Solution::Value(value),
            },
        }
    }
}

/// Helper to print the solutions for each solver
pub fn print_results(puzzle: Puzzle, solutions: &[Solution; 2]) {
    let mut builder: Builder = Builder::default();
    let error_string = "Error:".red().bold().to_string();

    builder.push_record(vec![format!(
        "Solutions {}/{:02}",
        puzzle.year(),
        puzzle.day()
    )
    .bold()
    .to_string()]);

    for (index, solution) in solutions.iter().enumerate() {
        let header = format!("P{}", index + 1).bold().to_string();
        let solution = match solution {
            Solution::NotExecuted => "Not executed".to_string(),
            Solution::NotImplemented => "Not implemented".to_string(),
            Solution::Err(err) => {
                format!("{} {}", error_string, err).to_string()
            }
            Solution::Value(x) => {
                let mut buf = Buffer::default();
                buf.write_formatted(x, &Locale::en);
                buf.as_str().green().to_string()
            }
        };

        builder.push_record(vec![header, solution]);
    }

    let mut table = builder.build();
    table
        .with(Style::rounded())
        .modify(Rows::first(), Span::column(2))
        .modify(
            Rows::first(),
            Settings::new(Alignment::center(), Alignment::center()),
        )
        .with(BorderSpanCorrection);

    println!("{}", table.to_string());
}

/// Runs the required solvers and returns a [Solution; 2] that can be used
/// to interpret the results
fn common_solve(
    execute: Execute,
    input_path: &Path,
    solve_part1: fn(&Path) -> Result<SolutionExecution, String>,
    solve_part2: fn(&Path) -> Result<SolutionExecution, String>,
) -> [Solution; 2] {
    let mut solutions: [Solution; 2] =
        [Solution::NotExecuted, Solution::NotExecuted];

    if execute == Execute::ALL || execute == Execute::P1 {
        solutions[0] = solve_part1(&input_path).into();
    }

    if execute == Execute::ALL || execute == Execute::P2 {
        solutions[1] = solve_part2(&input_path).into();
    }

    solutions
}
