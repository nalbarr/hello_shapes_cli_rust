use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn number_is_not_integer() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("shapes-cli")?;

    cmd.arg("--number").arg("abc");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid digit found in string"));

    Ok(())
}

#[test]
fn number_is_integer_5() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("shapes-cli")?;

    cmd.arg("--number").arg("5");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Created shapes with 5 shapes:"));

    Ok(())
}
