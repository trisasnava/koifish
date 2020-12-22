use std::io;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Logger {
    pub level: String,
    pub log_path: PathBuf,
}

impl Logger {
    pub fn level(level: &str) -> Result<Logger, fern::InitError> {
        let mut base_config = fern::Dispatch::new();
        let log_path = dirs::cache_dir().unwrap().join("koi.log");

        base_config = match level {
            "trace" => base_config
                .level(log::LevelFilter::Trace)
                .level_for("overly-verbose-target", log::LevelFilter::Trace),
            "debug" => base_config
                .level(log::LevelFilter::Debug)
                .level_for("overly-verbose-target", log::LevelFilter::Debug),
            "info" => base_config
                .level(log::LevelFilter::Info)
                .level_for("overly-verbose-target", log::LevelFilter::Info),
            "warn" => base_config
                .level(log::LevelFilter::Warn)
                .level_for("overly-verbose-target", log::LevelFilter::Warn),
            "error" => base_config
                .level(log::LevelFilter::Error)
                .level_for("overly-verbose-target", log::LevelFilter::Error),
            _ => base_config
                .level(log::LevelFilter::Error)
                .level_for("overly-verbose-target", log::LevelFilter::Error),
        };

        // Separate file config so we can include year, month and day in file logs
        let file_config = fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .chain(fern::log_file(&log_path.as_path())?);

        let stdout_config = fern::Dispatch::new()
            .format(|out, message, record| {
                // special format for debug messages coming from our own crate.
                if record.level() > log::LevelFilter::Info && record.target() == "cmd_program" {
                    out.finish(format_args!(
                        "---\nDEBUG: {}: {}\n---",
                        chrono::Local::now().format("%H:%M:%S"),
                        message
                    ))
                } else {
                    out.finish(format_args!(
                        "[{}][{}][{}] {}",
                        chrono::Local::now().format("%H:%M"),
                        record.target(),
                        record.level(),
                        message
                    ))
                }
            })
            .chain(io::stdout());

        base_config
            .chain(file_config)
            .chain(stdout_config)
            .apply()?;

        Ok(Self {
            level: level.to_string(),
            log_path,
        })
    }
}
