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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockCommand {
        should_fail: bool,
    }

    impl Execute for MockCommand {
        type Args = ();

        fn run(&self, _args: &Self::Args) -> Result<()> {
            if self.should_fail {
                Err(anyhow!("Mock error for testing"))
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn test_execute_success() {
        let cmd = MockCommand { should_fail: false };
        // This should not panic or exit
        cmd.execute(&());
    }

    // Note: We can't easily test the error path that calls std::process::exit(1)
    // in a unit test as it would terminate the test process
}
