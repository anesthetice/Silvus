// Modules
pub mod cards;
mod movie;
mod other;
mod show;

// Imports
use crate::utils::{
    get_extension, get_filename, get_filestem, get_rel_path_string, lazy_read_file_to_string,
};
use eyre::{eyre, OptionExt};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use tracing::{instrument, warn};

static VIDEO_FILE_EXTENSIONS: [&str; 11] = [
    "webm", "mkv", "vob", "ogg", "ogv", "avi", "move", "qt", "m4v", "m4v", "mp4",
];

#[derive(Debug)]
pub enum Card {
    Movie(movie::Movie),
    Show(show::Show),
    Other(other::Other),
}

impl Card {
    #[instrument]
    pub fn from_path(path: &Path) -> eyre::Result<Self> {
        let base = path
            .parent()
            .ok_or_eyre("Specified path doesn't have a parent")?;

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

        match vid_fps.len() {
            // Other route
            0 => Err(eyre!("'Other' route not ready")),
            // Movie route
            1 => movie::Movie::from_paths(base, path, vid_fps, dot_fps, otr_fps),
            // Show route
            2.. => show::Show::from_paths(base, path, vid_fps, dot_fps, otr_fps),
        }
    }

    pub fn into_html_string(self) -> String {
        match self {
            Self::Movie(movie) => movie.into_html_string(),
            Self::Show(show) => show.into_html_string(),
            Self::Other(other) => String::from("OTHER NOT IMPLEMENTED"),
        }
    }
}

trait CardMethods {
    fn from_paths(
        base: &Path,
        path: &Path,
        vid_fps: Vec<PathBuf>,
        dot_fps: Vec<PathBuf>,
        otr_fps: Vec<PathBuf>,
    ) -> eyre::Result<Card>;

    fn into_html_string(self) -> String;
}

// Size of a file represented by MB
#[derive(Debug)]
pub struct FileSize(u32);

impl From<u64> for FileSize {
    fn from(value: u64) -> Self {
        Self(u32::try_from(value / 1_000_000).unwrap_or(u32::MAX))
    }
}

fn display<T: Display>(input: Option<T>, pre: &str, post: &str, alt: &str) -> String {
    match input {
        Some(val) => format!("{pre}{}{post}", val),
        None => alt.to_owned(),
    }
}
