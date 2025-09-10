use serde::{Deserialize, Serialize};

/// Post-generation behavior configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostGenerationConfig {
    /// Run `mise install` automatically (default: true)
    #[serde(default = "super::default_true")]
    pub auto_install_deps: bool,

    /// Run `hk install` automatically (default: true)
    #[serde(default = "super::default_true")]
    pub auto_setup_hooks: bool,

    /// Automatically open project in editor after creation
    pub open_editor: Option<String>,
}

impl Default for PostGenerationConfig {
    fn default() -> Self {
        Self {
            auto_install_deps: true,
            auto_setup_hooks: true,
            open_editor: None,
        }
    }
}
