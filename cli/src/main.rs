use anyhow::Result;
use clap::Parser;
use cza::{
    cmd::{
        config::ConfigCommand, list::ListCommand, new::NewCommand, update::UpdateCommand, Execute,
    },
    config::Config,
    Cli, Command,
};
use log::debug;

fn init_logging(cli_verbose: Option<bool>) -> Result<()> {
    // Don't override if RUST_LOG is already set by user
    if std::env::var("RUST_LOG").is_err() {
        // Load config to get default verbose setting
        let config = Config::load().unwrap_or_default();

        // Determine level: CLI flag > config > default
        let verbose = cli_verbose.unwrap_or(config.development.verbose);
        let level = if verbose { "debug" } else { "info" };

        std::env::set_var("RUST_LOG", format!("cza={}", level));
    }

    env_logger::init();
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    // Initialize logging - for now we don't have CLI verbose flags, so pass None
    if let Err(e) = init_logging(None) {
        eprintln!("Failed to initialize logging: {}", e);
        std::process::exit(1);
    }

    debug!("CLI arguments parsed: {:#?}", cli);

    match &cli.command {
        Command::New(args) => {
            debug!("Executing new command");
            NewCommand.execute(args)
        }
        Command::List(args) => {
            debug!("Executing list command");
            ListCommand.execute(args)
        }
        Command::Config(args) => {
            debug!("Executing config command");
            ConfigCommand.execute(args)
        }
        Command::Update(args) => {
            debug!("Executing update command");
            UpdateCommand.execute(args)
        }
    }
}
