use clap::Parser;
use cza::{
    cmd::{
        config::ConfigCommand, list::ListCommand, new::NewCommand, update::UpdateCommand, Execute,
    },
    Cli, Command,
};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::New(args) => NewCommand.execute(&args),
        Command::List(args) => ListCommand.execute(&args),
        Command::Config(args) => ConfigCommand.execute(&args),
        Command::Update(args) => UpdateCommand.execute(&args),
    }
}
