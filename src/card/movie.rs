use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    title: String,
    description: Option<String>,
    thumbnail: Option<PathBuf>,
    filepath: PathBuf,
    filesize: f64,
}

impl CardMethods for Movie {
    fn from_paths(
        mut vid_fps: Vec<PathBuf>,
        dot_fps: Vec<PathBuf>,
        otr_fps: Vec<PathBuf>,
    ) -> eyre::Result<Card> {
        let vid_fp = vid_fps.pop().unwrap();
        Err(eyre::eyre!("wait"))
    }
}
