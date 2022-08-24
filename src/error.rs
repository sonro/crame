use colored::Colorize;
use std::{io, path::PathBuf, process};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("`{0}` already exists")]
    Conflict(PathBuf),
}

pub fn report_exit(err: &anyhow::Error, verbose: bool) {
    report(err, verbose);
    process::exit(exitcode_from_err(err));
}

pub fn report(err: &anyhow::Error, verbose: bool) {
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
            Error::Conflict(_) => exitcode::CANTCREAT,
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
        let path = PathBuf::new();
        let error = anyhow::anyhow!(Error::Conflict(path));
        let code = exitcode_from_err(&error);
        assert_eq!(exitcode::CANTCREAT, code);
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
}
