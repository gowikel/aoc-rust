use crate::{
    solvers,
    solvers::{Solution, SolutionExecution},
    Execute,
};
use log::trace;
use std::path::Path;

pub fn solve(execute: Execute, input_path: &Path) -> [Solution; 2] {
    trace!("Running solver for day 02 with Execute {}...", execute);
    solvers::common_solve(execute, input_path, solve_part1, solve_part2)
}

fn solve_part1(input_path: &Path) -> Result<SolutionExecution, &str> {
    trace!("Running part 1...");

    Ok(SolutionExecution::NotImplemented)
}

fn solve_part2(input_path: &Path) -> Result<SolutionExecution, &str> {
    trace!("Running part 2...");

    Ok(SolutionExecution::NotImplemented)
}