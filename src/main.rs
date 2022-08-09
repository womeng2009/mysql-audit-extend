mod crontab_handle;
mod daemon_util;
mod libc_util;
mod tasks;

fn main() {
    let username = libc_util::get_current_user();
    daemon_util::daemonize(tasks::start_backstage_task, username);
}
