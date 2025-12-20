use assert_cmd::prelude::*;
use escargot::CargoBuild;
use predicates::prelude::*;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;
use tempfile::TempDir;

static TEST_BINARY: OnceLock<PathBuf> = OnceLock::new();

fn get_test_binary() -> PathBuf {
    TEST_BINARY
        .get_or_init(|| {
            CargoBuild::new()
                .bin("cza")
                .current_release()
                .current_target()
                .run()
                .expect("Failed to build cza binary")
                .path()
                .to_path_buf()
        })
        .clone()
}

/// Integration tests for the cza CLI
/// These test the CLI as a black box through its command-line interface

#[test]
fn test_version_output() {
    let mut cmd = Command::new(get_test_binary());
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("cza "));
}

#[test]
fn test_help_output() {
    let mut cmd = Command::new(get_test_binary());
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "CLI tool for scaffolding zero-knowledge application",
        ))
        .stdout(predicate::str::contains("new"))
        .stdout(predicate::str::contains("list"));
}

#[test]
fn test_list_command() {
    let mut cmd = Command::new(get_test_binary());
    cmd.arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Available templates"))
        .stdout(predicate::str::contains("noir-vite"));
}

#[test]
fn test_list_json_command() {
    let mut cmd = Command::new(get_test_binary());
    cmd.args(["list", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("["))
        .stdout(predicate::str::contains("\"key\""))
        .stdout(predicate::str::contains("\"name\""))
        .stdout(predicate::str::contains("\"noir-vite\""));
}

#[test]
fn test_invalid_template() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::new(get_test_binary());
    cmd.current_dir(&temp_dir)
        .args(["new", "test-project", "--template", "nonexistent-template"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn test_invalid_project_name() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::new(get_test_binary());
    cmd.current_dir(&temp_dir)
        .args(["new", "invalid name", "--template", "noir-vite"])
        .assert()
        .failure();
}

#[test]
fn test_missing_arguments() {
    let mut cmd = Command::new(get_test_binary());
    cmd.arg("new")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_config_command_help() {
    let mut cmd = Command::new(get_test_binary());
    cmd.args(["config", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Configure global settings"));
}

#[test]
fn test_config_set_command() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_var("XDG_CONFIG_HOME", temp_dir.path());

    let mut cmd = Command::new(get_test_binary());
    cmd.env("XDG_CONFIG_HOME", temp_dir.path())
        .args(["config", "set", "user.author", "Test Author"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Set user.author = Test Author"));
}

#[test]
fn test_config_get_command() {
    let temp_dir = TempDir::new().unwrap();

    // First set a value
    let mut cmd = Command::new(get_test_binary());
    cmd.env("XDG_CONFIG_HOME", temp_dir.path())
        .args(["config", "set", "user.author", "Test Author"])
        .assert()
        .success();

    // Then get it
    let mut cmd = Command::new(get_test_binary());
    cmd.env("XDG_CONFIG_HOME", temp_dir.path())
        .args(["config", "get", "user.author"])
        .assert()
        .success()
        .stdout(predicate::str::contains("user.author = Test Author"));
}

#[test]
fn test_update_command() {
    let mut cmd = Command::new(get_test_binary());
    cmd.arg("update")
        .assert()
        .success()
        .stdout(predicate::str::contains("Checking for updates"));
}

#[test]
fn test_update_command_help() {
    let mut cmd = Command::new(get_test_binary());
    cmd.args(["update", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Update the CLI tool to the latest version",
        ));
}

#[test]
fn test_new_command_no_git_flag() {
    let mut cmd = Command::new(get_test_binary());
    cmd.args(["new", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--no-git"));
}
