use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildSystem {
    Crame,
    Just,
    Make,
}

impl Default for BuildSystem {
    fn default() -> Self {
        BuildSystem::Just
    }
}
