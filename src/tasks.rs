use std::thread::sleep;
use std::time::{Duration, SystemTime};
use rcron::{Job, JobScheduler};

pub fn start_backstage_task() {
    let mut sched = JobScheduler::new();

    sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
        println!("exec task every 10 seconds!");
    }));

    sched.add(Job::new("1/5 * * * * *".parse().unwrap(), || {
        println!("exec task every 5 seconds!");
    }));

    loop {
        sched.tick();

        sleep(Duration::from_secs(1));
    }
}
