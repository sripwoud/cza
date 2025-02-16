pub mod config;
pub mod list;
pub mod new;
pub mod update;

pub trait Execute {
    type Args;

    fn execute(&self, args: &Self::Args);
}
