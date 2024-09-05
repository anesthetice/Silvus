use super::*;

static SRE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"[Ss]\d{1,2}"#).unwrap());
static ERE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"[Ee]\d{1,2}"#).unwrap());

#[derive(Debug)]
pub struct Show {
    title: String,
    subtitle: Option<String>,
    year: Option<u16>,
    description: Option<String>,
    // relative filepath
    thumbnail: Option<String>,
    // season, episode, relative filepath, size
    episodes: Vec<(u8, u8, String, FileSize)>,
}

impl CardMethods for Show {
    fn from_paths(
        base: &Path,
        path: &Path,
        vid_fps: Vec<PathBuf>,
        dot_fps: Vec<PathBuf>,
        _otr_fps: Vec<PathBuf>,
    ) -> eyre::Result<Card> {
        let mut title = String::new();
        let mut subtitle = None;
        let mut year = None;
        let mut description = None;
        let mut thumbnail = None;

        for dot_fp in dot_fps.into_iter() {
            match get_filestem(&dot_fp) {
                ".title" => {
                    if let Some(string) = lazy_read_file_to_string(&dot_fp) {
                        title = string;
                    };
                }
                ".subtitle" | ".subt" => subtitle = lazy_read_file_to_string(&dot_fp),
                ".year" => {
                    if let Some(y) = lazy_read_file_to_string(&dot_fp) {
                        if let Ok(y) = y.parse::<u16>() {
                            year.replace(y);
                        }
                    }
                }
                ".description" | ".descr" => description = lazy_read_file_to_string(&dot_fp),
                ".thumbnail" => thumbnail = get_rel_path_string(&dot_fp, base),
                _ => (),
            }
        }

        if title.is_empty() {
            let folder_name = get_filestem(path);
            let folder_name = folder_name.replace("-", " ");
            let folder_name = folder_name.replace("_", " ");
            title = folder_name;
        }

        let episodes = vid_fps
            .into_iter()
            .filter_map(|fp| {
                let filesize = FileSize::from(fp.metadata().ok()?.len());
                let rel_fp = get_rel_path_string(&fp, base)?;

                let fs = get_filestem(&fp);

                let Some(season) = SRE.find(fs) else {
                    warn!("Failed to match season in filename");
                    return None;
                };
                // unwrapping because regex rule assures this is valid
                let season = season.as_str()[1..].parse::<u8>().unwrap();

                let Some(episode) = ERE.find(fs) else {
                    warn!("Failed to match episode in filename");
                    return None;
                };
                // unwrapping because regex rule assures this is valid
                let episode = episode.as_str()[1..].parse::<u8>().unwrap();

                Some((season, episode, rel_fp, filesize))
            })
            .sorted_by_key(|(s, ..)| *s)
            .sorted_by_key(|(_, e, ..)| *e)
            .collect_vec();

        Ok(Card::Show(Self {
            title,
            subtitle,
            year,
            description,
            thumbnail,
            episodes,
        }))
    }

    fn into_html_string(self) -> String {
        indoc::formatdoc! {
            "<div class=\"card\">
                <div class=\"card-header\">
                    <div class=\"card-header-thumbnail\"><img src=\"/res/{}\" /></div>
                    <div class=\"card-header-box\">
                        <div class=\"card-header-box-title\"><h2>{}</h2></div>
                        <div class=\"card-header-box-subtitle\">
                            <p>{} • {}</p>
                        </div>
                    </div>
                </div>
                <div class=\"card-expand\">
                    <p>
                        {}
                    </p>
                    <p>
                        {}
                    </p>
                </div>
            </div>",

            display(self.thumbnail, "", "", ".assets/default_thumbnail.png"),
            self.title,
            display(self.year, "", "", "????"),
            display(self.subtitle, "", "", ""),
            display(self.description, "", "", "No description provided."),
            self.episodes.into_iter().map(|(s, e, fp, size)| {
                format!(
                    "season  {:0>2} • episode  {:0>2} • {}  MB • <a href=\"/res/{}\" download><img src=\"/res/.assets/download.svg\" /></a>",
                    s,
                    e,
                    size.0,
                    fp,
                )
            }).fold(String::new(), |acc, x| acc + &x + "\n")
        }
    }
}
