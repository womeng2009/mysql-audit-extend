use std::time::Duration;
use rcron::{Job, JobScheduler};

mod daemon_util;
mod libc_util;
mod tasks;

fn main() {
    // let username = libc_util::get_current_user();
    // daemon_util::daemonize(tasks::start_backstage_task, username);

    let mut sched = JobScheduler::new();

    sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
        println!("exec task every 10 seconds!");
    }));

    sched.add(Job::new("1/5 * * * * *".parse().unwrap(), || {
        println!("exec task every 5 seconds!");
    }));

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_secs(1));
    }
}
