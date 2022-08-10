use std::borrow::BorrowMut;
use std::thread::sleep;
use std::time::Duration;
use rcron::{Job, JobScheduler};

fn mysql_audit_log_rotate(sched: &mut JobScheduler, path: &str) {
    sched.add(Job::new("1/10 * * * * *".parse().unwrap(),  || {
        // let path_ref = path.clone();
        println!("执行日志轮转任务! path:");
    }));
}

pub fn start_backstage_task(path: &str) {
    let mut sched = JobScheduler::new();

    mysql_audit_log_rotate(sched.borrow_mut(), path);

    loop {
        sched.tick();

        sleep(Duration::from_secs(1));
    }
}
