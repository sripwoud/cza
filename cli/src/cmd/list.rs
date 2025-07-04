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
                output::template_detailed(
                    template_key,
                    &template_info.name,
                    &template_info.description,
                    &template_info.frameworks,
                    &template_info.repository,
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
