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

fn exitcode_from_err(err: &anyhow::Error) -> i32 {
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
