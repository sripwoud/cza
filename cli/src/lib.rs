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
