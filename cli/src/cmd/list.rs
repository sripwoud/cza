use super::Execute;
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

    fn execute(&self, args: &Self::Args) {
        if let Err(e) = self.list_templates(args) {
            eprintln!("Error listing templates: {}", e);
            std::process::exit(1);
        }
    }
}

impl ListCommand {
    fn list_templates(&self, args: &ListArgs) -> Result<()> {
        // Load embedded template registry
        let registry = self.load_template_registry()?;

        if registry.templates.is_empty() {
            println!("No templates available.");
            return Ok(());
        }

        println!("Available templates:\n");

        // Sort templates by name for consistent output
        let mut templates: Vec<_> = registry.templates.iter().collect();
        templates.sort_by_key(|(key, _)| *key);

        for (template_key, template_info) in templates {
            if args.detailed {
                self.print_detailed_template(template_key, template_info);
            } else {
                self.print_simple_template(template_key, template_info);
            }
        }

        if !args.detailed {
            println!("\nUse 'cza list --detailed' for more information about templates.");
        }

        println!("\nTo create a new project:");
        println!("  cza new <template> <project-name>");
        println!("  Example: cza new noir-vite my-zk-app");

        Ok(())
    }

    fn print_simple_template(&self, key: &str, info: &TemplateInfo) {
        println!("  {} - {}", key, info.name);
    }

    fn print_detailed_template(&self, key: &str, info: &TemplateInfo) {
        println!("📦 {}", key);
        println!("   Name: {}", info.name);
        println!("   Description: {}", info.description);
        println!("   Frameworks: {}", info.frameworks.join(", "));
        println!("   Repository: {}", info.repository);
        println!();
    }

    fn load_template_registry(&self) -> Result<TemplateRegistry> {
        // Load embedded templates.toml
        let templates_toml = include_str!("../../templates.toml");
        toml::from_str(templates_toml)
            .map_err(|e| anyhow!("Failed to parse template registry: {}", e))
    }
}
