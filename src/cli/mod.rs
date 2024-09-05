// Modules
mod get;
mod init;
mod run;

// Imports
use crate::config::get as gcfg;
use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::{Context, OptionExt};
use std::path::PathBuf;
use tracing::warn;

pub fn cli() -> eyre::Result<()> {
    let command =
        clap::command!().subcommands([init::subcommand(), run::subcommand(), get::subcommand()]);

    let arg_matches = command.get_matches();

    match arg_matches.subcommand() {
        Some(("init", arg_matches)) => init::process(arg_matches),
        Some(("run", arg_matches)) => run::process(arg_matches),
        Some(("get", arg_matches)) => get::process(arg_matches),
        _ => Ok(()),
    }
}
