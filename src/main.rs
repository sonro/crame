use clap::Parser;
use cli::Args;
use tracing::Level;
use tracing_subscriber::fmt;

mod cli;

fn main() {
    let args = Args::parse();
    setup_logging(&args);
    tracing::info!("Starting app");
    println!("{:?}", args);
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
    fmt().with_max_level(Level::DEBUG).init();
}
