use crate::error;

use clap::ValueEnum;
use git2::Repository;

use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum VersionControl {
    Git,
    None,
}

impl Default for VersionControl {
    fn default() -> Self {
        Self::Git
    }
}

const GIT_IGNORES: &[&str] = &["target/"];

pub fn init_vcs(vcs: VersionControl, path: &mut PathBuf) -> anyhow::Result<()> {
    match vcs {
        VersionControl::None => (),
        VersionControl::Git => {
            let repo = existing_git_repo(path);

            if let Some(ref repo) = repo {
                tracing::debug!("Existing repo found at: `{}`", repo.path().display());
            } else {
                git_init(path);
            }

            let ignores = git_ignores(&repo)?;

            if !ignores.is_empty() {
                write_git_ignore(path, &ignores);
            }
        }
    }
    Ok(())
}

fn existing_git_repo(path: &Path) -> Option<Repository> {
    tracing::debug!("Checking for existing git repo`");

    match Repository::discover(path) {
        Ok(repo) => Some(repo),
        Err(_) => None,
    }
}

fn git_ignores(repo: &Option<Repository>) -> anyhow::Result<String> {
    let mut ignores = String::new();

    if let Some(repo) = repo {
        tracing::debug!("Checking if ignore rules already exist");

        for &rule in GIT_IGNORES {
            if !repo.is_path_ignored(rule)? {
                ignores += rule;
                ignores += "\n";
            } else {
                tracing::debug!(%rule, "Ignore already in existing repo");
            }
        }
    } else {
        tracing::debug!(?GIT_IGNORES, "Adding ignore rules");

        ignores = GIT_IGNORES.join("\n");
        ignores.push('\n');
    }

    Ok(ignores)
}

fn write_git_ignore(path: &mut PathBuf, contents: &str) {
    path.push(".gitignore");

    tracing::debug!(?path, "Writing ignore rules");
    if let Err(err) = fs::write(&path, contents) {
        error::report(&err.into(), true);
    }

    // non fatal error so guarentee path is reset
    path.pop();
}

fn git_init(path: &Path) {
    tracing::debug!("Initializing git repo");

    if let Err(err) = Repository::init(path) {
        error::report(&err.into(), true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::{tempdir, TempDir};

    #[test]
    fn none_empty_dir() {
        let dir = empty_tempdir();
        call_init_vcs(dir.path(), VersionControl::None);
        assert_no_dir_members(dir.path());
    }

    #[test]
    fn git_empty_dir_has_git_dir() {
        let dir = git_empty_dir_init_vcs();
        let git_dir_path = dir.path().join(".git");
        assert!(git_dir_path.exists());
    }

    #[test]
    fn git_empty_dir_has_ignore() {
        let dir = git_empty_dir_init_vcs();
        assert_full_ignore(dir.path());
    }

    #[test]
    fn git_existing_dir_no_error() {
        let dir = git_empty_dir_init_vcs();
        call_git_init_vcs(dir.path());
    }

    #[test]
    fn git_existing_parent_dir_has_no_git_dir() {
        let dir = git_init_tempdir_no_ignore();
        let subdir = git_subdir_init_vcs(dir.path());

        let git_dir_path = subdir.join(".git");
        assert!(!git_dir_path.exists());
    }

    #[test]
    fn git_existing_parent_dir_has_ignore() {
        let dir = git_init_tempdir_no_ignore();
        let subdir = git_subdir_init_vcs(dir.path());

        assert_full_ignore(&subdir);
    }

    #[test]
    fn git_existing_parent_dir_with_ignore() {
        let dir = git_empty_dir_init_vcs();
        let subdir = git_subdir_init_vcs(dir.path());

        let git_dir_path = subdir.join(".gitignore");
        assert!(!git_dir_path.exists());
    }

    fn git_empty_dir_init_vcs() -> TempDir {
        let dir = empty_tempdir();
        call_git_init_vcs(dir.path());
        dir
    }

    fn git_subdir_init_vcs(dir: &Path) -> PathBuf {
        let subdir = dir.join("subdir");
        fs::create_dir(&subdir).expect("create subdir");
        call_git_init_vcs(&subdir);
        subdir
    }

    fn git_init_tempdir_no_ignore() -> TempDir {
        let dir = empty_tempdir();
        git_init(dir.path());
        dir
    }

    fn empty_tempdir() -> TempDir {
        tempdir().expect("create temp directory")
    }

    fn call_git_init_vcs(dir: &Path) {
        call_init_vcs(dir, VersionControl::Git)
    }

    fn call_init_vcs(dir: &Path, vcs: VersionControl) {
        let mut dir_path = dir.to_owned();
        init_vcs(vcs, &mut dir_path).expect("no errors");
    }

    fn assert_no_dir_members(dir: &Path) {
        let dir_members = dir.read_dir().expect("accessing directroy members").count();
        assert_eq!(0, dir_members);
    }

    fn assert_full_ignore(dir: &Path) {
        let ignore_path = dir.join(".gitignore");
        let expected_contents = "target/\n";
        let contents = fs::read_to_string(ignore_path).expect(".gitignore created");
        assert_eq!(expected_contents, contents);
    }
}
