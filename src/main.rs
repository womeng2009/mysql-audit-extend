mod daemon_util;
mod libc_util;
mod tasks;

use clap::Parser;

/// An extension tool of mysql-audit, which provides functions such as log rotation and log cleaning.
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Seeker <womeng209@qq.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(author, version, about, long_about = None)]
struct Options {

    /// Absolute path to log file
    #[clap(short, long, value_parser)]
    path: String,
}

fn main() {
    let options = Options::parse();
    println!("Received parameters:{:?}", options);
    
    // let username = libc_util::get_current_user();
    // let pkg_name = env!("CARGO_PKG_NAME");
    // let author_name = env!("CARGO_PKG_AUTHORS");
    // daemon_util::daemonize(tasks::start_backstage_task, username.as_str(), pkg_name, author_name);
}
