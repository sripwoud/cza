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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_template_registry() {
        let registry = load_template_registry();
        assert!(registry.is_ok());

        let registry = registry.unwrap();
        assert!(!registry.templates.is_empty());

        // Verify the noir-vite template exists (from our embedded templates.toml)
        assert!(registry.templates.contains_key("noir-vite"));

        let noir_template = &registry.templates["noir-vite"];
        assert_eq!(noir_template.name, "Noir + Vite + TanStack");
        assert!(!noir_template.description.is_empty());
        assert!(!noir_template.repository.is_empty());
        assert!(!noir_template.subfolder.is_empty());
        assert!(!noir_template.frameworks.is_empty());
    }

    #[test]
    fn test_template_info_structure() {
        let registry = load_template_registry().unwrap();
        let noir_template = &registry.templates["noir-vite"];

        // Test that all required fields are present and non-empty
        assert!(!noir_template.name.is_empty());
        assert!(!noir_template.description.is_empty());
        assert!(!noir_template.repository.is_empty());
        assert!(!noir_template.subfolder.is_empty());
        assert!(!noir_template.frameworks.is_empty());

        // Test that frameworks is a valid vector
        assert!(noir_template.frameworks.iter().all(|f| !f.is_empty()));
    }
}
