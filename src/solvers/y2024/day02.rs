use crate::{
    solvers,
    solvers::{Solution, SolutionExecution},
    Execute,
};
use log::trace;
use std::fs;
use std::path::Path;

/// Solves both parts of day 02's puzzle
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
    trace!("Running solver for day 02 with Execute {}...", execute);
    solvers::common_solve(execute, input_path, solve_part1, solve_part2)
}

fn are_levels_safe(levels: &[u64]) -> bool {
    let have_all_levels_max_step_three = levels
        .iter()
        .zip(levels.iter().skip(1))
        .all(|(&a, &b)| a.abs_diff(b) <= 3);

    let are_all_levels_ascending = levels
        .iter()
        .zip(levels.iter().skip(1))
        .all(|(&a, &b)| a < b);

    let are_all_levels_descending = levels
        .iter()
        .zip(levels.iter().skip(1))
        .all(|(&a, &b)| a > b);

    let are_all_levels_valid = have_all_levels_max_step_three
        && (are_all_levels_ascending || are_all_levels_descending);

    are_all_levels_valid
}

fn solve_part1(path: &Path) -> Result<SolutionExecution, String> {
    trace!("Running part 1...");

    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut result = 0;

    for line in data.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let fields: Result<Vec<u64>, String> = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().map_err(|e| e.to_string()))
            .collect();

        let fields = fields?;

        if are_levels_safe(&fields) {
            result += 1;
        }
    }

    Ok(SolutionExecution::Value(result))
}

fn solve_part2(_path: &Path) -> Result<SolutionExecution, String> {
    trace!("Running part 2...");

    Ok(SolutionExecution::NotImplemented)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod are_levels_safe {
        use super::*;

        #[test]
        fn test_happy_path() {
            let ascending = vec![1, 2, 3, 4];
            let descending = vec![4, 3, 2, 1];
            let with_jumps = vec![1, 3, 6, 7];

            assert!(are_levels_safe(&ascending));
            assert!(are_levels_safe(&descending));
            assert!(are_levels_safe(&with_jumps));
        }

        #[test]
        fn test_trend_change() {
            let final_change = vec![1, 3, 6, 5];
            let start_change = vec![1, 7, 6, 5];
            let mid_change = vec![1, 4, 3, 5];

            assert!(!are_levels_safe(&final_change));
            assert!(!are_levels_safe(&start_change));
            assert!(!are_levels_safe(&mid_change));
        }

        #[test]
        fn test_jumps() {
            // Valid ones
            let jump_1 = vec![1, 2];
            let jump_2 = vec![1, 3];
            let jump_3 = vec![1, 4];

            // Invalid ones
            let jump_4 = vec![1, 5];
            let jump_5 = vec![1, 6];

            assert!(are_levels_safe(&jump_1));
            assert!(are_levels_safe(&jump_2));
            assert!(are_levels_safe(&jump_3));
            assert!(!are_levels_safe(&jump_4));
            assert!(!are_levels_safe(&jump_5));
        }

        #[test]
        fn test_unary_levels() {
            let unary = vec![1];
            assert!(are_levels_safe(&unary));
        }
    }
}
