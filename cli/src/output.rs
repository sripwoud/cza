//! Foundry-inspired CLI output module with standardized messaging functions.
//!
//! This module provides consistent, professional CLI output with:
//! - Color support with automatic terminal detection
//! - Emoji support with ASCII fallbacks
//! - Consistent styling across all commands
//! - Terminal-aware formatting

use anyhow;
use console::{style, Emoji, Term};

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
}

impl Output {
    /// Create a new output manager
    pub fn new() -> Self {
        Self {
            term: Term::stdout(),
        }
    }

    /// Print a success message with green styling
    pub fn success(&self, message: &str) {
        let styled_message = style(message).green().bold();
        println!("{SUCCESS_EMOJI} {styled_message}");
    }

    /// Print an info message with blue styling
    pub fn info(&self, message: &str) {
        let styled_message = style(message).blue();
        println!("{INFO_EMOJI} {styled_message}");
    }

    /// Print a warning message with yellow styling
    pub fn warning(&self, message: &str) {
        let styled_message = style(message).yellow().bold();
        println!("{WARNING_EMOJI} {styled_message}");
    }

    /// Print an error message with red styling
    pub fn error(&self, message: &str) {
        let styled_message = style(message).red().bold();
        eprintln!("{ERROR_EMOJI} {styled_message}");
    }

    /// Print a step message with cyan styling (for progress indication)
    pub fn step(&self, message: &str) {
        let styled_message = style(message).cyan();
        println!("{STEP_EMOJI} {styled_message}");
    }

    /// Print a directory path with consistent styling
    pub fn directory(&self, path: &str) {
        let styled_path = style(path).magenta().bold();
        println!("{DIRECTORY_EMOJI} Location: {styled_path}");
    }

    /// Print next steps with consistent styling
    pub fn next_steps(&self, steps: &[&str]) {
        if steps.is_empty() {
            return;
        }

        println!();
        let styled_header = style("Next steps:").cyan().bold();
        println!("{NEXT_EMOJI} {styled_header}");

        for step in steps {
            let styled_step = style(step).dim();
            println!("  {styled_step}");
        }
    }

    /// Print a command suggestion with consistent styling
    pub fn command_example(&self, description: &str, command: &str) {
        let styled_desc = style(description).dim();
        let styled_command = style(command).green().bold();
        println!("  {styled_desc}: {styled_command}");
    }

    /// Print a header for sections
    pub fn header(&self, title: &str) {
        println!();
        let styled_title = style(title).bold().underlined();
        println!("{styled_title}");
        println!();
    }

    /// Print a plain message without styling (for regular content)
    pub fn plain(&self, message: &str) {
        println!("{message}");
    }

    /// Print a styled key-value pair
    pub fn key_value(&self, key: &str, value: &str) {
        let styled_key = style(key).bold();
        println!("   {styled_key}: {value}");
    }

    /// Print a template item with consistent styling
    pub fn template_item(&self, name: &str, description: &str) {
        let styled_name = style(name).green().bold();
        let styled_desc = style(description).dim();
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
        let styled_key = style(key).green().bold();
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

/// Global output instance for convenience functions
static OUTPUT: std::sync::LazyLock<Output> = std::sync::LazyLock::new(Output::new);

/// Convenience function for success messages
pub fn success(message: &str) {
    OUTPUT.success(message);
}

/// Convenience function for info messages
pub fn info(message: &str) {
    OUTPUT.info(message);
}

/// Convenience function for warning messages
pub fn warning(message: &str) {
    OUTPUT.warning(message);
}

/// Convenience function for error messages
pub fn error(message: &str) {
    OUTPUT.error(message);
}

/// Convenience function for step messages
pub fn step(message: &str) {
    OUTPUT.step(message);
}

/// Convenience function for directory messages
pub fn directory(path: &str) {
    OUTPUT.directory(path);
}

/// Convenience function for next steps
pub fn next_steps(steps: &[&str]) {
    OUTPUT.next_steps(steps);
}

/// Convenience function for command examples
pub fn command_example(description: &str, command: &str) {
    OUTPUT.command_example(description, command);
}

/// Convenience function for headers
pub fn header(title: &str) {
    OUTPUT.header(title);
}

/// Convenience function for plain messages
pub fn plain(message: &str) {
    OUTPUT.plain(message);
}

/// Convenience function for template items
pub fn template_item(name: &str, description: &str) {
    OUTPUT.template_item(name, description);
}

/// Convenience function for detailed template info
pub fn template_detailed(
    key: &str,
    name: &str,
    description: &str,
    frameworks: &[String],
    repository: &str,
) {
    OUTPUT.template_detailed(key, name, description, frameworks, repository);
}

/// Format and display anyhow errors using our consistent output system
pub fn format_error(err: &anyhow::Error) {
    let error_msg = err.to_string();

    // Handle specific error patterns with enhanced formatting
    if error_msg.contains("not found. Use 'cza list'") {
        // Split the template not found error for better formatting
        if let Some(template_part) = error_msg.split('.').next() {
            error(template_part);
            info("Use 'cza list' to see available templates.");
        } else {
            error(&error_msg);
        }
    } else if error_msg.contains("already exists") {
        // Handle directory exists errors
        error(&error_msg);
        info("Choose a different project name or remove the existing directory.");
    } else if error_msg.contains("Project name") {
        // Handle project name validation errors
        error(&error_msg);
        info("Project names can only contain alphanumeric characters, hyphens, and underscores.");
    } else {
        // Default error formatting
        error(&error_msg);
    }
}
