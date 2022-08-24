use clap::Parser;
use cli::Args;
use tracing::Level;
use tracing_subscriber::fmt;

mod app;
mod cli;
mod error;
mod service;

fn main() {
    let args = Args::parse();
    setup_logging(&args);
    tracing::debug!("Starting app");

    if let Err(ref err) = args.run() {
        error::report_exit(err, args.verbose);
    }

    tracing::debug!("Closing app");
    std::process::exit(exitcode::OK)
}

fn setup_logging(args: &Args) {
    match args.verbose {
        true => verbose_logging(),
        false => std_logging(),
    }
}

fn std_logging() {
    fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .without_time()
        .init();
}

fn verbose_logging() {
    fmt().pretty().with_max_level(Level::DEBUG).init();
}
