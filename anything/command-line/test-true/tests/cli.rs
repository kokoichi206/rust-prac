use assert_cmd::{assert::OutputAssertExt, Command};

#[test]
fn true_ok() {
    let cmd = Command::new("true").output().unwrap();
    cmd.assert().success();
}

#[test]
fn true_not_ok() {
    let cmd = Command::new("false").output().unwrap();
    cmd.assert().failure();
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
    cmd.assert().success().stdout("Hello, world!\n");
}
