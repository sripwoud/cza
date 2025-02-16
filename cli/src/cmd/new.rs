use super::Execute;
use clap::Args;

#[derive(Args)]
pub struct NewArgs {
    /// The name of the new project
    project_name: String,

    /// Specify the template to use
    #[arg(long)]
    template: Option<String>,
}

pub struct NewCommand;

impl Execute for NewCommand {
    type Args = NewArgs;

    fn execute(&self, args: &Self::Args) {
        println!("Creating a new project with name: {}", args.project_name);
        // Implement the logic for the `new` command here
    }
}
