use crate::{
    error::Error,
    service::{build_system::BuildSystem, init::project_init, vcs::VersionControl},
};
use anyhow::Context;
use clap::Args;
use std::path::{Path, PathBuf};

/// Create a new crame project
#[derive(Debug, Args)]
pub struct Command {
    /// Path to new project
    pub path: PathBuf,

    /// Build system to use
    #[clap(long = "build", short, value_enum, default_value = "crame")]
    pub build_system: BuildSystem,

    /// Version control system
    #[clap(long, value_enum, default_value = "git")]
    pub vcs: VersionControl,
}

impl Command {
    #[tracing::instrument(level = "debug")]
    pub fn run(&self) -> anyhow::Result<()> {
        let path = new_absolute_path(&self.path)?;

        create_project_dir(&path)?;

        let path = normalize_path(&path)?;

        project_init(path, self.build_system, self.vcs)
    }
}

fn new_absolute_path(path: &Path) -> anyhow::Result<PathBuf> {
    let path = std::env::current_dir()
        .with_context(|| format!("unable to create absolute path from: `{}`", path.display()))?
        .join(path);

    tracing::debug!("Checking target path: `{}`", path.display());

    if path.exists() {
        anyhow::bail!(Error::Conflict(path));
    }

    Ok(path)
}

fn create_project_dir(path: &Path) -> anyhow::Result<()> {
    tracing::debug!("Creating project directory: `{}`", path.display());

    std::fs::create_dir_all(path).with_context(|| Error::CreateDir(path.to_owned()))
}

fn normalize_path(path: &Path) -> anyhow::Result<PathBuf> {
    tracing::debug!("Normalizing path: `{}`", path.display());

    path.canonicalize()
        .with_context(|| Error::Normalize(path.to_owned()))
}
