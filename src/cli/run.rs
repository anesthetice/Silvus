use crate::card::Card;

use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("run")
}

pub(super) fn process(arg_matches: &ArgMatches) -> eyre::Result<()> {
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

    println!("{:#?}", cards);

    for card in cards {
        println!("{}\n\n", card.into_html_string())
    }
    Ok(())
}
