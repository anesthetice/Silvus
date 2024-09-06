use super::*;

#[derive(Debug)]
pub struct Other {
    pub title: String,
    description: Option<String>,
    thumbnail: Option<String>,
    content: Vec<(String, FileSize)>,
}
