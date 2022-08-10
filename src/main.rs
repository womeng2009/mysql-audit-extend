mod daemon_util;
mod libc_util;
mod tasks;

/// 程序控制参数
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Options {
    /// 日志文件绝对路径
    #[clap(short, long, value_parser)]
    path: String,
}

fn main() {
    let options = Options::parse();
    println!("接收到参数:{:?}", options);
    
    
    // let username = libc_util::get_current_user();
    // let pkg_name = env!("CARGO_PKG_NAME");
    // let author_name = env!("CARGO_PKG_AUTHORS");
    // daemon_util::daemonize(tasks::start_backstage_task, username.as_str(), pkg_name, author_name);
}
