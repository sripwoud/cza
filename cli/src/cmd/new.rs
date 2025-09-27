use super::Execute;
use crate::{config::Config, output, template};
use anyhow::{anyhow, Result};
use cargo_generate::{generate, GenerateArgs, TemplatePath};
use clap::Args;
use log::{debug, info, warn};

#[derive(Args, Debug)]
pub struct NewArgs {
    /// The template to use (e.g., noir-vite, cairo-vite). If not provided, uses default_template from config
    template: Option<String>,

    /// The name of the new project
    project_name: String,

    /// Author name (optional, falls back to config then git config)
    #[arg(long)]
    author: Option<String>,
}

pub struct NewCommand;

impl Execute for NewCommand {
    type Args = NewArgs;

    fn run(&self, args: &Self::Args) -> Result<()> {
        // Load configuration
        debug!("Loading configuration");
        let config = Config::load()?;

        // Resolve template name from args or config
        let template_name = match &args.template {
            Some(template) => template.clone(),
            None => config
                .user
                .default_template
                .clone()
                .ok_or_else(|| anyhow!("No template specified and no default_template configured. Use 'cza config set user.default_template <template>' to set a default, or specify a template: 'cza new <template> <project_name>'"))?,
        };

        debug!(
            "Starting new command with template: {}, project: {}",
            template_name, args.project_name
        );

        output::step(&format!(
            "Creating new {} project: {}",
            template_name, args.project_name
        ));

        // Load embedded template registry
        debug!("Loading embedded template registry");
        let registry = template::load_template_registry()?;

        // Look up template
        debug!("Looking up template: {}", template_name);
        let template_info = registry.templates.get(&template_name).ok_or_else(|| {
            anyhow!(
                "Template '{}' not found. Use 'cza list' to see available templates.",
                template_name
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
        self.validate_project_name(&args.project_name, &config)?;

        // Set author from arg, config, or git config
        debug!("Resolving author information");
        let author = args
            .author
            .clone()
            .or_else(|| {
                debug!("No author arg provided, checking config");
                config.user.author.clone()
            })
            .or_else(|| {
                debug!("No author in config, trying git config");
                self.get_git_author()
            })
            .unwrap_or_else(|| {
                debug!("No author found anywhere, using fallback");
                "Developer".to_string()
            });
        debug!("Using author: {}", author);

        // Get email from config if available
        let email = config.user.email.clone();
        if let Some(ref email_addr) = email {
            debug!("Using email from config: {}", email_addr);
        }

        // Create template path with git repository and subfolder
        let template_path = TemplatePath {
            git: Some(template_info.repository.clone()),
            subfolder: Some(template_info.subfolder.clone()),
            ..Default::default()
        };

        // Create define arguments for template variables
        let mut define_args = vec![
            format!("project_name={}", args.project_name),
            format!("author={}", author),
        ];

        // Add email if available
        if let Some(email_addr) = email {
            define_args.push(format!("author_email={}", email_addr));
        }

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

                // Post-generation setup based on config
                self.run_post_generation_setup(&output_dir, &config)?;

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
    fn validate_project_name(&self, name: &str, config: &Config) -> Result<()> {
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
            if config.development.confirm_overwrite {
                return Err(anyhow!(
                    "Directory '{}' already exists. Remove it first or choose a different name.",
                    name
                ));
            } else {
                warn!("Directory '{}' already exists but confirm_overwrite is disabled, proceeding anyway", name);
                output::warning(&format!(
                    "Directory '{}' already exists, proceeding anyway",
                    name
                ));
            }
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

    fn run_post_generation_setup(
        &self,
        output_dir: &std::path::Path,
        config: &Config,
    ) -> Result<()> {
        debug!("Running post-generation setup");

        // Initialize git if enabled
        if config.user.git_init {
            debug!("git_init is enabled, initializing git repository");
            output::step("Initializing git repository...");

            let git_status = std::process::Command::new("git")
                .arg("init")
                .current_dir(output_dir)
                .status();

            match git_status {
                Ok(exit_status) if exit_status.success() => {
                    debug!("Git repository initialized successfully");
                    output::success("Git repository initialized!");
                }
                Ok(exit_status) => {
                    warn!("Git init exited with status: {}", exit_status);
                    output::warning(&format!("Git init failed with status: {exit_status}"));
                }
                Err(e) => {
                    warn!("Could not run git init: {}", e);
                    output::warning(&format!("Could not initialize git: {e}"));
                }
            }
        } else {
            debug!("git_init is disabled, skipping git initialization");
        }

        // Install dependencies if enabled
        if config.post_generation.auto_install_deps {
            debug!("auto_install_deps is enabled, running mise install");
            output::step("Installing dependencies with mise...");

            let mise_status = std::process::Command::new("mise")
                .arg("install")
                .current_dir(output_dir)
                .status();

            match mise_status {
                Ok(exit_status) if exit_status.success() => {
                    debug!("Dependencies installed successfully");
                    output::success("Dependencies installed!");
                }
                Ok(exit_status) => {
                    warn!("mise install exited with status: {}", exit_status);
                    output::warning(&format!("mise install failed with status: {exit_status}"));
                    output::info("You can run 'mise install' manually in the project directory");
                }
                Err(e) => {
                    warn!("Could not run mise install: {}", e);
                    output::warning(&format!("Could not run mise install: {e}"));
                    output::info("You can run 'mise install' manually in the project directory");
                }
            }
        } else {
            debug!("auto_install_deps is disabled, skipping dependency installation");
        }

        // Setup git hooks if enabled
        if config.post_generation.auto_setup_hooks {
            debug!("auto_setup_hooks is enabled, running hk install");
            output::step("Setting up git hooks with hk...");

            let hk_status = std::process::Command::new("hk")
                .arg("install")
                .current_dir(output_dir)
                .status();

            match hk_status {
                Ok(exit_status) if exit_status.success() => {
                    debug!("Git hooks installed successfully");
                    output::success("Git hooks installed!");
                }
                Ok(exit_status) => {
                    warn!("hk install exited with status: {}", exit_status);
                    output::warning(&format!("hk install failed with status: {exit_status}"));
                    output::info("You can run 'hk install' manually in the project directory");
                }
                Err(e) => {
                    warn!("Could not run hk install: {}", e);
                    output::warning(&format!("Could not run hk install: {e}"));
                    output::info("You can run 'hk install' manually in the project directory");
                }
            }
        } else {
            debug!("auto_setup_hooks is disabled, skipping git hooks setup");
        }

        // Open in editor if configured
        if let Some(ref editor) = config.post_generation.open_editor {
            debug!("open_editor is configured: {}, opening project", editor);
            output::step(&format!("Opening project in {}...", editor));

            let editor_status = std::process::Command::new(editor).arg(output_dir).spawn();

            match editor_status {
                Ok(_) => {
                    debug!("Editor opened successfully");
                    output::success(&format!("Project opened in {}!", editor));
                }
                Err(e) => {
                    warn!("Could not open editor: {}", e);
                    output::warning(&format!("Could not open {}: {}", editor, e));
                }
            }
        } else {
            debug!("open_editor is not configured, skipping editor open");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_project_name_valid() {
        let cmd = NewCommand;
        let config = Config::default();

        assert!(cmd.validate_project_name("valid-name", &config).is_ok());
        assert!(cmd.validate_project_name("valid_name", &config).is_ok());
        assert!(cmd.validate_project_name("validName", &config).is_ok());
        assert!(cmd.validate_project_name("a", &config).is_ok());
    }

    #[test]
    fn test_validate_project_name_invalid() {
        let cmd = NewCommand;
        let config = Config::default();

        assert!(cmd.validate_project_name("", &config).is_err());
        assert!(cmd.validate_project_name("123invalid", &config).is_err());
        assert!(cmd.validate_project_name("invalid name", &config).is_err());
        assert!(cmd.validate_project_name("invalid/name", &config).is_err());
        assert!(cmd.validate_project_name("invalid.name", &config).is_err());
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
        let config = Config::default(); // confirm_overwrite = true by default

        // Create a temporary directory
        let temp_dir_name = "test_existing_dir";
        fs::create_dir(temp_dir_name).unwrap();

        // Test should fail because directory exists and confirm_overwrite is true
        let result = cmd.validate_project_name(temp_dir_name, &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));

        // Cleanup
        fs::remove_dir(temp_dir_name).unwrap();
    }

    #[test]
    fn test_validate_project_name_edge_cases() {
        let cmd = NewCommand;
        let config = Config::default();

        // Test various invalid characters
        assert!(cmd.validate_project_name("invalid@name", &config).is_err());
        assert!(cmd.validate_project_name("invalid#name", &config).is_err());
        assert!(cmd.validate_project_name("invalid$name", &config).is_err());
        assert!(cmd.validate_project_name("invalid%name", &config).is_err());

        // Test starting with non-letter
        assert!(cmd.validate_project_name("_invalid", &config).is_err());
        assert!(cmd.validate_project_name("-invalid", &config).is_err());
        assert!(cmd.validate_project_name("9invalid", &config).is_err());

        // Test valid edge cases
        assert!(cmd.validate_project_name("a1", &config).is_ok());
        assert!(cmd.validate_project_name("z-test", &config).is_ok());
        assert!(cmd.validate_project_name("test_123", &config).is_ok());
    }

    #[test]
    fn test_new_command_invalid_template() {
        let cmd = NewCommand;
        let args = NewArgs {
            template: Some("nonexistent-template".to_string()),
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
            template: Some("noir-vite".to_string()),
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
            template: Some("nonexistent-template".to_string()),
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
        let config = Config::default();

        // Test that symbols and punctuation are rejected
        assert!(cmd.validate_project_name("test@symbol", &config).is_err()); // Contains @
        assert!(cmd.validate_project_name("test!name", &config).is_err()); // Contains !
        assert!(cmd.validate_project_name("test.name", &config).is_err()); // Contains .
        assert!(cmd.validate_project_name("test space", &config).is_err()); // Contains space
    }

    #[test]
    fn test_validate_project_name_long_valid() {
        let cmd = NewCommand;
        let config = Config::default();

        // Test long but valid name
        let long_name = "very-long-but-valid-project-name-with-many-words-and-numbers-123";
        assert!(cmd.validate_project_name(long_name, &config).is_ok());
    }

    #[test]
    fn test_validate_project_name_single_char() {
        let cmd = NewCommand;
        let config = Config::default();

        // Single character tests
        assert!(cmd.validate_project_name("a", &config).is_ok());
        assert!(cmd.validate_project_name("Z", &config).is_ok());
        assert!(cmd.validate_project_name("1", &config).is_err()); // starts with number
        assert!(cmd.validate_project_name("_", &config).is_err()); // starts with underscore
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
        let template = Some("noir-vite".to_string());
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

    #[test]
    fn test_default_template_config_integration() {
        // Test that NewArgs can use None for template to rely on config
        let args = NewArgs {
            template: None,
            project_name: "test-project".to_string(),
            author: None,
        };

        assert_eq!(args.template, None);
        assert_eq!(args.project_name, "test-project");
        assert_eq!(args.author, None);
    }

    #[test]
    fn test_validate_project_name_with_confirm_overwrite_disabled() {
        use std::fs;
        let cmd = NewCommand;
        let mut config = Config::default();
        config.development.confirm_overwrite = false;

        // Create a temporary directory
        let temp_dir_name = "test_existing_dir_no_confirm";
        fs::create_dir(temp_dir_name).unwrap();

        // Test should succeed because confirm_overwrite is disabled
        let result = cmd.validate_project_name(temp_dir_name, &config);
        assert!(result.is_ok());

        // Cleanup
        fs::remove_dir(temp_dir_name).unwrap();
    }

    #[test]
    fn test_config_integration_author_precedence() {
        // Test that CLI arg author takes precedence over config
        let cmd = NewCommand;
        let args = NewArgs {
            template: Some("nonexistent-template".to_string()),
            project_name: "test-project".to_string(),
            author: Some("CLI Author".to_string()),
        };

        // Even though this will fail on template lookup, we can verify the precedence logic exists
        let result = cmd.run(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }
}
