use crate::{solvers, solvers::Solution, Execute, Puzzle};
use log::trace;
use std::path::Path;

pub fn solve(execute: Execute, input_path: &Path) -> [Solution; 2] {
    trace!("Running solver for day 01 with Execute {}...", execute);
    let mut solutions: [Solution; 2] =
        [Solution::NotExecuted, Solution::NotExecuted];

    if execute == Execute::ALL || execute == Execute::P1 {
        solutions[0] = solve_part1(&input_path);
    }

    if execute == Execute::ALL || execute == Execute::P2 {
        solutions[1] = solve_part2(&input_path);
    }

    solutions
}

fn solve_part1(input_path: &Path) -> Solution {
    Solution::NotImplemented
}

fn solve_part2(input_path: &Path) -> Solution {
    Solution::NotImplemented
}
