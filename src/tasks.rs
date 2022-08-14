use chrono::{DateTime, Local};
use rcron::{Job, JobScheduler};
use rust_decimal::Decimal;
use std::borrow::BorrowMut;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration};

/// Handling mysql audit logs
fn mysql_audit_log_handle(sched: &mut JobScheduler, path: String, max_size: u32, max_file: u32) {
    let run_flag = Arc::new(Mutex::new(false));

    sched.add(Job::new("0/1 * * * * *".parse().unwrap(), move || {
        let run_flag = Arc::clone(&run_flag);
        let mut is_run = run_flag.lock().unwrap();
        if !(*is_run) {
            *is_run = true;
            let r = fs::File::options().write(true).open(path.as_str());
            match r {
                Ok(file) => {
                    let metadata = file.metadata().unwrap();
                    let file_len = Decimal::from(metadata.len());
                    let cf = Decimal::from(1024);
                    let file_size = (file_len / cf / cf).round_dp(2);
                    let file_max_size = Decimal::from(max_size);
                    if file_size >= file_max_size {
                        log::info!("The file size reaches the split standard:{:?}M", file_size);
                        let file_path = Path::new(path.as_str());
                        let origin_file_name =
                            file_path.file_name().unwrap().to_str().unwrap().to_string();
                        let sv: Vec<&str> = origin_file_name.split(".").collect();
                        let origin_name = sv[0];
                        let origin_file_type = sv[1];
                        let parent_path = file_path.parent().unwrap();
                        let local: DateTime<Local> = Local::now();
                        let time_str = local.format("%Y%m%d%H%M%S").to_string();
                        let new_file_name =
                            origin_name.to_owned() + "-" + time_str.as_str() + "." + origin_file_type;
                        let new_file_path =
                            parent_path.to_str().unwrap().to_owned() + "/" + new_file_name.as_str();
                        fs::copy(file_path, new_file_path.as_str()).unwrap();
                        log::info!("Log file copied to:{}", new_file_path);

                        file.set_len(0).unwrap();

                        let dir_files = fs::read_dir(parent_path).unwrap();
                        let mut files = dir_files
                            .into_iter()
                            .map(|d| d.unwrap().file_name().into_string().unwrap())
                            .filter(|f| f.starts_with(origin_name))
                            .collect::<Vec<String>>();
                        files.sort();
                        if files.len() > max_file as usize {
                            log::info!("The number of files exceeds the limit,start cleaning...");
                            for i in 0..(files.len() - max_file as usize) {
                                let item_file_name = files.get(i).unwrap();
                                let item_path =
                                    parent_path.to_str().unwrap().to_owned() + "/" + item_file_name;
                                fs::remove_file(item_path.as_str())
                                    .expect("Failed to clean up redundant files");
                                log::info!("Delete file {}", item_path);
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!("Read file failed[{}]：{}", path.as_str(), e.to_string());
                }
            }
        }
        *is_run = false;
    }));
}

/// Scheduled task control
pub fn start_backstage_task(path: String, max_size: u32, max_file: u32) {
    let mut sched = JobScheduler::new();

    mysql_audit_log_handle(sched.borrow_mut(), path, max_size, max_file);

    loop {
        sched.tick();

        sleep(Duration::from_millis(500));
    }
}
