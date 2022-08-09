mod daemon_util;
mod libc_util;
mod tasks;

fn main() {
    let vars1 = std::env::vars();
    for (k, v) in vars1 {
        println!("env k:{}, v:{}", k, v);
    }

    let username = libc_util::get_current_user();
    daemon_util::daemonize(tasks::start_backstage_task, username);
}
