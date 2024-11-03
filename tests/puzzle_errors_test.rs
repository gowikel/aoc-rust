use aoc::constants::{VALID_DAYS, VALID_YEARS};
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn invalid_year_test() {
    let mut cmd = Command::cargo_bin("aoc").unwrap();

    let expected_error = format!(
        "Error: InvalidYear(\"year should be in [{}-{}] range. Current: 1900.\")",
        VALID_YEARS.start(),
        VALID_YEARS.end()
    );

    cmd.arg("--year")
        .arg("1900")
        .arg("download")
        .arg("--aoc-cookie")
        .arg("dummy-cookie")
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected_error));
}

#[test]
fn invalid_day_test() {
    let mut cmd = Command::cargo_bin("aoc").unwrap();

    let expected_error = format!(
        "Error: InvalidDay(\"day should be in [{}-{}] range. Current: 26].\")",
        VALID_DAYS.start(),
        VALID_DAYS.end()
    );

    cmd.arg("--day")
        .arg("26") // First invalid day after the valid range
        .arg("download")
        .arg("--aoc-cookie")
        .arg("dummy-cookie")
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected_error));
}
