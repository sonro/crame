use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum VersionControl {
    Git,
    None,
}
