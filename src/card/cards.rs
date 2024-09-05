use super::*;
use axum::response::{Html, IntoResponse};
use indoc::formatdoc;

pub struct Cards(Vec<Card>);

static STYLE: &str = include_str!("../../assets/style.css");
static SCRIPT: &str = include_str!("../../assets/script.js");

impl Cards {
    pub fn load() -> eyre::Result<Self> {
        let path = crate::config::get()
            .target_dir
            .to_owned()
            .ok_or_eyre("No target path set, use the init subcommand")?;

        let cards: Vec<Card> = std::fs::read_dir(&path)?
            .filter_map(|dir| {
                let dir = match dir {
                    Ok(dir) => dir.path(),
                    Err(err) => {
                        warn!("{err}");
                        return None;
                    }
                };
                if dir.is_dir() {
                    match Card::from_path(&dir) {
                        Ok(card) => Some(card),
                        Err(err) => {
                            warn!("{err}");
                            None
                        }
                    }
                } else {
                    None
                }
            })
            .collect();

        Ok(Self(cards))
    }

    pub fn generate_static_html_page(self) -> String {
        formatdoc! {
            "<!doctype html>
            <html lang=\"en\">
                <head>
                    <meta charset=\"UTF-8\" />
                    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
                    <meta name=\"description\" content=\"server\" />
                    <meta name=\"author\" content=\"anesthetice\" />
                    <title>Silvus</title>
                    <style>
                        {}
                    </style>
                </head>
                <body>
                    <div class=\"page-header\">
                        <img src=\"/res/.assets/icon.svg\" />
                    </div>
                    <div class=\"card-row\">
                        <div class=\"card-column\">
                            {}
                        </div>
                    </div>
                    <script>
                        {}
                    </script>
                </body>
            </html>
            ",
            STYLE,
            self.0.into_iter().map(|card| card.into_html_string()).fold(String::new(), |acc, x| acc + &x +"\n"),
            SCRIPT,
        }
    }
}
