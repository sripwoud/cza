# Configuration

**cza** provides a comprehensive configuration system to customize your development experience and set global preferences for project generation.

## Configuration Location

Configuration is stored in an XDG-compliant location:

```
~/.config/cza/config.toml
```

The configuration file uses TOML format and is automatically created with default values when first accessed.

## Configuration Sections

### User Preferences

Control default values and personal information used in generated projects:

```toml
[user]
author = "Your Name" # Default author for new projects
email = "your.email@example.com" # Default email for git configuration
git_init = true # Automatically initialize git repositories
default_template = "noir-vite" # Default template when none specified
```

### Development Settings

Customize CLI behavior and output formatting:

```toml
[development]
verbose = false # Enable debug-level logging output
color = true # Use colored output in terminal
confirm_overwrite = true # Prompt before overwriting existing directories
```

### Post-Generation Behavior

Control what happens after a project is generated:

```toml
[post_generation]
auto_install_deps = true # Automatically run mise install after generation
auto_setup_hooks = true # Automatically run hk install to set up git hooks
open_editor = false # Open the project in your default editor after generation
```

## Configuration Commands

### View Configuration

```bash
# Show current configuration file path
cza config path

# List all configuration values
cza config list

# Get a specific configuration value
cza config get user.author
cza config get development.verbose
```

### Modify Configuration

```bash
# Set configuration values
cza config set user.author "John Doe"
cza config set user.email "john@example.com"
cza config set development.verbose true
cza config set post_generation.auto_install_deps false

# Reset configuration to defaults
cza config reset
```

## Environment Variable Override

The verbose logging setting can be overridden using the `RUST_LOG` environment variable:

```bash
# Override to debug level (most verbose)
RUST_LOG=debug cza new noir-vite my-project

# Override to error level only
RUST_LOG=error cza list

# Override to info level
RUST_LOG=info cza config list
```

## Configuration Examples

### Minimal Setup for CI/CD

For automated environments where you want minimal prompts:

```bash
cza config set development.confirm_overwrite false
cza config set post_generation.auto_install_deps false
cza config set post_generation.auto_setup_hooks false
```

### Development-Friendly Setup

For active development with full automation:

```bash
cza config set user.author "Your Name"
cza config set user.email "your@email.com"
cza config set development.verbose true
cza config set post_generation.auto_install_deps true
cza config set post_generation.auto_setup_hooks true
cza config set default_template "noir-vite"
```

### Team Consistency

Set default template for team standardization:

```bash
cza config set default_template "cairo-vite"
cza config set user.git_init true
cza config set post_generation.auto_install_deps true
```

## Configuration File Example

Here's a complete example configuration file:

```toml
[user]
author = "Alice Developer"
email = "alice@example.com"
git_init = true
default_template = "noir-vite"

[development]
verbose = false
color = true
confirm_overwrite = true

[post_generation]
auto_install_deps = true
auto_setup_hooks = true
open_editor = false
```

## Configuration Integration

These settings are automatically used by cza commands:

- **`cza new`**: Uses `user.*` settings for project setup, `development.confirm_overwrite` for safety, and `post_generation.*` settings for automation
- **All commands**: Respect `development.color` for output formatting and `development.verbose` for logging level
- **Template selection**: Falls back to `user.default_template` when no template is specified

The configuration system ensures consistent behavior across all cza operations while allowing fine-grained control over CLI behavior and project generation preferences.
