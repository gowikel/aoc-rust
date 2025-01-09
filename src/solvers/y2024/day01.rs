use crate::{
    solvers,
    solvers::{Solution, SolutionExecution},
    Execute,
};
use log::trace;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Solves both parts of day 01's puzzle
///
/// # Arguments
///
/// * `execute` - Execution mode configuration
/// * `input_path` - Path to the input file containing the schematic
///
/// # Returns
///
/// Array containing solutions for both parts of the puzzle
pub fn solve(execute: Execute, input_path: &Path) -> [Solution; 2] {
    trace!("Running solver for day 01 with Execute {}...", execute);
    solvers::common_solve(execute, input_path, solve_part1, solve_part2)
}

fn solve_part1(path: &Path) -> Result<SolutionExecution, String> {
    trace!("Running part 1...");
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let line_count = data.matches("\n").count() + 1;

    let mut list_a = Vec::with_capacity(line_count);
    let mut list_b = Vec::with_capacity(line_count);

    for line in data.lines() {
        let fields: Vec<String> =
            line.split_whitespace().map(|s| s.to_string()).collect();

        if fields.len() != 2 {
            return Err(format!("Invalid line: {}", line));
        }

        let a: i64 = fields[0]
            .parse()
            .map_err(|_| format!("Invalid number: {}", fields[0]))?;
        let b: i64 = fields[1]
            .parse()
            .map_err(|_| format!("Invalid number: {}", fields[1]))?;

        list_a.push(a);
        list_b.push(b);
    }

    list_a.sort();
    list_b.sort();

    let result: i64 = list_a
        .into_iter()
        .zip(list_b.into_iter())
        .fold(0, |acc, (a, b)| acc + (a - b).abs());

    Ok(SolutionExecution::Value(result as u64))
}

fn solve_part2(path: &Path) -> Result<SolutionExecution, String> {
    trace!("Running part 2...");

    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let line_count = data.matches("\n").count() + 1;

    let mut list_a = Vec::with_capacity(line_count);
    let mut occurrences = HashMap::new();

    for line in data.lines() {
        let fields: Vec<String> =
            line.split_whitespace().map(|s| s.to_string()).collect();

        if fields.len() != 2 {
            return Err(format!("Invalid line: {}", line));
        }

        let a: u64 = fields[0]
            .parse()
            .map_err(|_| format!("Invalid number: {}", fields[0]))?;
        let b: u64 = fields[1]
            .parse()
            .map_err(|_| format!("Invalid number: {}", fields[1]))?;

        list_a.push(a);
        occurrences.entry(b).and_modify(|v| *v += 1).or_insert(1u64);
    }

    let score: u64 = list_a
        .into_iter()
        .map(|a| a * occurrences.get(&a).unwrap_or(&0))
        .sum();

    Ok(SolutionExecution::Value(score))
}
