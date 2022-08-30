use crate::error::Error;

use anyhow::Context;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use std::{fs, io::Write, path::PathBuf};

#[derive(Debug, Clone, Copy, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildSystem {
    Just,
}

impl Default for BuildSystem {
    fn default() -> Self {
        BuildSystem::Just
    }
}

pub fn init_build_system(build_system: BuildSystem, path: &mut PathBuf) -> anyhow::Result<()> {
    match build_system {
        BuildSystem::Just => create_justfile(path),
    }
}

const JUSTFILE_CONTENTS: &[u8] = include_bytes!("../../template/justfile");

fn create_justfile(path: &mut PathBuf) -> anyhow::Result<()> {
    path.push("justfile");
    tracing::debug!(?path, "Writing justfile");

    let mut dest = fs::File::create(&path).with_context(|| Error::CreateFile(path.to_owned()))?;

    dest.write_all(JUSTFILE_CONTENTS)
        .with_context(|| Error::WriteFile(path.to_owned()))?;

    path.pop();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test::{template_dir, testdir_and_path};

    #[test]
    fn just() {
        let (dir, mut dir_path) = testdir_and_path();
        let just_path = dir.path().join("justfile");

        init_build_system(BuildSystem::Just, &mut dir_path).expect("initialize build system");
        let contents = fs::read_to_string(&just_path).expect("read created justfile");

        let template_path = template_dir().join("justfile");
        let expected = fs::read_to_string(&template_path).expect("read template justfile");
        assert_eq!(expected, contents);
    }
}
