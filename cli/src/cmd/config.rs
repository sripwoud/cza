use super::Execute;
use crate::config::Config;
use crate::output;
use anyhow::{Context, Result};
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: Option<ConfigSubcommand>,
}

#[derive(Subcommand)]
pub enum ConfigSubcommand {
    /// Set a configuration value
    Set {
        /// Configuration key (e.g., user.author)
        key: String,
        /// Value to set
        value: String,
    },
    /// Get a configuration value
    Get {
        /// Configuration key (e.g., user.author)
        key: String,
    },
    /// List all configuration values
    List,
    /// Reset configuration to defaults
    Reset,
    /// Show the configuration file path
    Path,
}

pub struct ConfigCommand;

impl Execute for ConfigCommand {
    type Args = ConfigArgs;

    fn run(&self, args: &Self::Args) -> Result<()> {
        match &args.command {
            Some(ConfigSubcommand::Set { key, value }) => {
                let mut config = Config::load()?;
                config
                    .set(key, value)
                    .context(format!("Failed to set {}", key))?;
                config.save().context("Failed to save configuration")?;
                output::success(&format!("Set {} = {}", key, value));
            }
            Some(ConfigSubcommand::Get { key }) => {
                let config = Config::load()?;
                match config.get(key) {
                    Some(value) => output::info(&format!("{} = {}", key, value)),
                    None => output::warning(&format!("Configuration key '{}' not found", key)),
                }
            }
            Some(ConfigSubcommand::List) => {
                let config = Config::load()?;
                output::header("Configuration Values");
                for (key, value) in config.list() {
                    println!("  {} = {}", key, value);
                }
            }
            Some(ConfigSubcommand::Reset) => {
                let mut config = Config::load()?;
                config.reset();
                config.save().context("Failed to save configuration")?;
                output::success("Configuration reset to defaults");
            }
            Some(ConfigSubcommand::Path) => {
                let path = Config::config_path()?;
                output::info(&format!("Configuration file: {}", path.display()));
            }
            None => {
                // If no subcommand is provided, show the list
                let config = Config::load()?;
                output::header("Configuration Values");
                for (key, value) in config.list() {
                    println!("  {} = {}", key, value);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;

    #[test]
    fn test_config_set() {
        let temp_dir = TempDir::new().unwrap();
        env::set_var("XDG_CONFIG_HOME", temp_dir.path());

        let command = ConfigCommand;
        let args = ConfigArgs {
            command: Some(ConfigSubcommand::Set {
                key: "user.author".to_string(),
                value: "Test Author".to_string(),
            }),
        };

        let result = command.run(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_get() {
        let temp_dir = TempDir::new().unwrap();
        env::set_var("XDG_CONFIG_HOME", temp_dir.path());

        let command = ConfigCommand;

        // First set a value
        let set_args = ConfigArgs {
            command: Some(ConfigSubcommand::Set {
                key: "user.author".to_string(),
                value: "Test Author".to_string(),
            }),
        };
        command.run(&set_args).unwrap();

        // Then get it
        let get_args = ConfigArgs {
            command: Some(ConfigSubcommand::Get {
                key: "user.author".to_string(),
            }),
        };

        let result = command.run(&get_args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_list() {
        let temp_dir = TempDir::new().unwrap();
        env::set_var("XDG_CONFIG_HOME", temp_dir.path());

        let command = ConfigCommand;
        let args = ConfigArgs {
            command: Some(ConfigSubcommand::List),
        };

        let result = command.run(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_reset() {
        let temp_dir = TempDir::new().unwrap();
        env::set_var("XDG_CONFIG_HOME", temp_dir.path());

        let command = ConfigCommand;
        let args = ConfigArgs {
            command: Some(ConfigSubcommand::Reset),
        };

        let result = command.run(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_path() {
        let temp_dir = TempDir::new().unwrap();
        env::set_var("XDG_CONFIG_HOME", temp_dir.path());

        let command = ConfigCommand;
        let args = ConfigArgs {
            command: Some(ConfigSubcommand::Path),
        };

        let result = command.run(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_no_subcommand() {
        let temp_dir = TempDir::new().unwrap();
        env::set_var("XDG_CONFIG_HOME", temp_dir.path());

        let command = ConfigCommand;
        let args = ConfigArgs { command: None };

        let result = command.run(&args);
        assert!(result.is_ok());
    }
}
