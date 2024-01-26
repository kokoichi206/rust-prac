use std::fs;

use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

// input files
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const JAPANESE: &str = "tests/inputs/japanese.txt";

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("wcr")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt")
}

#[test]
fn fox() -> TestResult {
    run(&[FOX], "tests/expected/fox.txt")
}

#[test]
fn japanese() -> TestResult {
    // wow, ok...
    run(&[JAPANESE], "tests/expected/japanese.txt")
}
