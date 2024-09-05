use std::{io::Write, path::Path};

use super::*;

static DOWNLOAD_IMAGE: &[u8] = include_bytes!("../../assets/download.svg");
static DEFAULT_THUMBNAIL_IMAGE: &[u8] = include_bytes!("../../assets/default_thumbnail.png");
static ICON_IMAGE: &[u8] = include_bytes!("../../assets/icon.svg");

pub(super) fn subcommand() -> Command {
    Command::new("init").arg(
        Arg::new("path")
            .value_parser(clap::value_parser!(PathBuf))
            .required(true)
            .index(1)
            .action(ArgAction::Set),
    )
}

pub(super) fn process(arg_matches: &ArgMatches) -> eyre::Result<()> {
    if let Some(path) = &crate::config::get().target_dir {
        warn!(
            "A target directory already exists with path '{}'\n{:>18} Please note that this directory will not be affected in any way",
            path.display(),
            ""
        )
    }
    let path = arg_matches
        .get_one::<PathBuf>("path")
        .ok_or_eyre("Failed to get path")?
        .canonicalize()
        .wrap_err("Failed to canonicalize path")?;

    let asset_path = path.join(".assets/");
    crate::utils::check_or_create_all(&asset_path)?;
    save_to_file(&asset_path, "download.svg", DOWNLOAD_IMAGE)?;
    save_to_file(
        &asset_path,
        "default_thumbnail.png",
        DEFAULT_THUMBNAIL_IMAGE,
    )?;
    save_to_file(&asset_path, "icon.svg", ICON_IMAGE)?;

    let mut owned_config = crate::config::get().clone();
    owned_config.target_dir = Some(path);
    owned_config.save_to_file()?;
    Ok(())
}

fn save_to_file(path: &Path, filename: &str, data: &[u8]) -> eyre::Result<()> {
    let fp = path.join(filename);
    Ok(std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&fp)?
        .write_all(data)?)
}
