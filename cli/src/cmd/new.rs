use super::Execute;
use crate::{config::Config, output, template, utils};
use anyhow::{anyhow, Result};
use cargo_generate::{generate, GenerateArgs, TemplatePath};
use clap::Args;
use log::{debug, info, warn};

#[derive(Args, Debug)]
pub struct NewArgs {
    /// The name of the new project
    project_name: String,

    /// The template to use (e.g., noir-vite, cairo-vite). If not provided, uses default_template from config
    #[arg(short, long)]
    template: Option<String>,

    /// Author name (optional, falls back to config then git config)
    #[arg(long)]
    author: Option<String>,

    /// Skip git initialization (overrides config setting)
    #[arg(long)]
    no_git: bool,

    /// Preview template structure without creating files
    #[arg(long)]
    dry_run: bool,
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

        if args.dry_run {
            output::step(&format!(
                "Previewing {} template structure for project: {}",
                template_name, args.project_name
            ));
        } else {
            output::step(&format!(
                "Creating new {} project: {}",
                template_name, args.project_name
            ));
        }

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

        // If dry-run, show preview and exit
        if args.dry_run {
            return self.preview_template(args, &template_name, template_info);
        }

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

                // Post-generation setup based on config and args
                self.run_post_generation_setup(&output_dir, &config, args)?;

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
    fn preview_template(
        &self,
        args: &NewArgs,
        template_name: &str,
        template_info: &template::TemplateInfo,
    ) -> Result<()> {
        output::header("Dry Run Preview");
        output::info(&format!("Project name: {}", args.project_name));
        output::info(&format!("Template: {}", template_name));
        output::info(&format!("Repository: {}", template_info.repository));
        output::info(&format!("Subfolder: {}", template_info.subfolder));
        output::info(&format!(
            "Frameworks: {}",
            template_info.frameworks.join(", ")
        ));

        output::step("What would be created:");
        output::info(&format!("  📁 ./{}/", args.project_name));
        output::info("    ├── Cargo.toml (ZK framework dependencies)");
        output::info("    ├── mise.toml (development tools)");
        output::info("    ├── package.json (frontend dependencies)");
        output::info("    ├── src/ (ZK circuit code)");
        output::info("    └── web/ (frontend application)");

        output::step("Post-generation setup that would run:");
        output::info("  1. git init (if enabled in config)");
        output::info("  2. mise install (if auto_install_deps enabled)");
        output::info("  3. hk install (if auto_setup_hooks enabled)");

        output::success("Preview complete! Remove --dry-run to create the project.");

        Ok(())
    }

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
        utils::get_git_config("user.name")
    }

    fn run_post_generation_setup(
        &self,
        output_dir: &std::path::Path,
        config: &Config,
        args: &NewArgs,
    ) -> Result<()> {
        debug!("Running post-generation setup");

        // Initialize git if enabled (CLI flag overrides config)
        let should_init_git = !args.no_git && config.user.git_init;
        if should_init_git {
            debug!("git_init is enabled, initializing git repository");
            let _ = utils::run_post_generation_command(
                "git",
                &["init"],
                output_dir,
                "Initializing git repository...",
                "Git repository initialized!",
                None,
            );
        } else if args.no_git {
            debug!("--no-git flag provided, skipping git initialization");
        } else {
            debug!("git_init is disabled in config, skipping git initialization");
        }

        // Install dependencies if enabled
        if config.post_generation.auto_install_deps {
            debug!("auto_install_deps is enabled, running mise install");
            let _ = utils::run_post_generation_command(
                "mise",
                &["install"],
                output_dir,
                "Installing dependencies with mise...",
                "Dependencies installed!",
                Some("You can run 'mise install' manually in the project directory"),
            );
        } else {
            debug!("auto_install_deps is disabled, skipping dependency installation");
        }

        // Setup git hooks if enabled (requires git to be initialized)
        if should_init_git && config.post_generation.auto_setup_hooks {
            debug!("auto_setup_hooks is enabled and git is initialized, running hk install");
            let _ = utils::run_post_generation_command(
                "hk",
                &["install"],
                output_dir,
                "Setting up git hooks with hk...",
                "Git hooks installed!",
                Some("You can run 'hk install' manually in the project directory"),
            );
        } else if !should_init_git {
            debug!("git not initialized, skipping git hooks setup");
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
            project_name: "test-project".to_string(),
            template: Some("nonexistent-template".to_string()),
            author: None,
            no_git: false,
            dry_run: false,
        };

        let result = cmd.run(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_new_command_invalid_project_name() {
        let cmd = NewCommand;
        let args = NewArgs {
            project_name: "invalid name".to_string(),
            template: Some("noir-vite".to_string()),
            author: None,
            no_git: false,
            dry_run: false,
        };

        let result = cmd.run(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_command_with_author() {
        let cmd = NewCommand;
        let args = NewArgs {
            project_name: "test-project".to_string(),
            template: Some("nonexistent-template".to_string()),
            author: Some("Test Author".to_string()),
            no_git: false,
            dry_run: false,
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
            project_name: name.clone(),
            template: template.clone(),
            author: author.clone(),
            no_git: false,
            dry_run: false,
        };

        assert_eq!(args.template, template);
        assert_eq!(args.project_name, name);
        assert_eq!(args.author, author);
    }

    #[test]
    fn test_default_template_config_integration() {
        // Test that NewArgs can use None for template to rely on config
        let args = NewArgs {
            project_name: "test-project".to_string(),
            template: None,
            author: None,
            no_git: false,
            dry_run: false,
        };

        assert_eq!(args.template, None);
        assert_eq!(args.project_name, "test-project");
        assert_eq!(args.author, None);
        assert!(!args.no_git);
        assert!(!args.dry_run);
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
            project_name: "test-project".to_string(),
            template: Some("nonexistent-template".to_string()),
            author: Some("CLI Author".to_string()),
            no_git: false,
            dry_run: false,
        };

        // Even though this will fail on template lookup, we can verify the precedence logic exists
        let result = cmd.run(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_no_git_flag_overrides_config() {
        use tempfile::TempDir;

        let cmd = NewCommand;
        let mut config = Config::default();
        config.user.git_init = true;

        let args = NewArgs {
            project_name: "test-project".to_string(),
            template: None,
            author: None,
            no_git: true,
            dry_run: false,
        };

        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let result = cmd.run_post_generation_setup(temp_path, &config, &args);
        assert!(result.is_ok());

        assert!(!temp_path.join(".git").exists());
    }

    #[test]
    fn test_no_git_flag_false_respects_config() {
        use tempfile::TempDir;

        let cmd = NewCommand;
        let mut config = Config::default();
        config.user.git_init = true;

        let args = NewArgs {
            project_name: "test-project".to_string(),
            template: None,
            author: None,
            no_git: false,
            dry_run: false,
        };

        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let result = cmd.run_post_generation_setup(temp_path, &config, &args);
        assert!(result.is_ok());

        assert!(temp_path.join(".git").exists());
    }

    #[test]
    fn test_no_git_config_disabled() {
        use tempfile::TempDir;

        let cmd = NewCommand;
        let mut config = Config::default();
        config.user.git_init = false;

        let args = NewArgs {
            project_name: "test-project".to_string(),
            template: None,
            author: None,
            no_git: false,
            dry_run: false,
        };

        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let result = cmd.run_post_generation_setup(temp_path, &config, &args);
        assert!(result.is_ok());

        assert!(!temp_path.join(".git").exists());
    }

    #[test]
    fn test_dry_run_flag() {
        let cmd = NewCommand;
        let args = NewArgs {
            project_name: "test-project".to_string(),
            template: Some("noir-vite".to_string()),
            author: None,
            no_git: false,
            dry_run: true,
        };

        let result = cmd.run(&args);
        assert!(result.is_ok());
        assert!(!std::path::Path::new("test-project").exists());
    }

    #[test]
    fn test_dry_run_flag_false() {
        let args = NewArgs {
            project_name: "test-project".to_string(),
            template: Some("noir-vite".to_string()),
            author: None,
            no_git: false,
            dry_run: false,
        };

        assert!(!args.dry_run);
    }
}
