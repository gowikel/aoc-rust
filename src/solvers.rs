//! This contains the solutions to the different challenges

use crate::Execute;
use derive_more::Display;
use std::path::Path;

pub mod y2023;
pub mod y2024;

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
