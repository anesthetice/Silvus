use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get() -> &'static Config {
    CONFIG.get().unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}
