use std::fs;
use std::fs::File;
use daemonize::Daemonize;

pub fn daemonize<T: FnOnce()>(task: T, username: String) {
    let base_path = "/tmp/seeker/mglog";
    fs::create_dir_all(base_path).unwrap();
    let stdout = File::create(base_path.to_owned() + "/mglog.log").unwrap();
    let stderr = File::create(base_path.to_owned() + "/mglog.error").unwrap();

    let daemonize = Daemonize::new()
        .pid_file(base_path.to_owned() + "/mglog.pid")
        .chown_pid_file(true)
        .working_directory(base_path)
        .user(username.as_str())
        .group(1)
        .umask(0o777)
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => {
            task();
        },
        Err(e) => eprintln!("Error, {}", e),
    }
}