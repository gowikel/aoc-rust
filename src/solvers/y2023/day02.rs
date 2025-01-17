use crate::{
    solvers,
    solvers::{Solution, SolutionExecution},
    Execute,
};
use log::trace;
use logos::{Lexer, Logos};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn solve(execute: Execute, input_path: &Path) -> [Solution; 2] {
    trace!("Running solver for day 02 with Execute {}...", execute);
    solvers::common_solve(execute, input_path, solve_part1, solve_part2)
}

#[derive(Logos, Debug)]
#[logos(skip r"[:;, \t\n\f]+")]
enum Token {
    #[regex(r"Game [0-9]+", parse_game)]
    Game(u64),

    #[regex(r"[0-9]+ blue", parse_color)]
    Blue(u64),

    #[regex(r"[0-9]+ green", parse_color)]
    Green(u64),

    #[regex(r"[0-9]+ red", parse_color)]
    Red(u64),
}

fn parse_game(lex: &mut Lexer<Token>) -> Option<u64> {
    let number = lex.slice().split_whitespace().nth(1)?;
    let parsed_number = number.parse().ok()?;

    Some(parsed_number)
}

fn parse_color(lex: &mut Lexer<Token>) -> Option<u64> {
    let number = lex.slice().split_whitespace().nth(0)?;
    let parsed_number = number.parse().ok()?;

    Some(parsed_number)
}

fn solve_part1(input_path: &Path) -> Result<SolutionExecution, String> {
    trace!("Running part 1...");

    let file = File::open(input_path)
        .map_err(|e| format!("cannot open input file {}", e))?;
    let reader = BufReader::new(file);

    const MAX_RED_CUBES: u64 = 12;
    const MAX_GREEN_CUBES: u64 = 13;
    const MAX_BLUE_CUBES: u64 = 14;

    let mut result: u64 = 0;

    for line in reader.lines() {
        let line = line.map_err(|e| format!("cannot read line: {}", e))?;

        let mut game_id = None;

        for token in Token::lexer(&line) {
            let token = token.map_err(|_| {
                format!("Error while parsing the line: {}", line)
            })?;

            match token {
                Token::Game(id) => game_id = Some(id),
                Token::Red(cubes) => {
                    if cubes > MAX_RED_CUBES {
                        game_id = None;
                        break;
                    }
                }
                Token::Green(cubes) => {
                    if cubes > MAX_GREEN_CUBES {
                        game_id = None;
                        break;
                    }
                }
                Token::Blue(cubes) => {
                    if cubes > MAX_BLUE_CUBES {
                        game_id = None;
                        break;
                    }
                }
            }
        }

        if let Some(game_id) = game_id {
            result += game_id;
        }
    }

    Ok(SolutionExecution::Value(result))
}

fn solve_part2(input_path: &Path) -> Result<SolutionExecution, String> {
    trace!("Running part 2...");

    let file = File::open(input_path)
        .map_err(|e| format!("cannot open input file {}", e))?;
    let reader = BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        let line = line.map_err(|e| format!("cannot read line: {}", e))?;

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for token in Token::lexer(&line) {
            let token = token.map_err(|_| {
                format!("Error while parsing the line: {}", line)
            })?;

            match token {
                Token::Red(cubes) => {
                    if cubes > max_red {
                        max_red = cubes;
                    }
                }
                Token::Green(cubes) => {
                    if cubes > max_green {
                        max_green = cubes;
                    }
                }
                Token::Blue(cubes) => {
                    if cubes > max_blue {
                        max_blue = cubes;
                    }
                }
                Token::Game(_) => {}
            }
        }

        let power_cube = max_red * max_green * max_blue;
        result += power_cube;
    }

    Ok(SolutionExecution::Value(result))
}
