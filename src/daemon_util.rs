use crate::tasks::start_backstage_task;
use daemonize::Daemonize;
use std::fs;
use std::fs::File;

pub fn daemonize(
    username: &str,
    pkg_name: &str,
    author_name: &str,
    path: String,
    max_size: u32,
    max_file: u32,
) {
    let mut base_path = String::new();
    base_path.push_str("/tmp/");
    base_path.push_str(author_name);
    base_path.push_str("/");
    base_path.push_str(pkg_name);
    fs::create_dir_all(base_path.as_str()).unwrap();
    let stdout = File::create(base_path.as_str().to_owned() + "/mysql-audit-extend.log").unwrap();
    let stderr = File::create(base_path.as_str().to_owned() + "/mysql-audit-extend.error").unwrap();

    let daemonize = Daemonize::new()
        .pid_file(base_path.to_owned() + "/mysql-audit-extend.pid")
        .chown_pid_file(true)
        .working_directory(base_path.as_str())
        .user(username)
        .group(1)
        .umask(0o777)
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => {
            start_backstage_task(path, max_size, max_file);
        }
        Err(e) => eprintln!("Error, {}", e),
    }
}
