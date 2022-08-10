use chrono::{DateTime, Local, TimeZone, Utc};
use rcron::{Job, JobScheduler};
use std::borrow::BorrowMut;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

fn mysql_audit_log_rotate(sched: &mut JobScheduler, path: String, max_size: u32, max_file: u32) {
    // utc time
    sched.add(Job::new("1/10 * * * * *".parse().unwrap(), move || {
        let utc: DateTime<Utc> = Utc::now();
        let local: DateTime<Local> = Local::now();
        println!("utc:{}", utc);
        println!("local:{}", local);
        println!(
            "执行日志轮转任务! path:{}, max_size:{}, max_file:{}, time:{:?}",
            path,
            max_size,
            max_file,
            SystemTime::now()
        );
    }));
}

pub fn start_backstage_task(path: String, max_size: u32, max_file: u32) {
    let mut sched = JobScheduler::new();

    mysql_audit_log_rotate(sched.borrow_mut(), path, max_size, max_file);

    loop {
        sched.tick();

        sleep(Duration::from_millis(500));
    }
}
