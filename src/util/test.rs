use std::path::PathBuf;

use tempfile::{tempdir, TempDir};

pub fn testdir_and_path() -> (TempDir, PathBuf) {
    let dir = tempdir().expect("create temporary directory");
    let path = dir.path().to_owned();
    (dir, path)
}

pub fn template_dir() -> PathBuf {
    let mut dir = PathBuf::from(file!());
    dir.pop();
    dir.pop();
    dir.pop();
    dir.push("template");
    dir
}
