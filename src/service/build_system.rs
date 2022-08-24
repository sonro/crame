use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum BuildSystem {
    Crame,
    Just,
    Make,
}
