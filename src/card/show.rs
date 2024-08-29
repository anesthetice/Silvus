use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
    title: String,
    description: String,
    thumbnail: Option<PathBuf>,
    // season, episode, filepath, size
    episodes: Vec<(u8, u8, PathBuf, f64)>,
}
