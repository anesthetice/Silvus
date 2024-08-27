use directories::ProjectDirs;
use eyre::OptionExt;
use std::sync::OnceLock;

static DIRS: OnceLock<ProjectDirs> = OnceLock::new();

pub fn init() -> eyre::Result<()> {
    DIRS.set(
        ProjectDirs::from("", "", "silvus").ok_or_eyre("Failed to generate project directories")?,
    )
    .map_err(|_| eyre::eyre!("Failed to set DIRS"))
}

pub fn get() -> &'static ProjectDirs {
    DIRS.get().unwrap()
}
