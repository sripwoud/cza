//! Configuration management for cza
//!
//! This module handles loading, saving, and managing user configuration stored in `~/.config/cza/config.toml`.
//!
//! ## Configuration Structure
//!
//! The configuration is divided into three main sections:
//!
//! - [`UserConfig`] - User preferences (author, email, default template, git initialization)
//! - [`DevelopmentConfig`] - Development settings (verbose logging, color output, overwrite confirmation)
//! - [`PostGenerationConfig`] - Post-generation behavior (auto-install deps, auto-setup hooks, open editor)
//!
//! ## Example
//!
//! ```no_run
//! use cza::config::Config;
//!
//! let mut config = Config::load()?;
//! config.set("user.author", "John Doe")?;
//! config.save()?;
//! # Ok::<(), anyhow::Error>(())
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

mod development;
mod post_generation;
mod user;

pub use development::DevelopmentConfig;
pub use post_generation::PostGenerationConfig;
pub use user::UserConfig;

/// Helper function for serde default values
pub(crate) fn default_true() -> bool {
    true
}

/// Main configuration structure for cza
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    /// User preferences
    #[serde(default)]
    pub user: UserConfig,

    /// Development settings
    #[serde(default)]
    pub development: DevelopmentConfig,

    /// Post-generation behavior
    #[serde(default)]
    pub post_generation: PostGenerationConfig,
}

impl Config {
    /// Get the configuration file path
    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Could not find config directory")?
            .join("cza");
        Ok(config_dir.join("config.toml"))
    }

    /// Load configuration from disk or create default
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let contents =
                fs::read_to_string(&config_path).context("Failed to read config file")?;
            toml::from_str(&contents).context("Failed to parse config file")
        } else {
            Ok(Self::default())
        }
    }

    /// Save configuration to disk
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        // Ensure the config directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        let contents = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&config_path, contents).context("Failed to write config file")?;

        Ok(())
    }

    /// Get a configuration value by key path (e.g., "user.author")
    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "user.author" => self.user.author.clone(),
            "user.email" => self.user.email.clone(),
            "user.git_init" => Some(self.user.git_init.to_string()),
            "user.default_template" => self.user.default_template.clone(),
            "development.verbose" => Some(self.development.verbose.to_string()),
            "development.color" => Some(self.development.color.to_string()),
            "development.confirm_overwrite" => Some(self.development.confirm_overwrite.to_string()),
            "post_generation.auto_install_deps" => {
                Some(self.post_generation.auto_install_deps.to_string())
            }
            "post_generation.auto_setup_hooks" => {
                Some(self.post_generation.auto_setup_hooks.to_string())
            }
            "post_generation.open_editor" => self.post_generation.open_editor.clone(),
            _ => None,
        }
    }

    /// Set a configuration value by key path
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "user.author" => self.user.author = Some(value.to_string()),
            "user.email" => self.user.email = Some(value.to_string()),
            "user.git_init" => {
                self.user.git_init = value.parse().context("Invalid boolean value")?
            }
            "user.default_template" => self.user.default_template = Some(value.to_string()),
            "development.verbose" => {
                self.development.verbose = value.parse().context("Invalid boolean value")?
            }
            "development.color" => {
                self.development.color = value.parse().context("Invalid boolean value")?
            }
            "development.confirm_overwrite" => {
                self.development.confirm_overwrite =
                    value.parse().context("Invalid boolean value")?
            }
            "post_generation.auto_install_deps" => {
                self.post_generation.auto_install_deps =
                    value.parse().context("Invalid boolean value")?
            }
            "post_generation.auto_setup_hooks" => {
                self.post_generation.auto_setup_hooks =
                    value.parse().context("Invalid boolean value")?
            }
            "post_generation.open_editor" => {
                self.post_generation.open_editor = Some(value.to_string())
            }
            _ => anyhow::bail!("Unknown configuration key: {}", key),
        }
        Ok(())
    }

    /// Reset configuration to defaults
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// List all configuration values
    pub fn list(&self) -> Vec<(String, String)> {
        vec![
            (
                "user.author".to_string(),
                self.user
                    .author
                    .clone()
                    .unwrap_or_else(|| "<not set>".to_string()),
            ),
            (
                "user.email".to_string(),
                self.user
                    .email
                    .clone()
                    .unwrap_or_else(|| "<not set>".to_string()),
            ),
            ("user.git_init".to_string(), self.user.git_init.to_string()),
            (
                "user.default_template".to_string(),
                self.user
                    .default_template
                    .clone()
                    .unwrap_or_else(|| "<not set>".to_string()),
            ),
            (
                "development.verbose".to_string(),
                self.development.verbose.to_string(),
            ),
            (
                "development.color".to_string(),
                self.development.color.to_string(),
            ),
            (
                "development.confirm_overwrite".to_string(),
                self.development.confirm_overwrite.to_string(),
            ),
            (
                "post_generation.auto_install_deps".to_string(),
                self.post_generation.auto_install_deps.to_string(),
            ),
            (
                "post_generation.auto_setup_hooks".to_string(),
                self.post_generation.auto_setup_hooks.to_string(),
            ),
            (
                "post_generation.open_editor".to_string(),
                self.post_generation
                    .open_editor
                    .clone()
                    .unwrap_or_else(|| "<not set>".to_string()),
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;
    use tempfile::TempDir;

    // Mutex to serialize config tests to avoid environment variable conflicts
    static CONFIG_TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.user.git_init);
        assert!(config.development.color);
        assert!(config.development.confirm_overwrite);
        assert!(config.post_generation.auto_install_deps);
        assert!(config.post_generation.auto_setup_hooks);
        assert!(!config.development.verbose);
    }

    #[test]
    fn test_get_config_values() {
        let mut config = Config::default();
        config.user.author = Some("Test Author".to_string());
        config.user.email = Some("test@example.com".to_string());

        assert_eq!(config.get("user.author"), Some("Test Author".to_string()));
        assert_eq!(
            config.get("user.email"),
            Some("test@example.com".to_string())
        );
        assert_eq!(config.get("user.git_init"), Some("true".to_string()));
        assert_eq!(config.get("invalid.key"), None);
    }

    #[test]
    fn test_set_config_values() {
        let mut config = Config::default();

        config.set("user.author", "New Author").unwrap();
        assert_eq!(config.user.author, Some("New Author".to_string()));

        config.set("user.git_init", "false").unwrap();
        assert!(!config.user.git_init);

        config.set("development.verbose", "true").unwrap();
        assert!(config.development.verbose);

        let result = config.set("invalid.key", "value");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_config_values() {
        let mut config = Config::default();
        config.user.author = Some("Test Author".to_string());

        let list = config.list();
        assert_eq!(list.len(), 10);

        let author = list.iter().find(|(k, _)| k == "user.author");
        assert_eq!(
            author,
            Some(&("user.author".to_string(), "Test Author".to_string()))
        );
    }

    #[test]
    fn test_save_and_load_config() {
        let _lock = CONFIG_TEST_MUTEX.lock().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("cza").join("config.toml");

        // For this test, we need to manually set the standard var temporarily
        let original_config_home = env::var("XDG_CONFIG_HOME").ok();
        env::set_var("XDG_CONFIG_HOME", temp_dir.path());

        let mut config = Config::default();
        config.user.author = Some("Test Author".to_string());
        config.user.email = Some("test@example.com".to_string());
        config.save().unwrap();

        assert!(config_path.exists());

        let loaded = Config::load().unwrap();
        assert_eq!(loaded.user.author, Some("Test Author".to_string()));
        assert_eq!(loaded.user.email, Some("test@example.com".to_string()));

        // Restore original environment
        match original_config_home {
            Some(original) => env::set_var("XDG_CONFIG_HOME", original),
            None => env::remove_var("XDG_CONFIG_HOME"),
        }
    }

    #[test]
    fn test_reset_config() {
        let mut config = Config::default();
        config.user.author = Some("Test Author".to_string());
        config.development.verbose = true;

        config.reset();

        assert_eq!(config.user.author, None);
        assert!(!config.development.verbose);
        assert!(config.user.git_init); // Should be back to default true
    }
}
