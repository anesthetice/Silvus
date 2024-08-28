use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("init").arg(
        Arg::new("path")
            .value_parser(clap::value_parser!(PathBuf))
            .required(true)
            .index(1),
    )
}

pub(super) async fn process(arg_matches: &ArgMatches) -> eyre::Result<()> {
    if let Some(path) = &crate::config::get().target_dir {
        warn!(
            "A target directory already exists with path '{}'\n{:>18} Please note that this directory will not be affected in any way",
            path.display(),
            ""
        )
    }
    let path = arg_matches
        .get_one::<PathBuf>("path")
        .ok_or_eyre("Failed to get path")?;
    let path = std::path::absolute(path).wrap_err("Failed to absolutize path")?;
    crate::utils::validate_or_create_dirs(&[&path]).await?;

    let mut owned_config = crate::config::get().clone();
    owned_config.target_dir = Some(path);
    owned_config.save_to_file().await?;
    Ok(())
}
