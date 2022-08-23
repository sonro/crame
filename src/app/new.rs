use clap::{Args, ValueEnum};
use std::path::PathBuf;

/// Create a new crame project
#[derive(Debug, Args)]
pub struct Command {
    /// Path to new project
    pub path: PathBuf,

    /// Build system to use
    #[clap(long = "build", short, value_enum)]
    pub build_system: Option<BuildSystem>,

    /// Version control system
    #[clap(long, value_enum)]
    pub vcs: Option<VersionControl>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum VersionControl {
    Git,
    None,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum BuildSystem {
    Crame,
    Just,
    Make,
}

impl Command {
    pub fn run(&self) -> anyhow::Result<()> {
        anyhow::bail!("unimplemented");
    }
}
