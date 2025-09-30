use super::Execute;
use crate::{output, template};
use anyhow::Result;
use clap::Args;
use log::{debug, warn};
use serde::Serialize;

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Show detailed information about templates
    #[arg(long)]
    detailed: bool,

    /// Output templates as JSON
    #[arg(long)]
    json: bool,
}

#[derive(Serialize)]
struct JsonTemplate {
    key: String,
    name: String,
    description: String,
    repository: String,
    subfolder: String,
    frameworks: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<String>,
}

pub struct ListCommand;

impl Execute for ListCommand {
    type Args = ListArgs;

    fn run(&self, args: &Self::Args) -> Result<()> {
        debug!(
            "Starting list command with detailed: {}, json: {}",
            args.detailed, args.json
        );

        // Load embedded template registry
        debug!("Loading embedded template registry");
        let registry = template::load_template_registry()?;
        debug!("Found {} templates", registry.templates.len());

        if registry.templates.is_empty() {
            warn!("No templates available in registry");
            if args.json {
                println!("[]");
            } else {
                output::warning("No templates available.");
            }
            return Ok(());
        }

        // Sort templates by name for consistent output
        let mut templates: Vec<_> = registry.templates.iter().collect();
        templates.sort_by_key(|(key, _)| *key);

        // Handle JSON output
        if args.json {
            let json_templates: Vec<JsonTemplate> = templates
                .iter()
                .map(|(key, info)| JsonTemplate {
                    key: (*key).clone(),
                    name: info.name.clone(),
                    description: info.description.clone(),
                    repository: info.repository.clone(),
                    subfolder: info.subfolder.clone(),
                    frameworks: info.frameworks.clone(),
                    revision: info.revision.clone(),
                })
                .collect();

            let json_output = serde_json::to_string_pretty(&json_templates)?;
            println!("{}", json_output);
            return Ok(());
        }

        // Regular formatted output
        output::header("Available templates");

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
                // Show pinned revision if present
                if let Some(ref revision) = template_info.revision {
                    output::info(&format!("    📌 Pinned to: {}", revision));
                }
            } else {
                output::template_item(template_key, &template_info.description);
                // Show pinned indicator in summary view
                if template_info.revision.is_some() {
                    output::info("      📌 (pinned)");
                }
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

impl ListCommand {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_command_execute() {
        let cmd = ListCommand;
        let args = ListArgs {
            detailed: false,
            json: false,
        };
        // Should not panic and should return Ok
        assert!(cmd.run(&args).is_ok());
    }

    #[test]
    fn test_list_detailed_command_execute() {
        let cmd = ListCommand;
        let args = ListArgs {
            detailed: true,
            json: false,
        };
        // Should not panic and should return Ok
        assert!(cmd.run(&args).is_ok());
    }

    #[test]
    fn test_list_json_command_execute() {
        let cmd = ListCommand;
        let args = ListArgs {
            detailed: false,
            json: true,
        };
        // Should not panic and should return Ok
        assert!(cmd.run(&args).is_ok());
    }

    #[test]
    fn test_json_output_format() {
        // Capture stdout
        let cmd = ListCommand;
        let args = ListArgs {
            detailed: false,
            json: true,
        };

        // Just verify command executes successfully
        // Full output validation would require capturing stdout
        assert!(cmd.run(&args).is_ok());

        // Verify JSON template structure can be serialized
        let test_template = JsonTemplate {
            key: "test-template".to_string(),
            name: "Test Template".to_string(),
            description: "A test template".to_string(),
            repository: "https://github.com/test/repo".to_string(),
            subfolder: "test".to_string(),
            frameworks: vec!["noir".to_string(), "vite".to_string()],
            revision: None,
        };

        // Should serialize without error
        let json_str = serde_json::to_string(&test_template).unwrap();
        assert!(json_str.contains("test-template"));
        assert!(json_str.contains("Test Template"));

        // Should be valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["key"], "test-template");
        assert_eq!(parsed["frameworks"][0], "noir");
    }

    #[test]
    fn test_template_registry_loading() {
        let registry = template::load_template_registry().unwrap();

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
        let template = template::TemplateInfo {
            name: "Test Template".to_string(),
            description: "A test template".to_string(),
            repository: "https://github.com/test/test".to_string(),
            subfolder: "test-template".to_string(),
            frameworks: vec!["test".to_string(), "framework".to_string()],
            revision: None,
        };

        // Test that template has expected properties
        assert_eq!(template.name, "Test Template");
        assert_eq!(template.frameworks.len(), 2);
        assert!(template.frameworks.contains(&"test".to_string()));
    }

    #[test]
    fn test_templates_sorting() {
        let registry = template::load_template_registry().unwrap();

        // Collect template keys and verify they can be sorted
        let mut template_keys: Vec<_> = registry.templates.keys().collect();
        template_keys.sort();

        // Should be sortable without panic
        assert!(!template_keys.is_empty());
    }
}
