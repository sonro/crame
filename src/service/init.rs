use super::{build_system::BuildSystem, vcs::VersionControl};
use std::path::Path;

pub fn project_init(
    _path: &Path,
    _build_system: BuildSystem,
    _vcs: VersionControl,
) -> anyhow::Result<()> {
    Ok(())
}
