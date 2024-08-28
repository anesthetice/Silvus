// Imports
use std::path::Path;
use time::{OffsetDateTime, UtcOffset};
use tracing::{error, info, warn};

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
pub(super) async fn pt_validate_or_create_dirs(dirs: &[&Path]) -> eyre::Result<()> {
    let mut error_flag: bool = false;
    for dir in dirs.iter() {
        if !dir.exists() {
            eprintln!("Failed to find directory with path '{}'", dir.display());
            match tokio::fs::create_dir_all(dir).await {
                Ok(()) => {
                    eprintln!(
                        "Successfully created directory with path '{}'",
                        dir.display()
                    );
                }
                Err(err) => {
                    eprintln!(
                        "Failed to create directory with path '{}', '{err}'",
                        dir.display()
                    );
                    error_flag = true;
                }
            }
        }
    }
    if error_flag {
        Err(eyre::eyre!(
            "Failed to validate or create all specified directories"
        ))
    } else {
        Ok(())
    }
}

// used by other parts of the application, tracing enabled
pub async fn validate_or_create_dirs(dirs: &[&Path]) -> eyre::Result<()> {
    let mut error_flag: bool = false;
    for dir in dirs.iter() {
        if !dir.exists() {
            warn!("Failed to find directory with path '{}'", dir.display());
            match tokio::fs::create_dir_all(dir).await {
                Ok(()) => {
                    info!(
                        "Successfully created directory with path '{}'",
                        dir.display()
                    );
                }
                Err(err) => {
                    error!(
                        "Failed to create directory with path '{}', '{err}'",
                        dir.display()
                    );
                    error_flag = true;
                }
            }
        }
    }
    if error_flag {
        Err(eyre::eyre!(
            "Failed to validate or create all specified directories"
        ))
    } else {
        Ok(())
    }
}
