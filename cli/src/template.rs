//! Template registry and validation
//!
//! This module manages the embedded template registry loaded from `templates.toml`.
//! It provides functionality to:
//!
//! - Load available templates from the embedded registry
//! - Validate template configuration
//! - Check system prerequisites (git availability)
//!
//! ## Template Structure
//!
//! Each template in the registry contains:
//! - Name and description
//! - Git repository URL
//! - Subfolder path within the repository
//! - Associated ZK frameworks
//!
//! ## Example
//!
//! ```no_run
//! use cza::template::{load_template_registry, validate_template};
//!
//! let registry = load_template_registry()?;
//! if let Some(template) = registry.templates.get("noir-vite") {
//!     validate_template(template)?;
//!     println!("Template {} is valid", template.name);
//! }
//! # Ok::<(), anyhow::Error>(())
//! ```

use anyhow::{anyhow, Result};
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

/// Template registry containing all available templates
#[derive(Deserialize)]
pub struct TemplateRegistry {
    /// Map of template keys to template information
    pub templates: HashMap<String, TemplateInfo>,
}

/// Information about a specific template
#[derive(Deserialize, Serialize)]
pub struct TemplateInfo {
    /// Display name of the template
    pub name: String,
    /// Description of what the template provides
    pub description: String,
    /// Git repository URL
    pub repository: String,
    /// Subfolder path within the repository
    pub subfolder: String,
    /// ZK frameworks included in the template
    pub frameworks: Vec<String>,
}

/// Load the embedded template registry from templates.toml
pub fn load_template_registry() -> Result<TemplateRegistry> {
    let templates_toml = include_str!("../templates.toml");
    toml::from_str(templates_toml).map_err(|e| anyhow!("Failed to parse template registry: {}", e))
}

/// Validate that a template's repository and subfolder exist
pub fn validate_template(template_info: &TemplateInfo) -> Result<()> {
    debug!(
        "Validating template repository: {} subfolder: {}",
        template_info.repository, template_info.subfolder
    );

    // For now, we'll do a basic validation by checking if the repository URL looks valid
    // In the future, we could add more sophisticated validation like checking if the repo exists
    // and if the subfolder exists within it

    if template_info.repository.is_empty() {
        return Err(anyhow!("Template repository URL cannot be empty"));
    }

    if template_info.subfolder.is_empty() {
        return Err(anyhow!("Template subfolder cannot be empty"));
    }

    // Basic URL validation - must contain github.com or be a valid git URL
    if !template_info.repository.contains("github.com")
        && !template_info.repository.starts_with("git@")
        && !template_info.repository.starts_with("https://")
    {
        return Err(anyhow!("Template repository must be a valid git URL"));
    }

    debug!("Template validation passed for {}", template_info.name);
    Ok(())
}

/// Check if git is available on the system
pub fn check_git_available() -> bool {
    debug!("Checking if git is available");

    Command::new("git")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
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

    #[test]
    fn test_validate_template_valid() {
        let valid_template = TemplateInfo {
            name: "Test Template".to_string(),
            description: "A test template".to_string(),
            repository: "https://github.com/test/test".to_string(),
            subfolder: "test-template".to_string(),
            frameworks: vec!["test".to_string()],
        };

        assert!(validate_template(&valid_template).is_ok());
    }

    #[test]
    fn test_validate_template_empty_repository() {
        let invalid_template = TemplateInfo {
            name: "Test Template".to_string(),
            description: "A test template".to_string(),
            repository: "".to_string(),
            subfolder: "test-template".to_string(),
            frameworks: vec!["test".to_string()],
        };

        let result = validate_template(&invalid_template);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("repository URL cannot be empty"));
    }

    #[test]
    fn test_validate_template_empty_subfolder() {
        let invalid_template = TemplateInfo {
            name: "Test Template".to_string(),
            description: "A test template".to_string(),
            repository: "https://github.com/test/test".to_string(),
            subfolder: "".to_string(),
            frameworks: vec!["test".to_string()],
        };

        let result = validate_template(&invalid_template);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("subfolder cannot be empty"));
    }

    #[test]
    fn test_validate_template_invalid_url() {
        let invalid_template = TemplateInfo {
            name: "Test Template".to_string(),
            description: "A test template".to_string(),
            repository: "invalid-url".to_string(),
            subfolder: "test-template".to_string(),
            frameworks: vec!["test".to_string()],
        };

        let result = validate_template(&invalid_template);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("must be a valid git URL"));
    }

    #[test]
    fn test_validate_template_github_url() {
        let github_template = TemplateInfo {
            name: "GitHub Template".to_string(),
            description: "A GitHub template".to_string(),
            repository: "https://github.com/user/repo".to_string(),
            subfolder: "template".to_string(),
            frameworks: vec!["test".to_string()],
        };

        assert!(validate_template(&github_template).is_ok());
    }

    #[test]
    fn test_validate_template_git_ssh_url() {
        let ssh_template = TemplateInfo {
            name: "SSH Template".to_string(),
            description: "An SSH template".to_string(),
            repository: "git@github.com:user/repo.git".to_string(),
            subfolder: "template".to_string(),
            frameworks: vec!["test".to_string()],
        };

        assert!(validate_template(&ssh_template).is_ok());
    }

    #[test]
    fn test_check_git_available() {
        // This test checks if git is available on the system
        // The result may vary depending on the test environment
        let git_available = check_git_available();
        // We don't assert the result since git may or may not be available
        // Just ensure the function doesn't panic
        println!("Git available: {}", git_available);
    }
}
