use crate::{
    solvers::{Solution, SolutionExecution},
    Execute,
};
use log::trace;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn solve(execute: Execute, input_path: &Path) -> [Solution; 2] {
    trace!("Running solver for day 01 with Execute {}...", execute);
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

fn solve_part1(input_path: &Path) -> Result<SolutionExecution, &str> {
    trace!("Running part 1...");

    let file =
        File::open(input_path).map_err(|_| input_path.to_str().unwrap())?;
    let reader = BufReader::new(file);

    let mut parsed_numbers: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line.");

        let digits: Vec<u32> = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        if digits.len() == 0 {
            continue;
        }

        let first = digits.first().unwrap();
        let last = digits.last().unwrap_or(first);

        parsed_numbers.push(10 * first + last);
    }

    let result = parsed_numbers.iter().sum();

    Ok(SolutionExecution::Value(result))
}

fn solve_part2(input_path: &Path) -> Result<SolutionExecution, &str> {
    trace!("Running part 2...");

    Ok(SolutionExecution::NotImplemented)
}
