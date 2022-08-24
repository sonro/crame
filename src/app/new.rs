use crate::service::{build_system::BuildSystem, init::project_init, vcs::VersionControl};
use clap::Args;
use std::path::PathBuf;

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
    pub fn run(&self) -> anyhow::Result<()> {
        project_init(&self.path, self.build_system, self.vcs)
    }
}
