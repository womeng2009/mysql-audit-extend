#[cfg(target_family = "unix")]
mod daemon_util;
#[cfg(target_family = "unix")]
mod libc_util;
mod tasks;

use crate::tasks::start_backstage_task;
use anyhow::Result;
use clap::Parser;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::fs;

/// An extension tool of mysql-audit, which provides functions such as log rotation and log cleaning.
#[derive(Parser, Debug)]
#[clap(version, author = "Seeker <womeng209@qq.com>")]
struct Options {
    /// Absolute path to log file
    #[clap(short, long, parse(try_from_str = parse_path), default_value_t = String::from("/var/lib/mysql/mysql-audit.json"))]
    path: String,

    /// Maximum file size, Unit: MB.
    #[clap(short = 'm', long, value_parser, default_value_t = 10)]
    max_size: u32,

    /// Maximum number of files to keep
    #[clap(short = 'f', long, value_parser, default_value_t = 10)]
    max_file: u32,
}

/// File path check
fn parse_path(s: &str) -> Result<String> {
    fs::File::open(s)?;
    Ok(s.into())
}

/// Initialize the log component
fn init_log() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_local_timestamps()
        .init()
        .unwrap();
}

/// main function
fn main() -> Result<()> {
    let options = Options::parse();
    log::info!("App environments:{:?}", options);

    init_log();

    let path: String = options.path;
    let max_size: u32 = options.max_size;
    let max_file: u32 = options.max_file;

    if cfg!(target_family = "unix") {
        #[cfg(target_family = "unix")]
        {
            let username = libc_util::get_current_user();
            let pkg_name = env!("CARGO_PKG_NAME");
            let author_name = env!("CARGO_PKG_AUTHORS");
            daemon_util::daemonize(
                username.as_str(),
                pkg_name,
                author_name,
                path,
                max_size,
                max_file,
            );
        }
    } else {
        start_backstage_task(path, max_size, max_file);
    }

    Ok(())
}
