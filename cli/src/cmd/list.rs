use super::Execute;
use crate::output;
use anyhow::{anyhow, Result};
use clap::Args;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Args)]
pub struct ListArgs {
    /// Show detailed information about templates
    #[arg(long)]
    detailed: bool,
}

#[derive(Deserialize)]
struct TemplateRegistry {
    templates: HashMap<String, TemplateInfo>,
}

#[derive(Deserialize)]
struct TemplateInfo {
    name: String,
    description: String,
    repository: String,
    subfolder: String,
    frameworks: Vec<String>,
}

pub struct ListCommand;

impl Execute for ListCommand {
    type Args = ListArgs;

    fn run(&self, args: &Self::Args) -> Result<()> {
        // Load embedded template registry
        let registry = self.load_template_registry()?;

        if registry.templates.is_empty() {
            output::warning("No templates available.");
            return Ok(());
        }

        output::header("Available templates");

        // Sort templates by name for consistent output
        let mut templates: Vec<_> = registry.templates.iter().collect();
        templates.sort_by_key(|(key, _)| *key);

        for (template_key, template_info) in templates {
            if args.detailed {
                // Build full URL to template subfolder
                let template_url = if template_info.repository.contains("github.com") {
                    format!(
                        "{}/tree/main/{}",
                        template_info.repository, template_info.subfolder
                    )
                } else {
                    template_info.repository.clone()
                };
                output::template_detailed(
                    template_key,
                    &template_info.name,
                    &template_info.description,
                    &template_info.frameworks,
                    &template_url,
                );
            } else {
                output::template_item(template_key, &template_info.description);
            }
        }

        if !args.detailed {
            output::info("Use 'cza list --detailed' for more information about templates.");
        }

        output::header("To create a new project");
        output::command_example("General syntax", "cza new <template> <project-name>");
        output::command_example("Example", "cza new noir-vite my-zk-app");

        Ok(())
    }
}

impl ListCommand {
    fn load_template_registry(&self) -> Result<TemplateRegistry> {
        // Load embedded templates.toml
        let templates_toml = include_str!("../../templates.toml");
        toml::from_str(templates_toml)
            .map_err(|e| anyhow!("Failed to parse template registry: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_command_execute() {
        let cmd = ListCommand;
        let args = ListArgs { detailed: false };
        // Should not panic and should return Ok
        assert!(cmd.run(&args).is_ok());
    }

    #[test]
    fn test_list_detailed_command_execute() {
        let cmd = ListCommand;
        let args = ListArgs { detailed: true };
        // Should not panic and should return Ok
        assert!(cmd.run(&args).is_ok());
    }

    #[test]
    fn test_template_registry_loading() {
        let cmd = ListCommand;
        let registry = cmd.load_template_registry().unwrap();

        // Should have at least the noir-vite template
        assert!(registry.templates.contains_key("noir-vite"));

        let noir_template = &registry.templates["noir-vite"];
        assert!(!noir_template.name.is_empty());
        assert!(!noir_template.description.is_empty());
        assert!(!noir_template.repository.is_empty());
        assert!(!noir_template.frameworks.is_empty());
    }

    #[test]
    fn test_template_info_structure() {
        let template = TemplateInfo {
            name: "Test Template".to_string(),
            description: "A test template".to_string(),
            repository: "https://github.com/test/test".to_string(),
            subfolder: "test-template".to_string(),
            frameworks: vec!["test".to_string(), "framework".to_string()],
        };

        // Test that template has expected properties
        assert_eq!(template.name, "Test Template");
        assert_eq!(template.frameworks.len(), 2);
        assert!(template.frameworks.contains(&"test".to_string()));
    }

    #[test]
    fn test_templates_sorting() {
        let cmd = ListCommand;
        let registry = cmd.load_template_registry().unwrap();

        // Collect template keys and verify they can be sorted
        let mut template_keys: Vec<_> = registry.templates.keys().collect();
        template_keys.sort();

        // Should be sortable without panic
        assert!(!template_keys.is_empty());
    }
}
