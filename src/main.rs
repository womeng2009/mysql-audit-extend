mod daemon_util;
mod libc_util;
mod tasks;

fn main() {
    let username = libc_util::get_current_user();
    daemon_util::daemonize(tasks::mysql_log_handle_task, username);
}
