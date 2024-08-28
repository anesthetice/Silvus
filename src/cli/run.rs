use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("run")
}

pub(super) async fn process(arg_matches: &ArgMatches) -> eyre::Result<()> {
    Ok(())
}
