// Modules
mod init;
mod run;

// Imports
use clap::{Arg, ArgMatches, Command};
use eyre::{Context, OptionExt};
use std::path::PathBuf;
use tracing::warn;

pub fn cli() -> eyre::Result<()> {
    let command = clap::command!().subcommands([init::subcommand(), run::subcommand()]);

    let arg_matches = command.get_matches();

    match arg_matches.subcommand() {
        Some(("init", arg_matches)) => init::process(arg_matches),
        Some(("run", arg_matches)) => run::process(arg_matches),
        _ => Ok(()),
    }
}
