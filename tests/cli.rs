use assert_cmd::prelude::*; 
use predicates::prelude::*; 
use std::process::Command; 
use cli_shapes::create_shapes;

#[test]
fn number_is_integer() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("shapes-cli")?;

    cmd.arg("number").arg("abc");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("number must be integer."));

    Ok(())
}

#[test]
fn create_shapes() -> Vec<String> {
    let shapes = vec![
        "square".to_string(),
        "triangle".to_string(),
        "rectangle".to_string(),
    ];
    shapes = create_shapes(3);
    assert_eq!(shapes, shapes)
}

