use serde::{Deserialize, Serialize};

/// User preferences configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserConfig {
    /// Default author name for new projects
    pub author: Option<String>,

    /// Default email for project metadata
    pub email: Option<String>,

    /// Whether to auto-initialize git repos (default: true)
    #[serde(default = "super::default_true")]
    pub git_init: bool,

    /// Preferred template when not specified
    pub default_template: Option<String>,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            author: None,
            email: None,
            git_init: true,
            default_template: None,
        }
    }
}
