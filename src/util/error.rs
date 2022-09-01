use std::{io, path::PathBuf, process};

use colored::Colorize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid config file: `{0}`")]
    Config(PathBuf),

    #[error("unable to create directory: `{0}`")]
    CreateDir(PathBuf),

    #[error("unable to create project file: `{0}`")]
    CreateFile(PathBuf),

    #[error("`{0}` already exists")]
    Conflict(PathBuf),

    #[error("unable to normalize path: `{0}`")]
    Normalize(PathBuf),

    #[error("unable to read file: `{0}`")]
    ReadFile(PathBuf),

    #[error("cannot write to file: `{0}`")]
    WriteFile(PathBuf),
}

pub fn report_exit(err: &anyhow::Error, verbose: bool) {
    report(err, verbose);
    process::exit(exitcode_from_err(err));
}

pub fn report(err: &anyhow::Error, mut verbose: bool) {
    if let Some(Error::Config(_)) = err.downcast_ref() {
        verbose = true;
    }

    eprintln!("{}{} {}", "error".red().bold(), ":".white().bold(), err);

    if verbose {
        for cause in err.chain().skip(1) {
            eprintln!("{}{} {}", "cause".red(), ":".white(), cause);
        }
    }
}

fn exitcode_from_err(err: &anyhow::Error) -> exitcode::ExitCode {
    if let Some(err) = err.downcast_ref::<Error>() {
        match err {
            Error::Conflict(_)
            | Error::CreateDir(_)
            | Error::Normalize(_)
            | Error::CreateFile(_) => exitcode::CANTCREAT,
            Error::WriteFile(_) => exitcode::IOERR,
            Error::ReadFile(_) => exitcode::NOINPUT,
            Error::Config { .. } => exitcode::CONFIG,
        }
    } else if let Some(err) = err.downcast_ref::<io::Error>() {
        match err.kind() {
            io::ErrorKind::NotFound => exitcode::NOINPUT,
            io::ErrorKind::PermissionDenied => exitcode::NOPERM,
            io::ErrorKind::AlreadyExists => exitcode::CANTCREAT,
            _ => exitcode::IOERR,
        }
    } else {
        exitcode::SOFTWARE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exitcode_generic_error() {
        let code = exitcode_from_err(&anyhow::anyhow!("test error"));
        assert_eq!(exitcode::SOFTWARE, code);
    }

    #[test]
    fn exitcode_conflict() {
        assert_error_code(Error::Conflict(PathBuf::new()), exitcode::CANTCREAT);
    }

    #[test]
    fn exitcode_create_dir() {
        assert_error_code(Error::CreateDir(PathBuf::new()), exitcode::CANTCREAT);
    }

    #[test]
    fn exitcode_create_file() {
        assert_error_code(Error::CreateFile(PathBuf::new()), exitcode::CANTCREAT);
    }

    #[test]
    fn exitcode_normalize() {
        assert_error_code(Error::Normalize(PathBuf::new()), exitcode::CANTCREAT);
    }

    #[test]
    fn exitcode_write_file() {
        assert_error_code(Error::WriteFile(PathBuf::new()), exitcode::IOERR);
    }

    #[test]
    fn exitcode_read_file() {
        assert_error_code(Error::ReadFile(PathBuf::new()), exitcode::NOINPUT);
    }

    #[test]
    fn exitcode_config() {
        assert_error_code(Error::Config(PathBuf::new()), exitcode::CONFIG);
    }

    #[test]
    fn exitcode_generic_io_error() {
        assert_io_error_code(io::ErrorKind::BrokenPipe, exitcode::IOERR);
    }

    #[test]
    fn exitcode_not_found_io_error() {
        assert_io_error_code(io::ErrorKind::NotFound, exitcode::NOINPUT);
    }

    #[test]
    fn exitcode_permisson_denied_io_error() {
        assert_io_error_code(io::ErrorKind::PermissionDenied, exitcode::NOPERM);
    }

    #[test]
    fn exitcode_already_exists_io_error() {
        assert_io_error_code(io::ErrorKind::AlreadyExists, exitcode::CANTCREAT);
    }

    fn assert_io_error_code(kind: io::ErrorKind, code: exitcode::ExitCode) {
        let error = io::Error::from(kind);
        let error = anyhow::anyhow!(error);
        assert_eq!(code, exitcode_from_err(&error));
    }

    fn assert_error_code(error: Error, code: exitcode::ExitCode) {
        let error = anyhow::anyhow!(error);
        assert_eq!(code, exitcode_from_err(&error));
    }
}
