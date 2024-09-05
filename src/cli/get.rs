use std::io::Write;

use super::*;
use reqwest::Client;

pub(super) fn subcommand() -> Command {
    Command::new("get")
        .arg(
            Arg::new("path")
                .index(1)
                .required(true)
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("link")
                .index(2)
                .required(true)
                .action(ArgAction::Set),
        )
}

pub(super) fn process(arg_matches: &ArgMatches) -> eyre::Result<()> {
    let path = arg_matches
        .get_one::<PathBuf>("path")
        .ok_or_eyre("Failed to get path")?;

    let path = std::fs::canonicalize(path)?;
    if !path.is_dir() {
        return Err(eyre::eyre!("Path is not an existing directory"));
    }
    if path
        .parent()
        .ok_or_eyre("Invalid path, no parent directory found")?
        != gcfg()
            .target_dir
            .as_ref()
            .ok_or_eyre("No target directory set")?
    {
        return Err(eyre::eyre!(
            "Invalid path, parent directory does not match target directory"
        ));
    }

    let link = arg_matches
        .get_one::<String>("link")
        .ok_or_eyre("Failed to get link")?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            let client = Client::builder()
                .connection_verbose(true)
                .connect_timeout(gcfg().connection_timeout)
                .user_agent(&gcfg().user_agent)
                .build()?;

            let response = client.get(link).send().await?;
            let text = response.text().await?;

            let mut sidx = text
                .find(&gcfg().imdb_description_start_match)
                .ok_or_eyre("Failed to find a match for the start of the description")?;
            sidx += gcfg().imdb_description_start_match.len();

            let offset = text[sidx..]
                .find(&gcfg().imdb_description_end_match)
                .ok_or_eyre("Failed to find a match for the end of the description")?;

            let description = &text[sidx..sidx + offset];

            std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(path.join(".description"))?
                .write_all(description.as_bytes())?;

            let mut sidx = text
                .find(&gcfg().imdb_image_redirect_start_match)
                .ok_or_eyre("Failed to find a match for the start of the image redirect")?;
            sidx += gcfg().imdb_image_redirect_start_match.len();

            let offset = text[sidx..]
                .find(&gcfg().imdb_image_redirect_end_match)
                .ok_or_eyre("Failed to find a match for the end of the image redirect")?;

            let image_redirect = gcfg().imdb_url.clone() + &text[sidx..sidx + offset];

            let response = client.get(&image_redirect).send().await?;
            let text = response.text().await?;

            let sidx = text
                .find(&gcfg().imdb_image_start_match)
                .ok_or_eyre("Failed to find a match for the start of the image")?;

            let offset = text[sidx..]
                .find(&gcfg().imdb_image_end_match)
                .ok_or_eyre("Failed to find a match for the end of the image")?;

            let image = &text[sidx..sidx + offset];

            println!(
                "description = '{}'\nimage redirect = '{}'\nimage = '{}'",
                description, image_redirect, image
            );

            let response = client.get(image).send().await?;
            let image = response.bytes().await?;

            std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(path.join(".thumbnail"))?
                .write_all(&image)?;

            Ok::<(), eyre::Error>(())
        })?;

    Ok(())
}
