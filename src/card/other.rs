use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Other {
    title: String,
    description: Option<String>,
    thumbnail: Option<PathBuf>,
    content: Vec<(PathBuf, f64)>,
}
