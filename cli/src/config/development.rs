use serde::{Deserialize, Serialize};

/// Development settings configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DevelopmentConfig {
    /// Default verbosity level
    #[serde(default)]
    pub verbose: bool,

    /// Enable/disable colored output (default: true)
    #[serde(default = "super::default_true")]
    pub color: bool,

    /// Ask before overwriting existing directories (default: true)
    #[serde(default = "super::default_true")]
    pub confirm_overwrite: bool,
}

impl Default for DevelopmentConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            color: true,
            confirm_overwrite: true,
        }
    }
}
