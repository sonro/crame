use super::build_system::BuildSystem;
use crate::error::Error;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use std::{
    fs,
    path::{Path, PathBuf},
};

const CONFIG_PATH: &str = "Crame.toml";

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub package: Package,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub build_system: BuildSystem,
}

impl Config {
    pub fn init_from_path<P: AsRef<Path>>(
        path: P,
        build_system: BuildSystem,
    ) -> anyhow::Result<Self> {
        tracing::debug!("Initalizing config");

        let err_closure = || {
            anyhow::anyhow!(
                "cannot determine package name from path: `{}`",
                path.as_ref().display()
            )
        };
        let name = path
            .as_ref()
            .file_name()
            .ok_or_else(err_closure)?
            .to_str()
            .ok_or_else(err_closure)?
            .to_owned();

        let mut config = Self::default();
        config.package.name = name;
        config.package.build_system = build_system;

        tracing::debug!(?config, "Initialized");

        Ok(config)
    }

    #[allow(unused)]
    pub fn load_from_dir(dir: &mut PathBuf) -> anyhow::Result<Self> {
        dir.push(CONFIG_PATH);
        let config = Self::load_from_path(dir)?;
        dir.pop();

        Ok(config)
    }

    pub fn load_from_path(path: &Path) -> anyhow::Result<Self> {
        tracing::debug!(?path, "Loading config");

        let contents = fs::read(path).with_context(|| Error::ReadFile(path.to_owned()))?;
        let config = toml::from_slice(&contents).with_context(|| Error::Config(path.to_owned()))?;

        Ok(config)
    }

    pub fn save_in_dir(&self, dir: &mut PathBuf) -> anyhow::Result<()> {
        dir.push(CONFIG_PATH);
        self.save(dir)?;
        dir.pop();

        Ok(())
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        tracing::debug!(?path, "Saving config");

        let contents = toml::to_string_pretty(self)?;
        fs::write(path, contents).with_context(|| Error::WriteFile(path.to_owned()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test::testdir_and_path;

    use tempfile::TempDir;

    #[test]
    fn init_from_path_error() {
        let path = "bad/path/..";
        let err = Config::init_from_path(path, BuildSystem::Just).expect_err("error init config");
        let msg = format!("cannot determine package name from path: `{}`", path);
        assert_eq!(msg, err.to_string());
    }

    #[test]
    fn init_from_path_success() {
        let name = "testname";
        let path = PathBuf::from("good/path/").join(name);
        let config = Config::init_from_path(&path, BuildSystem::Just).expect("init config");

        assert!(matches!(config.package.build_system, BuildSystem::Just));
        assert_eq!(name, config.package.name);
    }

    #[test]
    fn load_from_dir_success() {
        let (dir, _) = create_good_toml_file();
        let config = Config::load_from_dir(&mut dir.path().to_owned()).expect("valid config");
        assert_config(&config);
    }

    #[test]
    fn load_from_dir_error_invalid() {
        let (dir, _) = create_bad_toml_file();
        let err = Config::load_from_dir(&mut dir.path().to_owned()).expect_err("invalid config");
        assert_config_error(&err);
    }

    #[test]
    fn load_from_path_success() {
        let (_dir, config_path) = create_good_toml_file();
        let config = Config::load_from_path(&config_path).expect("valid config");
        assert_config(&config);
    }

    #[test]
    fn load_from_path_error_invalid() {
        let (_dir, config_path) = create_bad_toml_file();
        let err = Config::load_from_path(&config_path).expect_err("invalid config");
        assert_config_error(&err);
    }

    #[test]
    fn save_success() {
        let (_dir, path) = create_toml_path();
        test_config().save(&path).expect("save config");
        let contents = fs::read_to_string(&path).expect("read saved config");
        assert_eq!(GOOD_TOML, contents);
    }

    #[test]
    fn save_in_dir_success() {
        let (dir, path) = create_toml_path();
        test_config()
            .save_in_dir(&mut dir.path().to_owned())
            .expect("save config");
        let contents = fs::read_to_string(&path).expect("read saved config");
        assert_eq!(GOOD_TOML, contents);
    }

    const GOOD_TOML: &str = r#"[package]
name = 'testname'
build_system = 'just'
"#;

    const BAD_TOML: &str = r#"[package]
nam = 'testname'
build_system = 'just'
"#;

    fn test_config() -> Config {
        Config {
            package: Package {
                name: "testname".into(),
                build_system: BuildSystem::Just,
            },
        }
    }

    fn create_good_toml_file() -> (TempDir, PathBuf) {
        create_toml_file(GOOD_TOML.as_bytes())
    }

    fn create_bad_toml_file() -> (TempDir, PathBuf) {
        create_toml_file(BAD_TOML.as_bytes())
    }

    fn create_toml_file(contents: &[u8]) -> (TempDir, PathBuf) {
        let (dir, path) = create_toml_path();
        fs::write(&path, contents).expect("write to file");
        (dir, path)
    }

    fn create_toml_path() -> (TempDir, PathBuf) {
        let (dir, mut path) = testdir_and_path();
        path.push("Crame.toml");
        (dir, path)
    }

    fn assert_config(config: &Config) {
        let expected = test_config();
        assert_eq!(expected.package.name, config.package.name);
    }

    fn assert_config_error(err: &anyhow::Error) {
        let err = err.downcast_ref::<Error>().expect("Config error");
        assert!(matches!(err, Error::Config(_)));
    }
}
