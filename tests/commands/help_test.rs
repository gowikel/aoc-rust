use assert_cmd::Command;
use chrono::{Datelike, Local};
use predicates::prelude::*;

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("aoc").unwrap();
    let current = Local::now();
    let current_year = current.year();
    let current_month = current.month();
    let current_day = current.day();

    let mut expected_year = current_year;
    let mut expected_day = current_day;

    if current_month != 12 {
        expected_year = current_year - 1;
    }

    if current_month != 12 || current_day > 25 {
        expected_day = 1;
    }

    let expected = format!(
        r#"My own solutions to the Advent of Code in Rust

Usage: aoc [OPTIONS] <COMMAND>

Commands:
  download  Downloads the specified puzzle input from AoC
  solve     Solve the specified puzzle
  generate  Generate the boilerplate code to solve the aforementioned challenge
  help      Print this message or the help of the given subcommand(s)

Options:
  -y, --year <YEAR>  Selected year. Defaults to current year on December, last year otherwise [default: {}]
  -d, --day <DAY>    Selected day. Defaults to current day on December between 1-25, 1 otherwise [default: {}]
  -h, --help         Print help
  -V, --version      Print version"#,
        expected_year, expected_day
    );

    cmd.arg("help")
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
}
