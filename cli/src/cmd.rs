use crate::output;
use anyhow::Result;

pub mod config;
pub mod list;
pub mod new;
pub mod update;

pub trait Execute {
    type Args;

    fn run(&self, args: &Self::Args) -> Result<()>;

    fn execute(&self, args: &Self::Args) {
        if let Err(e) = self.run(args) {
            output::format_error(&e);
            std::process::exit(1);
        }
    }
}
