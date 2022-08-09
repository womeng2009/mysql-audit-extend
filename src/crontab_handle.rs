use std::time::SystemTime;
use delay_timer::prelude::*;

fn build_task() -> Result<Task, TaskError> {
    let id = 1;
    let name = String::from("someting");
    let mut task_builder = TaskBuilder::default();

    let body = move || {
        println!("create {} task success, id[{}], time:{}", name, id, SystemTime::now().elapsed().unwrap().as_secs());
    };

    //sec   min   hour      day of month    month   day of week     year
    task_builder
        .set_frequency_repeated_by_cron_str("0 10-40 18 * * * *")
        .set_task_id(1)
        .set_maximum_running_time(30)
        .spawn_routine(body)
}

pub fn init_crontab() {
    println!("初始化定时任务");
    let delay_timer = DelayTimerBuilder::default().build();
    delay_timer.insert_task(build_task().unwrap()).unwrap();
    delay_timer.stop_delay_timer().unwrap();
}
