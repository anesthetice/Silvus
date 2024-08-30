// Imports
use eyre::OptionExt;
use std::{io::Read, path::Path};
use time::{OffsetDateTime, UtcOffset};
use tracing::{info, warn};

pub fn get_local_datetime() -> OffsetDateTime {
    let local_offset = UtcOffset::current_local_offset().unwrap_or_else(|e| {
        eprintln!("Failed to get local offset, '{}'", e);
        UtcOffset::UTC
    });
    OffsetDateTime::now_utc().to_offset(local_offset)
}

pub fn datetime_to_pretty_string(dt: &OffsetDateTime) -> String {
    format!(
        "{:0>2}/{:0>2}/{:0>4} - {:0>2}:{:0>2}:{:0>2}",
        dt.day(),
        dt.month() as u8,
        dt.year(),
        dt.hour(),
        dt.minute(),
        dt.second(),
    )
}

pub fn datetime_to_path_string(dt: &OffsetDateTime) -> String {
    format!(
        "{:0>4}_{:0>2}_{:0>2}-{:0>2}_{:0>2}_{:0>2}",
        dt.year(),
        dt.month() as u8,
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second(),
    )
}

// used by main before tracing is available
pub fn check_or_create_all_nt(dir: &Path) -> eyre::Result<()> {
    if !dir.exists() {
        eprintln!("Failed to find directory with path '{}'", dir.display());
        match std::fs::create_dir_all(dir) {
            Ok(()) => {
                eprintln!(
                    "Successfully created directory with path '{}'",
                    dir.display()
                );
                Ok(())
            }
            Err(err) => {
                eprintln!("Failed to create directory with path '{}'", dir.display());
                Err(err)?
            }
        }
    } else {
        Ok(())
    }
}

pub fn check_or_create_all(dir: &Path) -> eyre::Result<()> {
    if !dir.exists() {
        warn!("Failed to find directory with path '{}'", dir.display());
        match std::fs::create_dir_all(dir) {
            Ok(()) => {
                info!(
                    "Successfully created directory with path '{}'",
                    dir.display()
                );
                Ok(())
            }
            Err(err) => {
                warn!("Failed to create directory with path '{}'", dir.display());
                Err(err)?
            }
        }
    } else {
        Ok(())
    }
}

pub fn get_extension(fp: &Path) -> &str {
    let Some(ext) = fp.extension() else {
        return "";
    };
    ext.to_str().unwrap_or("")
}

pub fn get_filename(fp: &Path) -> &str {
    let Some(name) = fp.file_name() else {
        return "";
    };
    name.to_str().unwrap_or("")
}

pub fn get_filestem(fp: &Path) -> &str {
    let Some(stem) = fp.file_stem() else {
        return "";
    };
    stem.to_str().unwrap_or("")
}

pub fn lazy_read_file_to_string(fp: &Path) -> Option<String> {
    if !fp.is_file() {
        return None;
    }
    let mut string = String::new();
    let mut read_op = || {
        std::fs::OpenOptions::new()
            .read(true)
            .open(fp)?
            .read_to_string(&mut string)?;
        Ok::<(), eyre::Error>(())
    };
    match read_op() {
        Ok(()) => Some(string.trim().to_string()),
        Err(err) => {
            warn!(
                "Failed to read file with path '{}', '{}'",
                fp.display(),
                err
            );
            None
        }
    }
}

pub fn get_rel_path_string(path: &Path, base: &Path) -> Option<String> {
    let operation = || {
        Ok::<String, eyre::Error>(
            path.strip_prefix(base)?
                .to_str()
                .ok_or_eyre("Filepath not encoded with valid-utf8")?
                .replace("\\", "/"),
        )
    };
    match operation() {
        Ok(string) => Some(string),
        Err(err) => {
            warn!("{err}");
            None
        }
    }
}
