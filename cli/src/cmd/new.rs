use super::Execute;
use clap::{Args, ValueEnum};
use flate2::bufread::GzDecoder;
use git2::Repository;
use reqwest::blocking as http;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt::Display,
    fs::create_dir_all,
    path::Path,
    process::{exit, Command},
};
use tar::Archive;

/// Metadata from the npm registry
#[derive(Deserialize, Debug)]
struct Dist {
    tarball: String,
}

#[derive(Deserialize, Debug)]
struct VersionInfo {
    dist: Dist,
}

#[derive(Deserialize, Debug)]
struct NpmMetadata {
    #[serde(rename = "dist-tags")]
    dist_tags: HashMap<String, String>,
    versions: HashMap<String, VersionInfo>,
}

/// Allowed template values
#[derive(Clone, Debug, ValueEnum)]
pub enum Template {
    Circom,
}

impl Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Template::Circom => write!(f, "circom"),
        }
    }
}

#[derive(Args)]
pub struct NewArgs {
    /// The name of the new project
    project_name: String,

    /// Specify the template to use
    #[arg(value_enum)]
    template: Option<Template>,
}

pub struct NewCommand;

impl NewCommand {
    /// Fetch the latest tarball URL for the given template from npm registry
    fn get_latest_tarball_url(template: &Template) -> Result<String, Box<dyn std::error::Error>> {
        let package_name = format!("cza-{}-template", template);
        let registry_url = format!("https://registry.npmjs.org/{}", package_name);

        let metadata: NpmMetadata = http::get(&registry_url)?.json()?;

        let latest_version = metadata
            .dist_tags
            .get("latest")
            .ok_or("No 'latest' tag found")?;

        let tarball_url = metadata
            .versions
            .get(latest_version)
            .ok_or("No metadata for this version")?
            .dist
            .tarball
            .clone();

        println!("Latest version: {}", latest_version);
        println!("Tarball URL: {}", tarball_url);

        Ok(tarball_url)
    }

    /// Download and extract the tarball into the project directory
    fn download_and_extract_tarball(
        url: &str,
        project_dir: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = reqwest::blocking::get(url)?;
        let tarball = response.bytes()?;

        let tar_gz = GzDecoder::new(&tarball[..]);
        let mut archive = Archive::new(tar_gz);

        // Unpack tarball, stripping the `package/` prefix
        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?;

            // Skip the `package/` prefix and re-map paths
            let stripped_path = path.strip_prefix("package").unwrap_or(&path);
            let dest_path = Path::new(project_dir).join(stripped_path);

            // Ensure parent directories exist
            if let Some(parent) = dest_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // Extract file to the new location
            entry.unpack(dest_path)?;
        }

        println!("Template successfully extracted into {}", project_dir);
        Ok(())
    }
}

impl Execute for NewCommand {
    type Args = NewArgs;

    fn execute(&self, args: &Self::Args) {
        let project_dir = &args.project_name;
        let template = match &args.template {
            Some(template) => template,
            None => panic!("Template must be specified"),
        };

        create_dir_all(project_dir).expect("Failed to create project directory");
        println!("Created project directory: {}", project_dir);

        let tarball_url = match Self::get_latest_tarball_url(template) {
            Ok(url) => url,
            Err(e) => {
                eprintln!("Failed to get tarball URL: {}", e);
                exit(1);
            }
        };

        if let Err(e) = Self::download_and_extract_tarball(&tarball_url, project_dir) {
            eprintln!("Failed to extract tarball: {}", e);
            exit(1);
        }

        match Repository::init(project_dir) {
            Ok(_) => println!("Initialized git repository in {}", project_dir),
            Err(e) => {
                eprintln!("Failed to initialize git repository: {}", e);
                exit(1);
            }
        }

        Command::new(format!("cd {}", project_dir));
        let setup_status = Command::new("./setup")
            .current_dir(project_dir)
            .status()
            .expect("Failed to run setup script");

        if !setup_status.success() {
            eprintln!("Setup script failed to run");
            exit(1);
        }

        println!("Setup script executed successfully.");
    }
}
