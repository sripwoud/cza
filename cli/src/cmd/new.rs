use super::Execute;
use crate::output;
use anyhow::{anyhow, Result};
use cargo_generate::{generate, GenerateArgs, TemplatePath};
use clap::Args;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Args)]
pub struct NewArgs {
    /// The template to use (e.g., noir-vite, cairo-vite)
    template: String,

    /// The name of the new project
    project_name: String,

    /// Author name (optional, falls back to git config)
    #[arg(long)]
    author: Option<String>,
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
    #[allow(dead_code)]
    frameworks: Vec<String>,
}

pub struct NewCommand;

impl Execute for NewCommand {
    type Args = NewArgs;

    fn run(&self, args: &Self::Args) -> Result<()> {
        output::step(&format!(
            "Creating new {} project: {}",
            args.template, args.project_name
        ));

        // Load embedded template registry
        let registry = self.load_template_registry()?;

        // Look up template
        let template_info = registry.templates.get(&args.template).ok_or_else(|| {
            anyhow!(
                "Template '{}' not found. Use 'cza list' to see available templates.",
                args.template
            )
        })?;

        output::info(&format!("Using template: {}", template_info.name));
        output::info(&format!("Description: {}", template_info.description));

        // Validate project name
        self.validate_project_name(&args.project_name)?;

        // Set author from arg or try to get from git config
        let author = args
            .author
            .clone()
            .or_else(|| self.get_git_author())
            .unwrap_or_else(|| "Developer".to_string());

        // Create template path
        let template_path = TemplatePath {
            git: Some(template_info.repository.clone()),
            ..Default::default()
        };

        // Create define arguments for template variables
        let define_args = vec![
            format!("project_name={}", args.project_name),
            format!("author={}", author),
        ];

        // Create cargo-generate args
        let generate_args = GenerateArgs {
            template_path,
            name: Some(args.project_name.clone()),
            define: define_args,
            ..Default::default()
        };

        // Generate project using cargo-generate
        output::step("Generating project from template...");
        match generate(generate_args) {
            Ok(output_dir) => {
                output::success("Project created successfully!");
                output::directory(&output_dir.display().to_string());

                // Run setup script if it exists
                let setup_script = output_dir.join("setup");
                if setup_script.exists() {
                    output::step("Running project setup...");

                    let status = std::process::Command::new("sh")
                        .arg(setup_script)
                        .current_dir(&output_dir)
                        .status();

                    match status {
                        Ok(exit_status) if exit_status.success() => {
                            output::success("Setup completed successfully!");
                        }
                        Ok(exit_status) => {
                            output::warning(&format!(
                                "Setup script exited with status: {}",
                                exit_status
                            ));
                            output::info("You can run './setup' manually to complete the setup");
                        }
                        Err(e) => {
                            output::warning(&format!("Could not run setup script: {}", e));
                            output::info("Please run './setup' manually to complete the setup");
                        }
                    }
                }

                output::next_steps(&[&format!("cd {}", args.project_name), "mise run dev"]);
            }
            Err(e) => {
                return Err(anyhow!("Failed to generate project: {}", e));
            }
        }

        Ok(())
    }
}

impl NewCommand {
    fn load_template_registry(&self) -> Result<TemplateRegistry> {
        // Load embedded templates.toml
        let templates_toml = include_str!("../../templates.toml");
        toml::from_str(templates_toml)
            .map_err(|e| anyhow!("Failed to parse template registry: {}", e))
    }

    fn validate_project_name(&self, name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(anyhow!("Project name cannot be empty"));
        }

        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(anyhow!(
                "Project name can only contain alphanumeric characters, hyphens, and underscores"
            ));
        }

        if std::path::Path::new(name).exists() {
            return Err(anyhow!("Directory '{}' already exists", name));
        }

        Ok(())
    }

    fn get_git_author(&self) -> Option<String> {
        std::process::Command::new("git")
            .args(&["config", "user.name"])
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    String::from_utf8(output.stdout)
                        .ok()
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                } else {
                    None
                }
            })
    }
}
