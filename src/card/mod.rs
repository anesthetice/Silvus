// Modules
mod cards;
mod episode;
mod movie;
mod other;
mod series;

// Imports
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub enum Card {
    Movie {
        title: String,
        description: Option<String>,
        filepath: PathBuf,
        thumbnail: Option<PathBuf>,
        size: f64,
    },
    Series {
        title: String,
        description: String,
        size: f64,
        episodes: Vec<(u8, PathBuf, f64)>,
    },
    Other {
        title: String,
        size: f64,
        path: PathBuf,
    },
}
