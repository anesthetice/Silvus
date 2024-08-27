// Modules
mod card;
mod cli;
mod config;
mod dirs;
mod utils;

// Imports
use eyre::Context;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt::{self, time::Uptime},
    layer::SubscriberExt,
    EnvFilter, Layer, Registry,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // ## Initialization
    // 1. init dirs, get local time
    dirs::init();
    let dt = utils::get_local_datetime();

    // 2. create log file
    let log_filepath = {
        let log_directory = dirs::get().data_local_dir().join("logs");
        if !log_directory.exists() {
            std::fs::create_dir_all(&log_directory).wrap_err_with(|| {
                format!(
                    "Failed to create directories to store logs, '{}'",
                    &log_directory.display()
                )
            })?;
        }
        log_directory.join(utils::datetime_to_path_string(&dt) + ".log")
    };
    let log_file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&log_filepath)
        .wrap_err_with(|| {
            format!(
                "Failed to create log file with path '{}'",
                log_filepath.display()
            )
        })?;

    // 3. setup tracing
    let (logfileout, _guard) = tracing_appender::non_blocking(log_file);

    let format_logfile = fmt::format()
        .compact()
        .with_ansi(false)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    let filter_logfile = LevelFilter::DEBUG;

    let format_console = fmt::format()
        .pretty()
        .with_ansi(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    let filter_console = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("INFO"));

    let subscriber = Registry::default()
        .with(
            fmt::layer()
                .event_format(format_console)
                .with_timer(Uptime::default())
                .with_writer(std::io::stderr)
                .with_filter(filter_console),
        )
        .with(
            fmt::Layer::new()
                .event_format(format_logfile)
                .with_timer(Uptime::default())
                .with_writer(logfileout)
                .with_filter(filter_logfile),
        );

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");
    info!("{}", utils::datetime_to_pretty_string(&dt));

    // init config

    Ok(())
}
