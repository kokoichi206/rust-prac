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

#[test]
fn all() -> TestResult {
    // output includes the total line
    run(&[EMPTY, FOX, JAPANESE], "tests/expected/all.txt")
}

#[test]
fn fox_c() -> TestResult {
    run(&[FOX, "-c"], "tests/expected/fox.c.txt")
}

#[test]
fn japanese_l() -> TestResult {
    run(&[JAPANESE, "-l"], "tests/expected/japanese.l.txt")
}

#[test]
fn stdin() -> TestResult {
    // $ echo 'hoge pien' | wc
    //    1       2      10
    let expected = String::from("       1       2      10\n");
    Command::cargo_bin("wcr")?
        .write_stdin("hoge pien\n")
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn stdin_c() -> TestResult {
    // $ echo -n 'hoge pien' | wc -c
    //    9
    let expected = String::from("       9\n");
    Command::cargo_bin("wcr")?
        .arg("-c")
        .write_stdin("hoge pien")
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn stdin_l_no_line() -> TestResult {
    // $ echo -n 'hoge pien' | wc -l
    //    0
    let expected = String::from("       0\n");
    Command::cargo_bin("wcr")?
        .arg("-l")
        .write_stdin("hoge pien")
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn stdin_l() -> TestResult {
    // $ echo 'hoge pien' | wc -l
    //    1
    let expected = String::from("       1\n");
    Command::cargo_bin("wcr")?
        .arg("-l")
        .write_stdin("hoge pien\n")
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
