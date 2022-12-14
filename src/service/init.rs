use super::{
    build_system::{init_build_system, BuildSystem},
    config::Config,
    vcs::{init_vcs, VersionControl},
};
use crate::util::error::Error;

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
    build_system: BuildSystem,
    vcs: VersionControl,
) -> anyhow::Result<()> {
    let depth = path_depth(&path);

    create_directories(&mut path, depth)?;

    create_program_files(&mut path, depth)?;

    init_vcs(vcs, &mut path)?;

    init_build_system(build_system, &mut path)?;

    let config = Config::init_from_path(&path, build_system)?;
    config.save_in_dir(&mut path)?;

    tracing::info!("Created new crame project: `{}`", config.package.name);

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

        tracing::debug!(?path, "Creating directory");

        fs::create_dir(&path).with_context(|| Error::CreateDir(path.to_owned()))?;

        remove_path_depth(path, new_depth - depth);
    }

    Ok(())
}

fn create_program_files(path: &mut PathBuf, depth: usize) -> anyhow::Result<()> {
    for source in PROJECT_SOURCE_FILES {
        path.push(source.path);
        let new_depth = path_depth(path);

        tracing::debug!(?path, "Adding source file");

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test::{template_dir, testdir_and_path};

    #[test]
    fn directories_exist() {
        let (_dir, dir_path) = testdir_and_path();
        let expected_dirs = &[
            dir_path.join("src"),
            dir_path.join("lib"),
            dir_path.join("tests"),
            dir_path.join("tests").join("unit"),
        ];

        project_init(dir_path, BuildSystem::Just, VersionControl::Git)
            .expect("no error in project_init");

        for dir in expected_dirs {
            assert!(dir.exists(), "directory should exist: `{}`", dir.display());
        }
    }

    #[test]
    fn program_files_exist() {
        let (_dir, dir_path) = testdir_and_path();
        let expected_files = program_file_paths(&dir_path);

        project_init(dir_path, BuildSystem::Just, VersionControl::Git)
            .expect("no error in project_init");

        for file in expected_files {
            assert!(file.exists(), "file should exist: `{}`", file.display());
        }
    }

    #[test]
    fn program_files_contents() {
        let (_tmp_dir, tmp_dir_path) = testdir_and_path();
        let tmp_files = program_file_paths(&tmp_dir_path);

        let template_dir = template_dir();
        let template_files = program_file_paths(&template_dir);

        project_init(tmp_dir_path, BuildSystem::Just, VersionControl::Git)
            .expect("no error in project_init");

        for (template, created) in template_files.iter().zip(tmp_files.iter()) {
            let template_contents = file_contents(template);
            let created_contents = file_contents(created);
            assert_eq!(template_contents, created_contents);
        }
    }

    fn program_file_paths(dir: &Path) -> Vec<PathBuf> {
        vec![
            dir.join("src").join("main.c"),
            dir.join("tests").join("run.c"),
            dir.join("tests").join("test_all.c"),
            dir.join("tests").join("unit").join("it_works.c"),
        ]
    }

    fn file_contents(path: &Path) -> String {
        fs::read_to_string(path)
            .map_err(|_| format!("read file: `{}`", path.display()))
            .unwrap()
    }
}
