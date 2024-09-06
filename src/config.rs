// Imports
use eyre::Context;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::PathBuf,
    sync::OnceLock,
};
use tracing::{debug, info, warn};

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init() -> eyre::Result<()> {
    CONFIG
        .set(Config::load())
        .map_err(|_| eyre::eyre!("Failed to set CONFIG"))
}

pub fn get() -> &'static Config {
    CONFIG.get().unwrap()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub target_dir: Option<PathBuf>,
    pub port: String,
    pub imdb_url: String,
    pub imdb_description_start_match: String,
    pub imdb_description_end_match: String,
    pub imdb_year_start_match: String,
    pub imdb_year_end_match: String,
    pub imdb_image_redirect_start_match: String,
    pub imdb_image_redirect_end_match: String,
    pub imdb_image_start_match: String,
    pub imdb_image_end_match: String,
    pub user_agent: String,
    pub connection_timeout: std::time::Duration,
}

#[allow(clippy::derivable_impls)]
impl Default for Config {
    fn default() -> Self {
        Self {
            target_dir: None,
            port: "1888".to_string(),
            imdb_url: r#"https://www.imdb.com/"#.to_string(),
            imdb_description_start_match: r#"class="sc-2d37a7c7-2 ggeRnl">"#.to_string(),
            imdb_description_end_match: r#"</span></p>"#.to_string(),
            imdb_year_start_match: r#"releaseinfo?ref_=tt_ov_rdat">"#.to_string(),
            imdb_year_end_match: r#"</a>"#.to_string(),
            imdb_image_redirect_start_match: r#"class="ipc-lockup-overlay ipc-focusable" href=""#
                .to_string(),
            imdb_image_redirect_end_match: r#"""#.to_string(),
            imdb_image_start_match: r#"https://m.media-amazon.com/images/"#.to_string(),
            imdb_image_end_match: r#"""#.to_string(),
            user_agent: "Mozilla/5.0 (X11; Linux x86_64; rv:129.0) Gecko/20100101 Firefox/129.0"
                .to_string(),
            connection_timeout: std::time::Duration::from_secs(5),
        }
    }
}

impl Config {
    const FILENAME: &'static str = "silvus.conf";

    pub fn load() -> Self {
        debug!("Attempting to load config...");
        match Self::load_from_file() {
            Ok(config) => {
                debug!("Config successfully loaded from file");
                config
            }
            Err(err) => {
                warn!("Failed to load config from file, '{err}'");
                let config = Self::default();
                if let Ok(io_error) = err.downcast::<tokio::io::Error>() {
                    if io_error.kind() == tokio::io::ErrorKind::NotFound {
                        info!(
                            "Generating default config at path '{}'",
                            crate::dirs::get().config_dir().display()
                        );
                        if let Err(err) = config.save_to_file() {
                            warn!("Failed to save generated config, '{}'", err);
                        }
                    }
                }
                debug!("Config generated");
                config
            }
        }
    }

    fn load_from_file() -> eyre::Result<Self> {
        let filepath = crate::dirs::get().config_dir().join(Self::FILENAME);
        let mut bytes: Vec<u8> = Vec::new();

        let mut read_file = std::fs::OpenOptions::new()
            .read(true)
            .open(&filepath)
            .wrap_err_with(|| {
                format!(
                    "Failed to read/open config file with path '{}'",
                    filepath.display()
                )
            })?;
        read_file.read_to_end(&mut bytes)?;

        Ok(ijson::from_value(&serde_json::from_slice(&bytes)?)?)
    }

    pub fn save_to_file(&self) -> eyre::Result<()> {
        debug!("Attempting to save config...");
        let filepath = crate::dirs::get().config_dir().join(Self::FILENAME);
        let bytes = serde_json::to_vec_pretty(&ijson::to_value(self)?)?;

        let mut write_file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&filepath)
            .wrap_err_with(|| {
                format!(
                    "Failed to write/truncate/create/open config file with path '{}'",
                    filepath.display()
                )
            })?;
        write_file.write_all(&bytes)?;
        write_file.sync_all()?;

        debug!("Saved config to '{}'", filepath.display());

        Ok(())
    }
}
