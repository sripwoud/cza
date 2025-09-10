mod args;
pub mod cmd;
pub mod config;
pub mod output;
pub mod template;
mod utils;

use crate::cmd::{config::ConfigArgs, list::ListArgs, new::NewArgs, update::UpdateArgs};
use clap::{Parser, Subcommand};

/// CLI tool to create zero-knowledge applications
#[derive(Parser)]
#[command(
    name = "create-zk-app",
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "CLI tool to create zero-knowledge applications"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
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
