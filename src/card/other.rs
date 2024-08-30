use super::*;

#[derive(Debug)]
pub struct Other {
    title: String,
    description: Option<String>,
    thumbnail: Option<String>,
    content: Vec<(String, FileSize)>,
}
