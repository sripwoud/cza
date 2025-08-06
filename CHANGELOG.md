# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- CLI tool for scaffolding zero-knowledge application projects
- Support for Noir + Vite + TanStack template
- Automatic project setup with mise, git hooks, and dependencies
- Template discovery and listing functionality
- Foundry-inspired CLI output with colored messages
- Integration with cargo-generate for template processing

### Features

- `cza new <template> <project-name>` - Create new ZK projects from templates
- `cza list` - List available project templates
- Automatic template variable substitution (project name, etc.)
- Post-generation setup script execution
- Template registry system with embedded templates.toml
