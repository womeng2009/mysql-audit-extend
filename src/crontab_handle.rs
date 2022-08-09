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
        // .set_frequency_repeated_by_cron_str("0 0/1 * * * * *")
        .set_frequency_repeated_by_seconds(1)
        .set_task_id(1)
        .set_maximum_running_time(30)
        .spawn_routine(body)
}

pub fn init_crontab(){
    println!("初始化定时任务");
    let delay_timer = DelayTimerBuilder::default().build();
    let task_instance_chain = delay_timer.insert_task(build_task().unwrap()).unwrap();
    let task_instance = task_instance_chain.next_with_wait().unwrap();

    // Cancel running task instances.
    println!("state:{}", task_instance.get_state());
    // No new tasks are accepted; running tasks are not affected.
    // delay_timer.stop_delay_timer().expect("停止接收新任务失败");
}
