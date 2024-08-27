use std::sync::OnceLock;

use eyre::Context;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{info, warn};

static CONFIG: OnceLock<Config> = OnceLock::new();

pub async fn init() -> eyre::Result<()> {
    CONFIG
        .set(Config::load().await)
        .map_err(|_| eyre::eyre!("Failed to set CONFIG"))
}

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

impl Config {
    const FILENAME: &'static str = "silvus.conf";

    async fn load() -> Self {
        match Self::load_from_file().await {
            Ok(config) => config,
            Err(err) => {
                warn!("Failed to load config from file, '{err}'");
                let config = Self::default();
                if let Ok(io_error) = err.downcast::<tokio::io::Error>() {
                    if io_error.kind() == tokio::io::ErrorKind::NotFound {
                        info!(
                            "Generating default config at path '{}'",
                            crate::dirs::get().config_dir().display()
                        );
                        if let Err(err) = config.save_to_file().await {
                            warn!("Failed to save generated config, '{}'", err);
                        }
                    }
                }
                config
            }
        }
    }

    async fn load_from_file() -> eyre::Result<Self> {
        let filepath = crate::dirs::get().config_dir().join(Self::FILENAME);
        let mut bytes: Vec<u8> = Vec::new();

        let mut read_file = tokio::fs::OpenOptions::new()
            .read(true)
            .open(&filepath)
            .await
            .wrap_err_with(|| {
                format!(
                    "Failed to read/open config file with path '{}'",
                    filepath.display()
                )
            })?;
        read_file.read_to_end(&mut bytes).await?;

        Ok(ijson::from_value(&serde_json::from_slice(&bytes)?)?)
    }

    async fn save_to_file(&self) -> eyre::Result<()> {
        let filepath = crate::dirs::get().config_dir().join(Self::FILENAME);
        let bytes = serde_json::to_vec_pretty(&ijson::to_value(self)?)?;

        let mut write_file = tokio::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&filepath)
            .await
            .wrap_err_with(|| {
                format!(
                    "Failed to write/truncate/create/open config file with path '{}'",
                    filepath.display()
                )
            })?;
        write_file.write_all(&bytes).await?;
        write_file.sync_all().await?;

        Ok(())
    }
}
