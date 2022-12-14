#[cfg(target_family = "unix")]
use users::{get_current_uid, get_user_by_uid};

/// Get current operating system user
#[cfg(target_family = "unix")]
pub fn get_current_user() -> String {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    let username = user.name().to_str().unwrap().to_string();
    username
}
