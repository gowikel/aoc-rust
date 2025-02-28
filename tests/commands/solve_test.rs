use crate::fixtures;
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_without_parameters() {
    let mut cmd = Command::cargo_bin("aoc").unwrap();

    cmd.arg("solve")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "error: the following required arguments were not provided:
  <PUZZLE_INPUT>

Usage: aoc solve <PUZZLE_INPUT> [EXECUTE]

For more information, try '--help'.",
        ));
}

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("aoc").unwrap();

    cmd.arg("solve")
  .arg("--help")
  .assert()
  .success()
  .stdout(predicate::str::contains("Solve the specified puzzle

Usage: aoc solve [OPTIONS] <PUZZLE_INPUT> [EXECUTE]

Arguments:
  <PUZZLE_INPUT>  
  [EXECUTE]       [default: all] [possible values: all, p1, p2]

Options:
  -s, --style <STYLE>  Control how the results are displayed [default: tabulated] [possible values: simple, tabulated]
  -y, --year <YEAR>    Selected year. Defaults to current year on December, last year otherwise [default: 2024]
  -d, --day <DAY>      Selected day. Defaults to current day on December between 1-25, 1 otherwise [default: 1]
      --no-color       Removes the color from the ouput [default: false]
  -h, --help           Print help"));
}

#[test]
fn test_happy_path() {
    let mut cmd = Command::cargo_bin("aoc").unwrap();
    let data_path = fixtures::get_data_path("y2024/01.txt");

    cmd.arg("solve")
        .arg("--year")
        .arg("2024")
        .arg("--day")
        .arg("1")
        .arg(data_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Solutions 2024/01"))
        .stdout(predicate::str::contains("P1"))
        .stdout(predicate::str::contains("P2"))
        .stdout(predicate::str::contains("Time elapsed:"));
}

#[test]
fn test_invalid_execute_option() {
    let mut cmd = Command::cargo_bin("aoc").unwrap();
    let data_path = fixtures::get_data_path("y2024/01.txt");

    cmd.arg("solve")
        .arg("--year")
        .arg("2024")
        .arg("--day")
        .arg("1")
        .arg(data_path)
        .arg("INVALID_EXEC_OPTION")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "error: invalid value 'INVALID_EXEC_OPTION' for '[EXECUTE]'",
        ))
        .stderr(predicate::str::contains("[possible values: all, p1, p2]"));
}

#[test]
fn test_p1_execution_only() {
    let mut cmd = Command::cargo_bin("aoc").unwrap();
    let data_path = fixtures::get_data_path("y2024/01.txt");

    cmd.arg("solve")
        .arg("--year")
        .arg("2024")
        .arg("--day")
        .arg("1")
        .arg(data_path)
        .arg("p1")
        .assert()
        .success()
        .stdout(predicate::str::contains("P1"))
        .stdout(predicate::str::contains(
            "\u{1b}[1mP2\u{1b}[0m │ Not executed ",
        ))
        .stdout(predicate::str::contains("Time elapsed:"));
}

#[test]
fn test_p2_execution_only() {
    let mut cmd = Command::cargo_bin("aoc").unwrap();
    let data_path = fixtures::get_data_path("y2024/01.txt");

    cmd.arg("solve")
        .arg("--year")
        .arg("2024")
        .arg("--day")
        .arg("1")
        .arg(data_path)
        .arg("p2")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "\u{1b}[1mP1\u{1b}[0m │ Not executed ",
        ))
        .stdout(predicate::str::contains("P2"))
        .stdout(predicate::str::contains("Time elapsed:"));
}
