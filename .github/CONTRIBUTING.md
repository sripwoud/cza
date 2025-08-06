# Contributing to create-zk-app (cza)

Welcome! We're excited that you're interested in contributing to cza. This document outlines the development setup, coding guidelines, and contribution process.

## Development Setup

### Prerequisites

- **Rust** (latest stable version)
- **Git** for version control

### Initial Setup

1. **Clone the repository**:
   ```bash
   git clone https://github.com/sripwoud/cza.git
   cd cza
   ```

2. **Run the setup script** (this configures all development tools):
   ```bash
   ./setup
   ```

The `./setup` script will install and configure:

- **[mise](https://mise.jdx.dev/)** - Runtime and task manager
- **[hk](https://github.com/comtrya/hk)** - Git hooks manager
- **[dprint](https://dprint.dev/)** - Code formatting
- **[biome](https://biomejs.dev/)** - JavaScript/TypeScript linting
- **[convco](https://convco.github.io/)** - Conventional commits (main branch only)

### Development Workflow

#### Available Commands

After setup, use mise to run development tasks:

```bash
# Run tests
mise run test

# Format code
mise run fmt

# Lint code  
mise run lint

# Build the project
mise run build

# Run the CLI locally
cargo run -- list
cargo run -- new noir-vite test-project
```

#### Pre-commit Automation

The git hooks (via hk) will automatically run before each commit:

- **Code formatting** with dprint
- **Rust linting** with clippy
- **Tests** to ensure nothing breaks
- **Conventional commits** enforcement (main branch only)

If you bypass the hooks, don't worry! Our **GitHub Actions CI** will still check:

- ✅ Code formatting (dprint)
- ✅ Linting (clippy + biome)
- ✅ Tests (all 27 tests must pass)
- ✅ Build validation

## Code Guidelines

### Rust Style

- Follow standard Rust conventions and idioms
- Use `cargo fmt` for consistent formatting (configured via dprint)
- Address all `cargo clippy` warnings
- Write comprehensive tests for new functionality
- Document public APIs with rustdoc comments

### CLI Design Principles

We follow best practices from [Command Line Interface Guidelines](https://clig.dev/):

- **Clear, actionable output** with proper colors and formatting
- **Helpful error messages** that guide users to solutions
- **Consistent command patterns** across all subcommands
- **Fast execution** - operations should feel instantaneous when possible

### Testing Strategy

We use a comprehensive testing approach inspired by [Foundry](https://github.com/foundry-rs/foundry):

- **Unit tests** in `src/` files (`#[cfg(test)]` modules) for internal logic
- **Integration tests** in `tests/` directory for CLI command testing
- **Black box testing** using `assert_cmd` for full CLI validation

All tests must pass before merging:

```bash
mise run test  # Runs all 27 tests via nextest
```

## Contributing Process

### 1. Create an Issue (Optional)

For major changes or new features, consider creating an issue first to discuss the approach.

### 2. Fork & Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 3. Development

- Make your changes following the code guidelines
- Add tests for new functionality
- Ensure all existing tests still pass
- Update documentation if needed

### 4. Commit

We use conventional commits for semantic versioning:

```bash
# Examples
git commit -m "feat: add cairo-vite template support"
git commit -m "fix: handle empty project names gracefully" 
git commit -m "docs: update installation instructions"
```

On the main branch, convco will automatically format your commits.

### 5. Submit Pull Request

- Ensure your branch is up to date with main
- All tests pass locally: `mise run test`
- Code is formatted: `mise run fmt`
- No clippy warnings: `mise run lint`

Our CI will automatically verify:

- Tests pass (27 tests across unit, integration, and CLI tests)
- Code is properly formatted
- No linting issues
- Build succeeds

## Types of Contributions

### 🚀 New Templates

The most impactful contributions! Create templates for new ZK frameworks:

- Templates should follow the structure in [cza-noir-vite](https://github.com/sripwoud/cza-noir-vite)
- Include complete development stack (ZK framework + Vite + TanStack)
- Add to `cli/templates.toml` registry
- Test end-to-end project generation

### 🔧 CLI Features

- New commands (config, update, etc.)
- Enhanced template discovery
- Improved error handling and user experience
- Performance optimizations

### 📚 Documentation

- README improvements
- API documentation
- Template documentation
- Example projects

### 🐛 Bug Fixes

- CLI behavior fixes
- Template generation issues
- Cross-platform compatibility
- Error handling improvements

## Development Environment Details

### Project Structure

```
cza/
├── cli/                    # Main CLI crate
│   ├── src/
│   │   ├── cmd/           # Command implementations
│   │   ├── output.rs      # Foundry-inspired CLI output
│   │   └── main.rs        # CLI entry point
│   ├── tests/             # Integration tests
│   └── templates.toml     # Embedded template registry
├── .github/
│   ├── workflows/         # CI/CD automation
│   └── CONTRIBUTING.md    # This file
├── mise.toml              # Development tasks
├── hk.pkl                 # Git hooks config
└── dprint.json           # Formatting config
```

### Architecture

- **CLI Framework**: Clap v4 with derive macros
- **Template Engine**: cargo-generate with Handlebars
- **Output System**: Foundry-inspired with console colors
- **Error Handling**: anyhow for user-friendly messages
- **Testing**: nextest with assert_cmd for CLI testing

## Getting Help

- **Questions**: Open a GitHub Discussion
- **Bugs**: Create an Issue with reproduction steps
- **Ideas**: Create an Issue with the enhancement label

Thank you for contributing to cza! 🚀
