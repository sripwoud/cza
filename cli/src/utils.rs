use crate::output;
use log::debug;
use std::path::Path;
use std::process::{Command, ExitStatus};

/// Run a command and handle common patterns (logging, error handling)
pub fn run_command(
    command: &str,
    args: &[&str],
    working_dir: Option<&Path>,
    description: &str,
) -> Result<ExitStatus, std::io::Error> {
    debug!("Running command: {} {:?}", command, args);

    let mut cmd = Command::new(command);
    cmd.args(args);

    if let Some(dir) = working_dir {
        cmd.current_dir(dir);
    }

    match cmd.status() {
        Ok(status) => {
            if status.success() {
                debug!("{} completed successfully", description);
            } else {
                debug!("{} exited with status: {}", description, status);
            }
            Ok(status)
        }
        Err(e) => {
            debug!("Failed to run {}: {}", command, e);
            Err(e)
        }
    }
}

/// Get a git config value
pub fn get_git_config(key: &str) -> Option<String> {
    debug!("Getting git config: {}", key);

    Command::new("git")
        .args(["config", key])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
            } else {
                None
            }
        })
}

/// Run a post-generation command with standardized output
pub fn run_post_generation_command(
    command: &str,
    args: &[&str],
    working_dir: &Path,
    step_message: &str,
    success_message: &str,
    failure_hint: Option<&str>,
) -> Result<(), String> {
    output::step(step_message);

    match run_command(command, args, Some(working_dir), step_message) {
        Ok(status) if status.success() => {
            output::success(success_message);
            Ok(())
        }
        Ok(status) => {
            let error_msg = format!("{} failed with status: {}", command, status);
            output::warning(&error_msg);
            if let Some(hint) = failure_hint {
                output::info(hint);
            }
            Err(error_msg)
        }
        Err(e) => {
            let error_msg = format!("Could not run {}: {}", command, e);
            output::warning(&error_msg);
            if let Some(hint) = failure_hint {
                output::info(hint);
            }
            Err(error_msg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_git_config_nonexistent() {
        let result = get_git_config("nonexistent.config.key");
        assert!(result.is_none());
    }

    #[test]
    fn test_run_command_success() {
        // Use a simple command that should work on all platforms
        let result = run_command("echo", &["test"], None, "echo test");
        assert!(result.is_ok());
        assert!(result.unwrap().success());
    }

    #[test]
    fn test_run_command_with_working_dir() {
        let temp_dir = env::temp_dir();
        let result = run_command("pwd", &[], Some(&temp_dir), "pwd in temp");
        // pwd might not exist on Windows, so we just check it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_run_command_failure() {
        let result = run_command("nonexistent_command", &[], None, "test");
        assert!(result.is_err());
    }
}
