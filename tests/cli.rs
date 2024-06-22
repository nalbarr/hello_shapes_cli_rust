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
fn number_is_not_positive_integer() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("shapes-cli")?;

    cmd.arg("--number").arg("-123");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: unexpected argument '-1' found",
    ));

    Ok(())
}

#[test]
fn number_is_integer_3() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("shapes-cli")?;

    cmd.arg("--number").arg("3");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Created shapes with 3 shapes:"));

    Ok(())
}

#[test]
fn number_is_integer_10() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("shapes-cli")?;

    cmd.arg("--number").arg("10");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Created shapes with 10 shapes:"));

    Ok(())
}
