use crate::error::Error;

use anyhow::Context;

use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn absolute_path(path: &Path) -> anyhow::Result<PathBuf> {
    let mut output = std::env::current_dir().context("unable to locate current directory")?;
    output.push(path);

    Ok(output)
}

pub fn create_project_dir(path: &Path) -> anyhow::Result<()> {
    tracing::debug!(?path, "Creating project directory");

    fs::create_dir_all(path).with_context(|| Error::CreateDir(path.to_owned()))
}

pub fn normalize_path(path: &Path) -> anyhow::Result<PathBuf> {
    tracing::debug!(?path, "Normalizing");

    path.canonicalize()
        .with_context(|| Error::Normalize(path.to_owned()))
}
