//! # create-zk-app (cza)
//!
//! A CLI tool for scaffolding zero-knowledge application projects with modern development tooling and best practices built-in.
//!
//! ## What is cza?
//!
//! **cza** (create-zk-app) is like `create-react-app` but for zero-knowledge applications. It provides curated, opinionated project templates that combine ZK frameworks with modern frontend stacks, complete with development tooling and CI/CD setup.
//!
//! ## Features
//!
//! - **🚀 Quick Project Setup** - Generate complete ZK projects in seconds
//! - **🎯 Opinionated Templates** - Pre-configured with best practices and modern tooling
//! - **🔧 Development Ready** - Includes mise, git hooks, formatting, linting, and CI/CD
//! - **🌐 Full Stack** - ZK circuits + modern frontend (Vite + TanStack)
//! - **🛠️ Smart Setup** - Automatic dependency installation and project initialization
//!
//! ## Quick Start
//!
//! ### Installation
//!
//! Install via cargo:
//!
//! ```bash
//! cargo install cza
//! ```
//!
//! Or use npx without installation:
//!
//! ```bash
//! npx create-zk-app
//! ```
//!
//! ### Usage
//!
//! List available templates:
//!
//! ```bash
//! cza list
//! ```
//!
//! Create a new project:
//!
//! ```bash
//! cza new noir-vite my-zk-app
//! cd my-zk-app
//! mise run dev
//! ```
//!
//! ## Available Templates
//!
//! Templates are hosted at [cza-templates](https://github.com/sripwoud/cza-templates).
//!
//! | Template | ZK Framework | Frontend |
//! |----------|--------------|----------|
//! | `cairo-vite` | [Cairo](https://www.cairo-lang.org) | [Vite](https://vitejs.dev/) + [React](https://react.dev/) + [TanStack](https://tanstack.com/) |
//! | `noir-vite` | [Noir](https://noir-lang.org/) | [Vite](https://vitejs.dev/) + [React](https://react.dev/) + [TanStack](https://tanstack.com/) |
//!
//! More templates coming soon: Risc0, o1js, and more!
//!
//! ## How It Works
//!
//! **cza** uses:
//!
//! - **[cargo-generate](https://github.com/cargo-generate/cargo-generate)** for template processing
//! - **Template repositories** hosted on GitHub with Handlebars variable substitution
//! - **Automatic setup** via post-generation scripts that install tools and configure the environment
//! - **Embedded registry** for fast template discovery and listing
//!
//! Each template is a complete, working project that includes:
//!
//! - ZK framework setup (Noir, Cairo, etc.)
//! - Modern frontend stack (Vite + TanStack Router/Query/Form)
//! - Development tooling (mise, dprint, biome, convco)
//! - Git hooks and CI/CD workflows
//! - Ready-to-use examples and documentation
//!
//! ## Modules
//!
//! - [`cmd`] - Command implementations (new, list, config, update)
//! - [`config`] - Configuration management
//! - [`output`] - Formatted terminal output
//! - [`template`] - Template registry and validation
//! - [`utils`] - Utility functions

pub mod cmd;
pub mod config;
pub mod output;
pub mod template;
pub mod utils;

use crate::cmd::{config::ConfigArgs, list::ListArgs, new::NewArgs, update::UpdateArgs};
use clap::{Parser, Subcommand};

/// CLI tool to create zero-knowledge applications
#[derive(Parser, Debug)]
#[command(
    name = "cza",
    version,
    author = "sripwoud",
    about = "CLI tool for scaffolding zero-knowledge application projects"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// Available commands for the CLI
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Create a new ZK application project
    New(NewArgs),
    /// List available templates and frameworks
    List(ListArgs),
    /// Configure global settings for the CLI
    Config(ConfigArgs),
    /// Update the CLI tool to the latest version
    Update(UpdateArgs),
}
