use chrono::{DateTime, Local, Utc};
use rcron::{Job, JobScheduler};
use std::borrow::BorrowMut;
use std::fs;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use rust_decimal::Decimal;

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
        let r = fs::File::open(path.as_str());
        match r {
            Ok(file) => {
                let metadata = file.metadata().unwrap();
                let file_len = Decimal::from(metadata.len());
                let cf = Decimal::from(1024);
                let file_size = (file_len / cf / cf).round_dp(0);
                println!("file_size:{:?}M", file_size);
            }
            Err(e) => {
                eprintln!("Read file failed[{}]：{}", path.as_str(), e.to_string());
            }
        }
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
