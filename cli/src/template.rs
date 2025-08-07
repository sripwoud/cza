use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::collections::HashMap;

/// Template registry containing all available templates
#[derive(Deserialize)]
pub struct TemplateRegistry {
    pub templates: HashMap<String, TemplateInfo>,
}

/// Information about a specific template
#[derive(Deserialize)]
pub struct TemplateInfo {
    pub name: String,
    pub description: String,
    pub repository: String,
    pub subfolder: String,
    pub frameworks: Vec<String>,
}

/// Load the embedded template registry from templates.toml
pub fn load_template_registry() -> Result<TemplateRegistry> {
    let templates_toml = include_str!("../templates.toml");
    toml::from_str(templates_toml).map_err(|e| anyhow!("Failed to parse template registry: {}", e))
}
