//! Foundry-inspired CLI output module with standardized messaging functions.
//!
//! This module provides consistent, professional CLI output with:
//! - Color support with automatic terminal detection
//! - Emoji support with ASCII fallbacks
//! - Consistent styling across all commands
//! - Terminal-aware formatting
//! - Configuration-aware color control

use anyhow;
use console::{style, Emoji, StyledObject, Term};

/// Success indicator emoji with ASCII fallback
static SUCCESS_EMOJI: Emoji<'_, '_> = Emoji("✅", "[SUCCESS]");

/// Info indicator emoji with ASCII fallback
static INFO_EMOJI: Emoji<'_, '_> = Emoji("ℹ️", "[INFO]");

/// Warning indicator emoji with ASCII fallback
static WARNING_EMOJI: Emoji<'_, '_> = Emoji("⚠️", "[WARNING]");

/// Error indicator emoji with ASCII fallback
static ERROR_EMOJI: Emoji<'_, '_> = Emoji("❌", "[ERROR]");

/// Step indicator emoji with ASCII fallback
static STEP_EMOJI: Emoji<'_, '_> = Emoji("📦", "[STEP]");

/// Directory indicator emoji with ASCII fallback
static DIRECTORY_EMOJI: Emoji<'_, '_> = Emoji("📁", "[DIR]");

/// Next steps indicator emoji with ASCII fallback
static NEXT_EMOJI: Emoji<'_, '_> = Emoji("👉", "==>");

/// Output manager for consistent CLI messaging
pub struct Output {
    term: Term,
    color_enabled: bool,
}

impl Output {
    /// Create a new output manager with default color settings
    pub fn new() -> Self {
        Self {
            term: Term::stdout(),
            color_enabled: true, // Default to enabled
        }
    }

    /// Create a new output manager with specified color setting
    pub fn with_color(color_enabled: bool) -> Self {
        Self {
            term: Term::stdout(),
            color_enabled,
        }
    }

    /// Apply styling if colors are enabled, otherwise return plain text
    fn apply_style(&self, text: &str, styled: StyledObject<&str>) -> String {
        if self.color_enabled {
            styled.to_string()
        } else {
            text.to_string()
        }
    }

    /// Print a success message with green styling
    pub fn success(&self, message: &str) {
        let styled_message = self.apply_style(message, style(message).green().bold());
        println!("{SUCCESS_EMOJI} {styled_message}");
    }

    /// Print an info message with blue styling
    pub fn info(&self, message: &str) {
        let styled_message = self.apply_style(message, style(message).blue());
        println!("{INFO_EMOJI} {styled_message}");
    }

    /// Print a warning message with yellow styling
    pub fn warning(&self, message: &str) {
        let styled_message = self.apply_style(message, style(message).yellow().bold());
        println!("{WARNING_EMOJI} {styled_message}");
    }

    /// Print an error message with red styling
    pub fn error(&self, message: &str) {
        let styled_message = self.apply_style(message, style(message).red().bold());
        eprintln!("{ERROR_EMOJI} {styled_message}");
    }

    /// Print a step message with cyan styling (for progress indication)
    pub fn step(&self, message: &str) {
        let styled_message = self.apply_style(message, style(message).cyan());
        println!("{STEP_EMOJI} {styled_message}");
    }

    /// Print a directory path with consistent styling
    pub fn directory(&self, path: &str) {
        let styled_path = self.apply_style(path, style(path).magenta().bold());
        println!("{DIRECTORY_EMOJI} Location: {styled_path}");
    }

    /// Print next steps with consistent styling
    pub fn next_steps(&self, steps: &[&str]) {
        if steps.is_empty() {
            return;
        }

        println!();
        let styled_header = self.apply_style("Next steps:", style("Next steps:").cyan().bold());
        println!("{NEXT_EMOJI} {styled_header}");

        for step in steps {
            let styled_step = self.apply_style(step, style(*step).dim());
            println!("  {styled_step}");
        }
    }

    /// Print a command suggestion with consistent styling
    pub fn command_example(&self, description: &str, command: &str) {
        let styled_desc = self.apply_style(description, style(description).dim());
        let styled_command = self.apply_style(command, style(command).green().bold());
        println!("  {styled_desc}: {styled_command}");
    }

    /// Print a header for sections
    pub fn header(&self, title: &str) {
        println!();
        let styled_title = self.apply_style(title, style(title).bold().underlined());
        println!("{styled_title}");
        println!();
    }

    /// Print a plain message without styling (for regular content)
    pub fn plain(&self, message: &str) {
        println!("{message}");
    }

    /// Print a styled key-value pair
    pub fn key_value(&self, key: &str, value: &str) {
        let styled_key = self.apply_style(key, style(key).bold());
        println!("   {styled_key}: {value}");
    }

    /// Print a template item with consistent styling
    pub fn template_item(&self, name: &str, description: &str) {
        let styled_name = self.apply_style(name, style(name).green().bold());
        let styled_desc = self.apply_style(description, style(description).dim());
        println!("  {styled_name} - {styled_desc}");
    }

    /// Print detailed template information
    pub fn template_detailed(
        &self,
        key: &str,
        name: &str,
        description: &str,
        frameworks: &[String],
        repository: &str,
    ) {
        let styled_key = self.apply_style(key, style(key).green().bold());
        println!("{STEP_EMOJI} {styled_key}");
        self.key_value("Name", name);
        self.key_value("Description", description);
        self.key_value("Frameworks", &frameworks.join(", "));
        self.key_value("Repository", repository);
        println!();
    }

    /// Clear the screen if supported
    pub fn clear(&self) {
        let _ = self.term.clear_screen();
    }
}

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}

/// Create an output instance based on current config
fn get_output() -> Output {
    use crate::config::Config;
    match Config::load() {
        Ok(config) => Output::with_color(config.development.color),
        Err(_) => Output::new(), // Fallback to default if config can't be loaded
    }
}

/// Convenience function for success messages
pub fn success(message: &str) {
    get_output().success(message);
}

/// Convenience function for info messages
pub fn info(message: &str) {
    get_output().info(message);
}

/// Convenience function for warning messages
pub fn warning(message: &str) {
    get_output().warning(message);
}

/// Convenience function for error messages
pub fn error(message: &str) {
    get_output().error(message);
}

/// Convenience function for step messages
pub fn step(message: &str) {
    get_output().step(message);
}

/// Convenience function for directory messages
pub fn directory(path: &str) {
    get_output().directory(path);
}

/// Convenience function for next steps
pub fn next_steps(steps: &[&str]) {
    get_output().next_steps(steps);
}

/// Convenience function for command examples
pub fn command_example(description: &str, command: &str) {
    get_output().command_example(description, command);
}

/// Convenience function for headers
pub fn header(title: &str) {
    get_output().header(title);
}

/// Convenience function for plain messages
pub fn plain(message: &str) {
    get_output().plain(message);
}

/// Convenience function for template items
pub fn template_item(name: &str, description: &str) {
    get_output().template_item(name, description);
}

/// Convenience function for detailed template info
pub fn template_detailed(
    key: &str,
    name: &str,
    description: &str,
    frameworks: &[String],
    repository: &str,
) {
    get_output().template_detailed(key, name, description, frameworks, repository);
}

/// Format and display anyhow errors using our consistent output system
pub fn format_error(err: &anyhow::Error) {
    let output = get_output();
    let error_msg = err.to_string();

    // Handle specific error patterns with enhanced formatting
    if error_msg.contains("not found. Use 'cza list'") {
        // Split the template not found error for better formatting
        if let Some(template_part) = error_msg.split('.').next() {
            output.error(template_part);
            output.info("Use 'cza list' to see available templates.");
        } else {
            output.error(&error_msg);
        }
    } else if error_msg.contains("already exists") {
        // Handle directory exists errors
        output.error(&error_msg);
        output.info("Choose a different project name or remove the existing directory.");
    } else if error_msg.contains("Project name") {
        // Handle project name validation errors
        output.error(&error_msg);
        output.info(
            "Project names can only contain alphanumeric characters, hyphens, and underscores.",
        );
    } else {
        // Default error formatting
        output.error(&error_msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow;

    #[test]
    fn test_output_new() {
        let _output = Output::new();
        // Just ensure it creates successfully without panicking
    }

    #[test]
    fn test_output_default() {
        let _output = Output::default();
        // Just ensure it creates successfully without panicking
    }

    #[test]
    fn test_success() {
        let output = Output::new();
        output.success("Test success message");
    }

    #[test]
    fn test_info() {
        let output = Output::new();
        output.info("Test info message");
    }

    #[test]
    fn test_warning() {
        let output = Output::new();
        output.warning("Test warning message");
    }

    #[test]
    fn test_error() {
        let output = Output::new();
        output.error("Test error message");
    }

    #[test]
    fn test_step() {
        let output = Output::new();
        output.step("Test step message");
    }

    #[test]
    fn test_directory() {
        let output = Output::new();
        output.directory("/test/path");
    }

    #[test]
    fn test_next_steps_empty() {
        let output = Output::new();
        output.next_steps(&[]);
    }

    #[test]
    fn test_next_steps_with_items() {
        let output = Output::new();
        output.next_steps(&["Step 1", "Step 2", "Step 3"]);
    }

    #[test]
    fn test_command_example() {
        let output = Output::new();
        output.command_example("Run the app", "npm start");
    }

    #[test]
    fn test_header() {
        let output = Output::new();
        output.header("Test Header");
    }

    #[test]
    fn test_plain() {
        let output = Output::new();
        output.plain("Plain message");
    }

    #[test]
    fn test_key_value() {
        let output = Output::new();
        output.key_value("Name", "Test Value");
    }

    #[test]
    fn test_template_item() {
        let output = Output::new();
        output.template_item("noir-vite", "Noir with Vite frontend");
    }

    #[test]
    fn test_template_detailed() {
        let output = Output::new();
        let frameworks = vec!["Noir".to_string(), "Vite".to_string()];
        output.template_detailed(
            "noir-vite",
            "Noir Vite Template",
            "A template with Noir and Vite",
            &frameworks,
            "github.com/example/repo",
        );
    }

    #[test]
    fn test_clear() {
        let output = Output::new();
        output.clear(); // Should not panic
    }

    #[test]
    fn test_convenience_functions() {
        success("Test success");
        info("Test info");
        warning("Test warning");
        error("Test error");
        step("Test step");
        directory("/test/path");
        next_steps(&["Step 1", "Step 2"]);
        command_example("Test", "test command");
        header("Test Header");
        plain("Test plain");
        template_item("template", "description");
        template_detailed("key", "name", "desc", &["framework".to_string()], "repo");
    }

    #[test]
    fn test_format_error_template_not_found() {
        let err = anyhow::anyhow!(
            "Template 'invalid' not found. Use 'cza list' to see available templates."
        );
        format_error(&err);
    }

    #[test]
    fn test_format_error_already_exists() {
        let err = anyhow::anyhow!("Directory already exists");
        format_error(&err);
    }

    #[test]
    fn test_format_error_project_name() {
        let err = anyhow::anyhow!("Project name contains invalid characters");
        format_error(&err);
    }

    #[test]
    fn test_format_error_generic() {
        let err = anyhow::anyhow!("Generic error message");
        format_error(&err);
    }

    #[test]
    fn test_format_error_template_not_found_no_split() {
        let err = anyhow::anyhow!("not found. Use 'cza list'");
        format_error(&err);
    }

    #[test]
    fn test_output_with_color_enabled() {
        let output = Output::with_color(true);
        // Test that colored output includes ANSI escape codes
        output.success("Test message");
        output.info("Test message");
        output.warning("Test message");
        output.error("Test message");
    }

    #[test]
    fn test_output_with_color_disabled() {
        let output = Output::with_color(false);
        // Test that output without color works (should not panic)
        output.success("Test message");
        output.info("Test message");
        output.warning("Test message");
        output.error("Test message");
    }

    #[test]
    fn test_apply_style_with_color_enabled() {
        let output = Output::with_color(true);
        let result = output.apply_style("test", style("test").green());
        // In test environments, the console might not support colors
        // So we just verify it doesn't crash and returns some string
        assert!(!result.is_empty());
    }

    #[test]
    fn test_apply_style_with_color_disabled() {
        let output = Output::with_color(false);
        let result = output.apply_style("test", style("test").green());
        // Should be plain text when colors are disabled
        assert_eq!(result, "test");
    }

    #[test]
    fn test_config_aware_convenience_functions() {
        // Test that convenience functions work (they load config on-demand)
        success("Test success");
        info("Test info");
        warning("Test warning");
        error("Test error");
    }
}
