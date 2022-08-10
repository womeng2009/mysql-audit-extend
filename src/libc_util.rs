use users::{get_current_uid, get_user_by_uid};
use anyhow::Result;

pub fn get_current_user() -> Result<String> {
    let user = get_user_by_uid(get_current_uid())?;
    let username = user.name().to_str().unwrap().to_string();
    Ok(username)
}
