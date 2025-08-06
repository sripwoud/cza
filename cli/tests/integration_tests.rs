use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::TempDir;

/// Integration tests for the cza CLI
/// These test the CLI as a black box through its command-line interface

#[test]
fn test_version_output() {
    let mut cmd = Command::cargo_bin("cza").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("create-zk-app "));
}

#[test]
fn test_help_output() {
    let mut cmd = Command::cargo_bin("cza").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "CLI tool to create zero-knowledge",
        ))
        .stdout(predicate::str::contains("new"))
        .stdout(predicate::str::contains("list"));
}

#[test]
fn test_list_command() {
    let mut cmd = Command::cargo_bin("cza").unwrap();
    cmd.arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Available templates"))
        .stdout(predicate::str::contains("noir-vite"));
}

#[test]
fn test_invalid_template() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("cza").unwrap();
    cmd.current_dir(&temp_dir)
        .args(["new", "nonexistent-template", "test-project"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn test_invalid_project_name() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("cza").unwrap();
    cmd.current_dir(&temp_dir)
        .args(["new", "noir-vite", "invalid name"])
        .assert()
        .failure();
}

#[test]
fn test_missing_arguments() {
    let mut cmd = Command::cargo_bin("cza").unwrap();
    cmd.arg("new")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}
