use super::Execute;
use crate::output;
use anyhow::{Context, Result};
use clap::Args;
use log::debug;
use self_update::cargo_crate_version;

#[derive(Args, Debug)]
pub struct UpdateArgs;

pub struct UpdateCommand;

impl Execute for UpdateCommand {
    type Args = UpdateArgs;

    fn run(&self, _args: &Self::Args) -> Result<()> {
        debug!("Starting update command");

        output::step("Checking for updates...");

        // Get current version from Cargo.toml
        let current_version = cargo_crate_version!();
        debug!("Current version: {}", current_version);

        // Check for updates from GitHub releases
        let releases = self_update::backends::github::ReleaseList::configure()
            .repo_owner("sripwoud")
            .repo_name("cza")
            .build()
            .context("Failed to configure GitHub release checker")?
            .fetch()
            .context("Failed to fetch release information from GitHub")?;

        let latest_release = releases.first().context("No releases found")?;

        let latest_version = &latest_release.version;
        debug!("Latest version: {}", latest_version);

        // Compare versions
        if current_version == latest_version {
            output::success(&format!("Already up to date (v{})", current_version));
            return Ok(());
        }

        output::info(&format!(
            "Found newer version: {} → {}",
            current_version, latest_version
        ));
        output::step("Downloading and installing update...");

        // Perform the update
        let update_result = self_update::backends::github::Update::configure()
            .repo_owner("sripwoud")
            .repo_name("cza")
            .bin_name("cza")
            .show_download_progress(true)
            .current_version(current_version)
            .build()
            .context("Failed to configure updater")?
            .update()
            .context("Failed to download and install update");

        match update_result {
            Ok(update_status) => match update_status {
                self_update::Status::UpToDate(version) => {
                    output::success(&format!("Already up to date (v{})", version));
                }
                self_update::Status::Updated(version) => {
                    output::success(&format!("Successfully updated to v{}", version));
                    output::info(
                        "Restart your terminal or run 'cza --version' to verify the update",
                    );
                }
            },
            Err(e) => {
                output::error(&format!("Update failed: {}", e));
                output::info("You can manually download the latest version from:");
                output::plain("https://github.com/sripwoud/cza/releases/latest");
                return Err(e);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_command_structure() {
        // Test that the command can be instantiated
        let command = UpdateCommand;
        let args = UpdateArgs;

        // Verify the types implement required traits
        assert_eq!(format!("{:?}", args), "UpdateArgs");

        // This will test the command structure but not run the actual update
        // since that would require network access and real GitHub releases
        let _command = command;
    }

    #[test]
    fn test_update_args_debug() {
        let args = UpdateArgs;
        let debug_output = format!("{:?}", args);
        assert_eq!(debug_output, "UpdateArgs");
    }

    // Note: Integration testing for actual update functionality
    // should be done manually or with network access in CI
    // since it requires real GitHub API calls and binary downloads
}
