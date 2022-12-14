use clap::Parser;
use tracing::Level;
use tracing_subscriber::fmt;

use app::Args;

mod app;
mod service;
mod util;

fn main() {
    let args = Args::parse();
    setup_logging(&args);
    tracing::debug!("Starting app");

    if let Err(ref err) = args.run() {
        util::error::report_exit(err, args.verbose);
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
