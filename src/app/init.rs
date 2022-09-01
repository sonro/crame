use std::path::PathBuf;

use clap::Args;

use crate::{
    service::{build_system::BuildSystem, init::project_init, vcs::VersionControl},
    util::file::{absolute_path, create_project_dir, normalize_path},
};

/// Initialize a new crame project
#[derive(Debug, Args)]
pub struct Command {
    /// Path to project directory
    #[clap(default_value = ".")]
    pub path: PathBuf,

    /// Build system to use
    #[clap(long = "build", short, value_enum, default_value_t = BuildSystem::default())]
    pub build_system: BuildSystem,

    /// Version control system
    #[clap(long, value_enum, default_value_t = VersionControl::default())]
    pub vcs: VersionControl,
}

impl Command {
    #[tracing::instrument(level = "debug")]
    pub fn run(&self) -> anyhow::Result<()> {
        let path = absolute_path(&self.path)?;

        if !path.exists() {
            create_project_dir(&path)?;
        }

        let path = normalize_path(&path)?;

        project_init(path, self.build_system, self.vcs)
    }
}
