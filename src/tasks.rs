use std::thread::sleep;
use std::time::{Duration, SystemTime};

pub fn mysql_log_handle_task() {
    println!("Success, daemonized");
    loop {
        println!("开始循环处理:{:?}", SystemTime::now());
        sleep(Duration::from_secs(1));
    }
}
