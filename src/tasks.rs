use std::borrow::BorrowMut;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use rcron::{Job, JobScheduler};

fn mysql_audit_log_rotate(sched: &mut JobScheduler) {
    sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
        println!("执行日志轮转任务!");
    }));
}

pub fn start_backstage_task() {
    let mut sched = JobScheduler::new();

    mysql_audit_log_rotate(sched.borrow_mut());

    loop {
        sched.tick();

        sleep(Duration::from_secs(1));
    }
}
