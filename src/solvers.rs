//! This contains the solutions to the different challenges

use crate::Puzzle;
use derive_more::Display;
use owo_colors::OwoColorize;
use tabled::builder::Builder;
use tabled::settings::object::Rows;
use tabled::settings::style::BorderSpanCorrection;
use tabled::settings::{Alignment, Settings, Span, Style};

pub mod y2023;

/// Solution returned by the solver
#[derive(Debug, Display, PartialEq, Eq, Clone)]
pub enum Solution {
    Value(i32),
    Err(String),
    NotExecuted,
    NotImplemented,
}

#[derive(Debug, Display, PartialEq, Eq, Clone)]
pub enum SolutionExecution {
    Value(i32),
    NotImplemented,
}

impl From<Result<SolutionExecution, &str>> for Solution {
    fn from(value: Result<SolutionExecution, &str>) -> Self {
        match value {
            Err(s) => Solution::Err(s.to_string()),
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
            Solution::Value(x) => format!("{}", x).green().to_string(),
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
