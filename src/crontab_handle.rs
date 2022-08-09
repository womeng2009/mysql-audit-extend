use delay_timer::prelude::*;

fn build_task() -> Result<Task, TaskError> {
    let id = 1;
    let name = String::from("someting");
    let mut task_builder = TaskBuilder::default();

    let body = move || {
        println!("create {} task success, id[{}]", name, id);
    };

    task_builder
        .set_frequency_repeated_by_cron_str("0,10,15,25,50 0/1 * * Jan-Dec * 2020-2100")
        .set_task_id(5)
        .set_maximum_running_time(5)
        .spawn_async_routine(body)
}

fn init_crontab() {
    let delay_timer = DelayTimerBuilder::default().build();
    delay_timer.insert_task(build_task().unwrap()).unwrap();
    // 停止接收新任务
    delay_timer.stop_delay_timer().unwrap();
}
