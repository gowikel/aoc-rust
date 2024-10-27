//! Day 3 solver implementation for processing schematic data
//!
//! This module provides functionality to parse and analyze a schematic containing
//! numbers and symbols, particularly focusing on identifying numbers adjacent to symbols
//! and calculating various metrics based on their positions.

use crate::{
    solvers,
    solvers::{Solution, SolutionExecution},
    Execute,
};
use log::trace;
use logos::{Lexer, Logos, Skip};
use std::fs;
use std::ops::Range;
use std::path::Path;

/// Solves both parts of day 3's puzzle
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
    trace!("Running solver for day 03 with Execute {}...", execute);
    solvers::common_solve(execute, input_path, solve_part1, solve_part2)
}

/// Represents the position of a number token in the schematic
#[derive(Debug, Clone, PartialEq, Eq)]
struct NumberTokenPosition {
    /// Row index (0-based)
    row: u32,

    /// Column range containing the number
    col: Range<u32>,
}

impl NumberTokenPosition {
    /// Creates a new number token position
    fn new(row: u32, col: Range<u32>) -> Self {
        Self { row, col }
    }
}

/// Represents the position of a symbol in the schematic
#[derive(Debug, Clone, PartialEq, Eq)]
struct SymbolPosition {
    /// Row index (0-based)
    row: u32,

    /// Col index (0-based)
    col: u32,
}

impl SymbolPosition {
    /// Creates a new symbol position
    fn new(row: u32, col: u32) -> Self {
        Self { row, col }
    }
}

/// Represents a number found in the schematic
#[derive(Debug, Clone, PartialEq, Eq)]
struct NumberToken {
    /// The numeric value
    value: u32,

    /// Position information of the number in the schematic
    position: NumberTokenPosition,
}

impl NumberToken {
    /// Creates a new number token
    fn new(value: u32, position: NumberTokenPosition) -> Self {
        Self { value, position }
    }

    /// Creates a number token from a lexeme and position
    ///
    /// # Arguments
    ///
    /// * `lexeme` - The string representation of the number
    /// * `position` - Position information for the number
    ///
    /// # Returns
    ///
    /// Option containing the NumberToken if parsing succeeds
    fn from_lexeme(
        lexeme: &str,
        position: NumberTokenPosition,
    ) -> Option<Self> {
        let number: u32 = lexeme.parse().ok()?;

        Some(Self::new(number, position))
    }

    /// Determines if the number is in range of a given SymbolPosition
    ///
    /// # Arguments
    ///
    /// * `position` - The SymbolPosition to check against
    ///
    /// # Returns
    ///
    /// True if the number is in range of the SymbolPosition, false otherwise
    fn is_in_range(&self, position: &SymbolPosition) -> bool {
        let number_row = self.position.row;
        let number_col_start = self.position.col.start;
        let number_col_end = self.position.col.end;

        let mut expanded_row_position_start = 0;
        if number_row > 0 {
            expanded_row_position_start = number_row - 1;
        }

        let mut expanded_col_position_start = 0;
        if number_col_start > 0 {
            expanded_col_position_start = number_col_start - 1;
        }

        let expanded_row_position =
            expanded_row_position_start..=(number_row + 1);
        let expanded_col_position =
            expanded_col_position_start..=(number_col_end);

        expanded_col_position.contains(&position.col)
            && expanded_row_position.contains(&position.row)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SymbolToken {
    /// Position information of the symbol in the schematic
    position: SymbolPosition,
}

impl SymbolToken {
    /// Creates a new symbol token
    fn new(position: SymbolPosition) -> Self {
        Self { position }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GearToken {
    /// Position information of the gear in the schematic
    position: SymbolPosition,
}

impl GearToken {
    /// Creates a new gear token
    fn new(position: SymbolPosition) -> Self {
        Self { position }
    }
}

/// Metadata for tracking lexer position information
struct Metadata {
    /// Current line number
    line_number: u32,

    /// Current offset in the line
    offset: u32,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            line_number: 1,
            offset: 0,
        }
    }
}

/// Token types recognized by the lexer
#[derive(Logos, Debug)]
#[logos(skip r"[. \t\f]+")]
#[logos(extras = Metadata)]
enum Token {
    /// Newline character
    #[token("\n", newline_callback)]
    NewLine,

    /// Numeric tokens
    #[regex(r"[0-9]+", number_callback)]
    Number(NumberToken),

    /// Gear tokens (asterisk symbol)
    #[token("*", gear_callback)]
    Gear(GearToken),

    /// Symbol tokens (anything that's not a number, newline, or whitespace)
    #[regex(r"[^0-9*\n. \t\f]", symbol_callback)]
    Symbol(SymbolToken),
}

/// Calculates the adjusted position based on the current offset
///
/// # Arguments
///
/// * `n` - Raw position
/// * `metadata` - Current lexer metadata
///
/// # Returns
///
/// Adjusted position accounting for line offsets
fn calculate_offset_position(n: u32, metadata: &Metadata) -> u32 {
    if metadata.offset == 0 {
        return n;
    }

    n - metadata.offset - 1
}

/// Callback for handling newline tokens
///
/// Updates the line number and offset in the lexer metadata
fn newline_callback(lexer: &mut Lexer<Token>) -> Skip {
    lexer.extras.line_number += 1;
    lexer.extras.offset = lexer.span().start as u32;

    Skip
}

/// Callback for handling number tokens
///
/// Creates a NumberToken with position information from the lexer
fn number_callback(lexer: &Lexer<Token>) -> Option<NumberToken> {
    let lexeme = lexer.slice();
    let row = lexer.extras.line_number;

    let col_start =
        calculate_offset_position(lexer.span().start as u32, &lexer.extras);
    let col_end =
        calculate_offset_position(lexer.span().end as u32, &lexer.extras);

    let col_range = col_start..col_end;

    let position = NumberTokenPosition::new(row, col_range);

    NumberToken::from_lexeme(lexeme, position)
}

/// Callback for handling symbol tokens
///
/// Creates a SymbolToken with position information from the lexer
fn symbol_callback(lexer: &Lexer<Token>) -> SymbolToken {
    let row = lexer.extras.line_number;
    let col_start =
        calculate_offset_position(lexer.span().start as u32, &lexer.extras);

    SymbolToken::new(SymbolPosition::new(row, col_start))
}

/// Callback for handling gear tokens
///
/// Creates a GearToken with position information from the lexer
fn gear_callback(lexer: &Lexer<Token>) -> GearToken {
    let row = lexer.extras.line_number;
    let col_start =
        calculate_offset_position(lexer.span().start as u32, &lexer.extras);

    GearToken::new(SymbolPosition::new(row, col_start))
}

/// Solves part 1 of the puzzle
///
/// Finds all numbers that are adjacent to symbols and sums them
///
/// # Arguments
///
/// * `input_path` - Path to the input file
///
/// # Returns
///
/// Result containing the sum of all numbers adjacent to symbols
fn solve_part1(input_path: &Path) -> Result<SolutionExecution, String> {
    trace!("Running part 1...");

    let mut symbols: Vec<SymbolPosition> = Vec::new();
    let mut numbers: Vec<NumberToken> = Vec::new();

    let data = fs::read_to_string(input_path).map_err(|e| e.to_string())?;

    for token in Token::lexer(&data) {
        let token = token.map_err(|_| "Failed to parse data")?;

        match token {
            Token::Number(data) => numbers.push(data),
            Token::Symbol(data) => symbols.push(data.position),
            Token::Gear(data) => symbols.push(data.position),
            _ => {}
        }
    }

    let mut result = 0;

    'outer: for number in numbers {
        for symbol in &symbols {
            if number.is_in_range(symbol) {
                result += number.value;
                continue 'outer;
            }
        }
    }

    Ok(SolutionExecution::Value(result))
}

/// Solves part 2 of the puzzle (not implemented)
///
/// # Arguments
///
/// * `input_path` - Path to the input file
///
/// # Returns
///
/// Result indicating the solution is not implemented
fn solve_part2(input_path: &Path) -> Result<SolutionExecution, String> {
    trace!("Running part 2...");

    let data = fs::read_to_string(input_path).map_err(|e| e.to_string())?;
    let mut gears: Vec<SymbolPosition> = Vec::new();
    let mut numbers: Vec<NumberToken> = Vec::new();
    let mut result = 0;

    for token in Token::lexer(&data) {
        let token = token.map_err(|_| "Failed to parse data")?;

        match token {
            Token::Number(data) => numbers.push(data),
            Token::Gear(data) => gears.push(data.position),
            _ => {}
        }
    }

    for gear in gears {
        let mut numbers_in_range: Vec<u32> = Vec::new();

        for number in &numbers {
            if number.is_in_range(&gear) {
                numbers_in_range.push(number.value);
            }

            if numbers_in_range.len() > 2 {
                result += number.value;
                break;
            }
        }

        if numbers_in_range.len() == 2 {
            result += numbers_in_range.iter().fold(1, |acc, x| acc * x);
        }
    }

    Ok(SolutionExecution::Value(result))
}
