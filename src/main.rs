mod daemon_util;
mod libc_util;
mod tasks;

fn main() {

    println!("pkg_name:{}", env!("CARGO_PKG_NAME"));
    println!("author_name:{}", env!("CARGO_PKG_AUTHORS").get(0).unwrap());

    let username = libc_util::get_current_user();
    daemon_util::daemonize(tasks::start_backstage_task, username);
}
