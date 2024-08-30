use super::*;

#[derive(Debug)]
pub struct Movie {
    title: String,
    year: Option<u16>,
    description: Option<String>,
    // relative path, http-compatible
    thumbnail: Option<String>,
    // relative path, http-compatible
    filepath: String,
    filesize: FileSize,
}

// this is sane way of doing things, trust me bro
macro_rules! E {
    ($e:expr, $i:literal) => {
        if let Some(ref a) = $e {
            a
        } else {
            $i
        }
    };
}

// this is sane way of doing things, trust me bro
macro_rules! F {
    ($e:expr, $p:literal, $i:literal) => {
        if let Some(ref a) = $e {
            a.to_string() + $p
        } else {
            $i.to_string()
        }
    };
}

impl CardMethods for Movie {
    fn from_paths(
        base: &Path,
        path: &Path,
        mut vid_fps: Vec<PathBuf>,
        dot_fps: Vec<PathBuf>,
        _otr_fps: Vec<PathBuf>,
    ) -> eyre::Result<Card> {
        let mut title = String::new();
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

        let fp = vid_fps.pop().unwrap();
        let filesize = FileSize::from(fp.metadata()?.len());
        let filepath = get_rel_path_string(&fp, base).ok_or_eyre("Video filepath is crucial")?;

        Ok(Card::Movie(Self {
            title,
            year,
            description,
            thumbnail,
            filepath,
            filesize,
        }))
    }

    fn into_html_string(self) -> String {
        indoc::formatdoc! {
            "<div class=\"card\">
                <div class=\"card-header\">
                    <div class=\"card-header-thumbnail\"><img src=\"{}\" /></div>
                    <div class=\"card-header-box\">
                        <div class=\"card-header-box-title\"><h2>{}</h2></div>
                        <div class=\"card-header-box-subtitle\">
                            <p>
                                {}{:?} MB • <a href=\"{}\" download><img src=\".assets/download.svg\" /></a>
                            </p>
                        </div>
                    </div>
                </div>
                <div class=\"card-expand\">
                    <p>{}</p>
                </div>",

            E!(self.thumbnail, "./assets/thumbnail.jpg"),
            self.title,
            F!(self.year, " • ", ""),
            self.filesize.0,
            self.filepath,
            E!(self.description, "No description provided"),
        }
    }
}
