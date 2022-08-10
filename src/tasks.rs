use std::borrow::BorrowMut;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use chrono::{DateTime, Local, Utc};
use rcron::{Job, JobScheduler};

fn mysql_audit_log_rotate(sched: &mut JobScheduler, path: String) {
    sched.add(Job::new("1/10 38 8 * * *".parse().unwrap(),  move || {
        let utc: DateTime<Utc> = Utc::now();       // e.g. `2014-11-28T12:45:59.324310806Z`
        let local: DateTime<Local> = Local::now();
        println!("utc:{}", utc);
        println!("local:{}", local);
        println!("执行日志轮转任务! path:{}, time:{:?}", path, SystemTime::now());
    }));
}

pub fn start_backstage_task(path: String) {
    let mut sched = JobScheduler::new();

    mysql_audit_log_rotate(sched.borrow_mut(), path);

    loop {
        sched.tick();

        sleep(Duration::from_secs(1));
    }
}
