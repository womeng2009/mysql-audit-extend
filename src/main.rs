mod daemon_util;
mod libc_util;
mod tasks;

use std::fs;
use clap::Parser;
use anyhow::Result;

/// An extension tool of mysql-audit, which provides functions such as log rotation and log cleaning.
#[derive(Parser, Debug)]
#[clap(version, author = "Seeker <womeng209@qq.com>")]
struct Options {

    /// Absolute path to log file
    #[clap(short, long, parse(try_from_str = parse_path))]
    path: String,
}

/// 文件路径检查
fn parse_path(s: &str) -> Result<String> {
    fs::File::open(s)?;
    Ok(s.into())
}

fn main() -> Result<()> {
    let options = Options::parse();
    println!("Received parameters:{:?}", options);
    
    let username = libc_util::get_current_user()?;
    let pkg_name = env!("CARGO_PKG_NAME");
    let author_name = env!("CARGO_PKG_AUTHORS");
    let path: &str = options.path.as_str();
    daemon_util::daemonize(tasks::start_backstage_task(path), username.as_str(), pkg_name, author_name);

    Ok(())
}
