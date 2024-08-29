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

    let mut cards: Vec<Card> = Vec::new();

    Ok(())
}
