use std::process;

use colored::Colorize;

pub fn report_exit(err: &anyhow::Error) {
    report(err);
    process::exit(exitcode::SOFTWARE);
}

pub fn report(err: &anyhow::Error) {
    eprintln!("{}{} {}", "error".red().bold(), ":".white().bold(), err);
}
