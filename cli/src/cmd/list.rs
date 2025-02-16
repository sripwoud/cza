use super::Execute;
use clap::Args;

#[derive(Args)]
pub struct ListArgs {
    /// List available templates
    #[arg(long)]
    templates: bool,
}

pub struct ListCommand;

impl Execute for ListCommand {
    type Args = ListArgs;

    fn execute(&self, args: &ListArgs) {
        if args.templates {
            println!("Listing available templates...");
            // Implement the logic to list templates
        }
    }
}
