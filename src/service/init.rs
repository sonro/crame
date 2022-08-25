use super::{build_system::BuildSystem, vcs::VersionControl};
use crate::error::Error;

use anyhow::Context;

use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

/// Initialize project
///
/// Uses an owned [`PathBuf`] to reduce addition allocations joining paths.
#[tracing::instrument(level = "debug")]
pub fn project_init(
    mut path: PathBuf,
    _build_system: BuildSystem,
    _vcs: VersionControl,
) -> anyhow::Result<()> {
    let depth = path_depth(&path);

    create_directories(&mut path, depth)?;

    create_program_files(&mut path, depth)?;

    Ok(())
}

const PROJECT_DIRS: &[&str] = &["src", "lib", "tests", "tests/unit"];

struct TemplateFile {
    path: &'static str,
    contents: &'static [u8],
}

const PROJECT_SOURCE_FILES: &[TemplateFile] = &[
    TemplateFile {
        path: "src/main.c",
        contents: include_bytes!("../../template/src/main.c"),
    },
    TemplateFile {
        path: "tests/run.c",
        contents: include_bytes!("../../template/tests/run.c"),
    },
    TemplateFile {
        path: "tests/test_all.c",
        contents: include_bytes!("../../template/tests/test_all.c"),
    },
    TemplateFile {
        path: "tests/unit/it_works.c",
        contents: include_bytes!("../../template/tests/unit/it_works.c"),
    },
];

fn create_directories(path: &mut PathBuf, depth: usize) -> anyhow::Result<()> {
    for dir in PROJECT_DIRS {
        path.push(dir);
        let new_depth = path_depth(path);

        tracing::debug!("Creating directory: `{}`", path.display());

        fs::create_dir(&path).with_context(|| Error::CreateDir(path.to_owned()))?;

        remove_path_depth(path, new_depth - depth);
    }

    Ok(())
}

fn create_program_files(path: &mut PathBuf, depth: usize) -> anyhow::Result<()> {
    for source in PROJECT_SOURCE_FILES {
        path.push(source.path);
        let new_depth = path_depth(path);

        tracing::debug!("Adding source file: `{}`", path.display());

        let mut dest =
            fs::File::create(&path).with_context(|| Error::CreateFile(path.to_owned()))?;

        dest.write_all(source.contents)
            .with_context(|| Error::WriteFile(path.to_owned()))?;

        remove_path_depth(path, new_depth - depth);
    }

    Ok(())
}

fn path_depth(path: &Path) -> usize {
    path.iter().count()
}

fn remove_path_depth(path: &mut PathBuf, depth: usize) {
    for _ in 0..depth {
        path.pop();
    }
}
