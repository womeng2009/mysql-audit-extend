mod daemon_util;
mod libc_util;
mod tasks;

fn main() {
    let username = libc_util::get_current_user();
    let pkg_name = env!("CARGO_PKG_NAME");
    let author_name = env!("CARGO_PKG_AUTHORS");
    daemon_util::daemonize(tasks::start_backstage_task, username.as_str(), pkg_name, author_name);
}
