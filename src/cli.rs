use crate::app;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,

    /// Turn on logging output
    #[clap(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    New(app::new::Command),
    Init(app::init::Command),
}

impl Args {
    pub fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Command::New(com) => com.run(),
            Command::Init(com) => com.run(),
        }
    }
}
