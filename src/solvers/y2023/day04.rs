//! # Scratchcard Game Solver Documentation
//!
//! This module implements a solver for a scratchcard game puzzle (Day 4).
//! The implementation uses lexical analysis to process game cards and calculate scores
//! based on matching numbers.
//!
//! For more information, visit the
//! [AoC 2023 Day 4 puzzle page](https://adventofcode.com/2023/day/4).

use crate::{
    solvers,
    solvers::{Solution, SolutionExecution},
    Execute,
};
use log::{debug, trace};
use logos::Logos;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Solves both parts of day 04's puzzle
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
    trace!("Running solver for day 04 with Execute {}...", execute);
    solvers::common_solve(execute, input_path, solve_part1, solve_part2)
}

/// Enum representing the lexical tokens for the scratchcard game.
#[derive(Debug, PartialEq, Logos)]
#[logos(skip r"[ \t\n\f]+")]
enum Token {
    /// Token representing a card number with its associated card number.
    #[regex("Card +[0-9]+:", card_callback)]
    Card(u32),

    /// Token representing a separator (`|`).
    #[token("|")]
    Separator,

    /// Token representing a number.
    #[regex("[0-9]+", number_callback)]
    Number(u32),
}

/// Enum representing the reading status while parsing the input.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ReadingStatus {
    /// Reading the winning numbers, before the separator.
    ReadingWinningNumbers,

    /// Reading the played numbers, after the separator.
    MatchingNumbers,
}

/// Callback function for the `Card` token. Extracts the card number from the token.
fn card_callback(lexer: &logos::Lexer<Token>) -> Option<u32> {
    let slice = lexer.slice();

    slice
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .ok()
}

/// Callback function for the `Number` token.
fn number_callback(lexer: &logos::Lexer<Token>) -> Option<u32> {
    lexer.slice().parse().ok()
}

/// Solution to part 1.
/// # Challenge
///
/// Read a set of cards, and return the sum of all the card values.
/// A card value is defined as:
///
/// * 1, for the first played number in the winning set
/// * For each remaining number, multiply the value by 2
fn solve_part1(input_path: &Path) -> Result<SolutionExecution, String> {
    trace!("Running part 1...");

    let file = File::open(input_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut result = 0;

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let mut reading_status = ReadingStatus::ReadingWinningNumbers;
        let mut winning_numbers = Vec::new();
        let mut card_value = 0;

        for token in Token::lexer(&line) {
            let token = token.map_err(|_| {
                format!("Unable to parse token in line: {}", line)
            })?;

            match token {
                Token::Separator => {
                    reading_status = ReadingStatus::MatchingNumbers
                }
                Token::Number(n) => {
                    if reading_status == ReadingStatus::ReadingWinningNumbers {
                        winning_numbers.push(n);
                    } else {
                        if winning_numbers.contains(&n) {
                            if card_value == 0 {
                                card_value = 1;
                            } else {
                                card_value *= 2;
                            }
                        }
                    }
                }
                Token::Card(_) => (),
            }
        }

        result += card_value;
    }

    Ok(SolutionExecution::Value(result))
}

fn solve_part2(input_path: &Path) -> Result<SolutionExecution, String> {
    trace!("Running part 2...");

    Ok(SolutionExecution::NotImplemented)
}
