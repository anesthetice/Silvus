// Modules
mod cards;
mod movie;
mod other;
mod show;

// Imports
use crate::utils::{get_extension, get_filename};
use eyre::eyre;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::{instrument, warn};

static VIDEO_FILE_EXTENSIONS: [&str; 10] = [
    "webm", "mkv", "vob", "ogg", "ogv", "avi", "move", "qt", "m4v", "m4v",
];

#[derive(Debug, Serialize, Deserialize)]
pub enum Card {
    Movie(movie::Movie),
    Show(show::Show),
    Other(other::Other),
}

impl Card {
    #[instrument]
    pub fn from_path(path: &Path) -> eyre::Result<Self> {
        let mut vid_fps: Vec<PathBuf> = Vec::new();
        let mut dot_fps: Vec<PathBuf> = Vec::new();
        let mut otr_fps: Vec<PathBuf> = Vec::new();

        for fp in std::fs::read_dir(path)? {
            let fp = match fp {
                Ok(fp) => fp.path(),
                Err(err) => {
                    warn!("{err}");
                    continue;
                }
            };

            if !fp.is_file() {
                warn!("Found sub-directory, will be ignored");
                continue;
            }

            if VIDEO_FILE_EXTENSIONS.contains(&get_extension(&fp)) {
                vid_fps.push(fp);
            } else if get_filename(&fp).starts_with('.') {
                dot_fps.push(fp)
            } else {
                otr_fps.push(fp);
            }
        }

        /*
        match video_filepaths.len() {
            // Other route
            0 => unreachable!(),
            // Movie route
            1 => unreachable!(),
            // Series route
            2.. => unreachable!(),
        }
        */
        Err(eyre!("misc"))
    }
}

trait CardMethods {
    fn from_paths(
        vid_fps: Vec<PathBuf>,
        dot_fps: Vec<PathBuf>,
        otr_fps: Vec<PathBuf>,
    ) -> eyre::Result<Card>;
}
