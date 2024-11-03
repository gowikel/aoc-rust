use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn test_aoc_env_missing() {
    let mut cmd = Command::cargo_bin("aoc").unwrap();

    cmd.arg("download")
        .env_remove("AOC_COOKIE")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "error: the following required arguments were not provided:",
        ));
}

#[test]
fn test_wrong_aoc_env_fails() {
    let mut cmd = Command::cargo_bin("aoc").unwrap();

    cmd.arg("download")
        .env("AOC_COOKIE", "wrong")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Error: FetchError(\"HTTP status client error (400 Bad Request)",
        ));
}

#[test]
fn test_download_happy_path() -> Result<(), &'static str> {
    dotenv::from_filename(".env.test")
        .map_err(|_| ".env.test is either not present or corrupted")?;

    let cookie = dotenv::var("AOC_COOKIE").map_err(|_| "AOC_COOKIE missing")?;
    let mut cmd = Command::cargo_bin("aoc").unwrap();

    cmd.arg("download")
        .env("AOC_COOKIE", cookie)
        .assert()
        .success()
        .stdout(predicate::str::is_match(".+").unwrap());

    Ok(())
}
