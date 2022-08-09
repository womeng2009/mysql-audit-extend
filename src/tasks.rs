use std::thread::sleep;
use std::time::{Duration, SystemTime};
use crate::crontab_handle::init_crontab;

pub fn start_backstage_task() {
    println!("this is a test ... ");
    init_crontab();
}
