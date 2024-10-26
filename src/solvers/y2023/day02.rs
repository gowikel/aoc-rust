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
    Game(u32),

    #[regex(r"[0-9]+ blue", parse_color)]
    Blue(u32),

    #[regex(r"[0-9]+ green", parse_color)]
    Green(u32),

    #[regex(r"[0-9]+ red", parse_color)]
    Red(u32),
}

fn parse_game(lex: &mut Lexer<Token>) -> Option<u32> {
    let number = lex.slice().split_whitespace().nth(1)?;
    let parsed_number = number.parse().ok()?;

    Some(parsed_number)
}

fn parse_color(lex: &mut Lexer<Token>) -> Option<u32> {
    let number = lex.slice().split_whitespace().nth(0)?;
    let parsed_number = number.parse().ok()?;

    Some(parsed_number)
}

fn solve_part1(input_path: &Path) -> Result<SolutionExecution, String> {
    trace!("Running part 1...");

    let file = File::open(input_path).map_err(|_| "cannot open input file")?;
    let reader = BufReader::new(file);

    const MAX_RED_CUBES: u32 = 12;
    const MAX_GREEN_CUBES: u32 = 13;
    const MAX_BLUE_CUBES: u32 = 14;

    let mut result: u32 = 0;

    for line in reader.lines() {
        let line = line.map_err(|_| "cannot read line")?;

        let mut game_id = None;

        for token in Token::lexer(&line) {
            let token = token.map_err(|_| "Error while parsing the line")?;

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

    Ok(SolutionExecution::NotImplemented)
}
