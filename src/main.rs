mod daemon_util;
mod libc_util;
mod tasks;

fn main() {
    env!("CARGO_PKG_AUTHORS").
    println!("pkg_name:{}", env!("CARGO_PKG_NAME"));
    println!("author_name:{}", env!("CARGO_PKG_AUTHORS"));

    let username = libc_util::get_current_user();
    daemon_util::daemonize(tasks::start_backstage_task, username);
}
