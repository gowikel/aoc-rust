use crate::{
    solvers,
    solvers::{Solution, SolutionExecution},
    Execute,
};
use aho_corasick::AhoCorasick;
use log::trace;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn solve(execute: Execute, input_path: &Path) -> [Solution; 2] {
    trace!("Running solver for day 01 with Execute {}...", execute);
    solvers::common_solve(execute, input_path, solve_part1, solve_part2)
}

fn solve_part1(input_path: &Path) -> Result<SolutionExecution, String> {
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

fn solve_part2(input_path: &Path) -> Result<SolutionExecution, String> {
    trace!("Running part 2...");

    let patterns = &[
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight",
        "nine", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    ];
    let ac = AhoCorasick::new(patterns).unwrap();

    let file =
        File::open(input_path).map_err(|_| input_path.to_str().unwrap())?;
    let reader = BufReader::new(file);
    let mut parsed_numbers: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line.");

        let digits: Vec<u32> = ac
            .find_overlapping_iter(&line)
            .map(|m| line[m.start()..m.end()].to_string())
            .map(|s| match s.as_str() {
                "zero" => "0".to_string(),
                "one" => "1".to_string(),
                "two" => "2".to_string(),
                "three" => "3".to_string(),
                "four" => "4".to_string(),
                "five" => "5".to_string(),
                "six" => "6".to_string(),
                "seven" => "7".to_string(),
                "eight" => "8".to_string(),
                "nine" => "9".to_string(),
                other => other.to_string(),
            })
            .map(|n| n.parse::<u32>().expect("Expected an integer!"))
            .collect();

        if digits.len() == 0 {
            continue;
        }

        let first = digits.first().expect("Expected at least one digit.");
        let last = digits.last().unwrap_or(first);
        let parsed_line_result = 10 * first + last;

        parsed_numbers.push(parsed_line_result);
    }

    let result = parsed_numbers.iter().sum();

    Ok(SolutionExecution::Value(result))
}
