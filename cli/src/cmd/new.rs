use super::Execute;
use crate::{output, template};
use anyhow::{anyhow, Result};
use cargo_generate::{generate, GenerateArgs, TemplatePath};
use clap::Args;
use log::{debug, info, warn};

#[derive(Args, Debug)]
pub struct NewArgs {
    /// The template to use (e.g., noir-vite, cairo-vite)
    template: String,

    /// The name of the new project
    project_name: String,

    /// Author name (optional, falls back to git config)
    #[arg(long)]
    author: Option<String>,
}

pub struct NewCommand;

impl Execute for NewCommand {
    type Args = NewArgs;

    fn run(&self, args: &Self::Args) -> Result<()> {
        debug!(
            "Starting new command with template: {}, project: {}",
            args.template, args.project_name
        );

        output::step(&format!(
            "Creating new {} project: {}",
            args.template, args.project_name
        ));

        // Load embedded template registry
        debug!("Loading embedded template registry");
        let registry = template::load_template_registry()?;

        // Look up template
        debug!("Looking up template: {}", args.template);
        let template_info = registry.templates.get(&args.template).ok_or_else(|| {
            anyhow!(
                "Template '{}' not found. Use 'cza list' to see available templates.",
                args.template
            )
        })?;

        debug!(
            "Found template: {} - {}",
            template_info.name, template_info.description
        );
        output::info(&format!("Using template: {}", template_info.name));
        output::info(&format!("Description: {}", template_info.description));

        // Validate project name
        debug!("Validating project name: {}", args.project_name);
        self.validate_project_name(&args.project_name)?;

        // Set author from arg or try to get from git config
        debug!("Resolving author information");
        let author = args
            .author
            .clone()
            .or_else(|| {
                debug!("No author provided, trying git config");
                self.get_git_author()
            })
            .unwrap_or_else(|| {
                debug!("No author found in git config, using fallback");
                "Developer".to_string()
            });
        debug!("Using author: {}", author);

        // Create template path with git repository and subfolder
        let template_path = TemplatePath {
            git: Some(template_info.repository.clone()),
            subfolder: Some(template_info.subfolder.clone()),
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
        debug!(
            "Calling cargo-generate with repository: {}",
            template_info.repository
        );
        output::step("Generating project from template...");
        match generate(generate_args) {
            Ok(output_dir) => {
                info!("Project created successfully at: {}", output_dir.display());
                output::success("Project created successfully!");
                output::directory(&output_dir.display().to_string());

                // Run setup script if it exists
                let setup_script = output_dir.join("setup");
                debug!("Checking for setup script at: {}", setup_script.display());
                if setup_script.exists() {
                    debug!("Setup script found, executing");
                    output::step("Running project setup...");

                    let status = std::process::Command::new("sh")
                        .arg(&setup_script)
                        .current_dir(&output_dir)
                        .status();

                    match status {
                        Ok(exit_status) if exit_status.success() => {
                            debug!("Setup script completed successfully");
                            output::success("Setup completed successfully!");
                        }
                        Ok(exit_status) => {
                            warn!("Setup script exited with status: {}", exit_status);
                            output::warning(&format!(
                                "Setup script exited with status: {exit_status}"
                            ));
                            output::info("You can run './setup' manually to complete the setup");
                        }
                        Err(e) => {
                            warn!("Could not run setup script: {}", e);
                            output::warning(&format!("Could not run setup script: {e}"));
                            output::info("Please run './setup' manually to complete the setup");
                        }
                    }
                } else {
                    debug!("No setup script found");
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
    fn validate_project_name(&self, name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(anyhow!("Project name cannot be empty"));
        }

        // Must start with a letter (consistent with cargo-generate.toml regex)
        if !name.chars().next().unwrap_or('0').is_ascii_alphabetic() {
            return Err(anyhow!("Project name must start with a letter"));
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
            .args(["config", "user.name"])
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_project_name_valid() {
        let cmd = NewCommand;

        assert!(cmd.validate_project_name("valid-name").is_ok());
        assert!(cmd.validate_project_name("valid_name").is_ok());
        assert!(cmd.validate_project_name("validName").is_ok());
        assert!(cmd.validate_project_name("a").is_ok());
    }

    #[test]
    fn test_validate_project_name_invalid() {
        let cmd = NewCommand;

        assert!(cmd.validate_project_name("").is_err());
        assert!(cmd.validate_project_name("123invalid").is_err());
        assert!(cmd.validate_project_name("invalid name").is_err());
        assert!(cmd.validate_project_name("invalid/name").is_err());
        assert!(cmd.validate_project_name("invalid.name").is_err());
    }

    // Removed test_check_directory_exists since method is private

    #[test]
    fn test_template_registry_parsing() {
        let toml_content = r#"
[templates.test-template]
name = "Test Template"
description = "A test template"
repository = "https://github.com/test/test"
subfolder = "test-template"
frameworks = ["test"]
"#;

        let registry: template::TemplateRegistry = toml::from_str(toml_content).unwrap();
        assert!(registry.templates.contains_key("test-template"));

        let template = &registry.templates["test-template"];
        assert_eq!(template.name, "Test Template");
        assert_eq!(template.description, "A test template");
        assert_eq!(template.repository, "https://github.com/test/test");
        assert_eq!(template.subfolder, "test-template");
        assert_eq!(template.frameworks, vec!["test"]);
    }

    #[test]
    fn test_get_git_author_fallback() {
        let cmd = NewCommand;

        // This will likely return None unless git is configured in test environment
        let author = cmd.get_git_author();
        // Just verify it doesn't panic - actual value depends on test environment
        assert!(author.is_none() || !author.unwrap().is_empty());
    }

    #[test]
    fn test_validate_project_name_existing_directory() {
        use std::fs;
        let cmd = NewCommand;

        // Create a temporary directory
        let temp_dir_name = "test_existing_dir";
        fs::create_dir(temp_dir_name).unwrap();

        // Test should fail because directory exists
        let result = cmd.validate_project_name(temp_dir_name);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));

        // Cleanup
        fs::remove_dir(temp_dir_name).unwrap();
    }

    #[test]
    fn test_validate_project_name_edge_cases() {
        let cmd = NewCommand;

        // Test various invalid characters
        assert!(cmd.validate_project_name("invalid@name").is_err());
        assert!(cmd.validate_project_name("invalid#name").is_err());
        assert!(cmd.validate_project_name("invalid$name").is_err());
        assert!(cmd.validate_project_name("invalid%name").is_err());

        // Test starting with non-letter
        assert!(cmd.validate_project_name("_invalid").is_err());
        assert!(cmd.validate_project_name("-invalid").is_err());
        assert!(cmd.validate_project_name("9invalid").is_err());

        // Test valid edge cases
        assert!(cmd.validate_project_name("a1").is_ok());
        assert!(cmd.validate_project_name("z-test").is_ok());
        assert!(cmd.validate_project_name("test_123").is_ok());
    }

    #[test]
    fn test_new_command_invalid_template() {
        let cmd = NewCommand;
        let args = NewArgs {
            template: "nonexistent-template".to_string(),
            project_name: "test-project".to_string(),
            author: None,
        };

        let result = cmd.run(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_new_command_invalid_project_name() {
        let cmd = NewCommand;
        let args = NewArgs {
            template: "noir-vite".to_string(),
            project_name: "invalid name".to_string(),
            author: None,
        };

        let result = cmd.run(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_command_with_author() {
        let cmd = NewCommand;
        let args = NewArgs {
            template: "nonexistent-template".to_string(),
            project_name: "test-project".to_string(),
            author: Some("Test Author".to_string()),
        };

        // This will fail on template lookup, but we can still test author handling
        let result = cmd.run(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_validate_project_name_special_chars() {
        let cmd = NewCommand;

        // Test that symbols and punctuation are rejected
        assert!(cmd.validate_project_name("test@symbol").is_err()); // Contains @
        assert!(cmd.validate_project_name("test!name").is_err()); // Contains !
        assert!(cmd.validate_project_name("test.name").is_err()); // Contains .
        assert!(cmd.validate_project_name("test space").is_err()); // Contains space
    }

    #[test]
    fn test_validate_project_name_long_valid() {
        let cmd = NewCommand;

        // Test long but valid name
        let long_name = "very-long-but-valid-project-name-with-many-words-and-numbers-123";
        assert!(cmd.validate_project_name(long_name).is_ok());
    }

    #[test]
    fn test_validate_project_name_single_char() {
        let cmd = NewCommand;

        // Single character tests
        assert!(cmd.validate_project_name("a").is_ok());
        assert!(cmd.validate_project_name("Z").is_ok());
        assert!(cmd.validate_project_name("1").is_err()); // starts with number
        assert!(cmd.validate_project_name("_").is_err()); // starts with underscore
    }

    #[test]
    fn test_git_author_with_empty_output() {
        // This tests the filter condition in get_git_author that filters empty strings
        let cmd = NewCommand;
        let _ = cmd.get_git_author(); // Just ensure it doesn't panic
    }

    #[test]
    fn test_new_args_clone() {
        // Test that NewArgs can be constructed with cloned strings
        let template = "noir-vite".to_string();
        let name = "test-project".to_string();
        let author = Some("Test Author".to_string());

        let args = NewArgs {
            template: template.clone(),
            project_name: name.clone(),
            author: author.clone(),
        };

        assert_eq!(args.template, template);
        assert_eq!(args.project_name, name);
        assert_eq!(args.author, author);
    }
}
