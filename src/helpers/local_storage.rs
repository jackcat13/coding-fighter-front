use web_sys::Storage;

pub const USER_SESSION: &str = "user-session";
pub const AFTER_UUID_POSITION: usize = 36;

/// Get the local storage helper function. Using unwrap is safe here because we know that the local storage is available.
pub fn local_storage() -> web_sys::Storage {
    web_sys::window().unwrap().local_storage().unwrap().unwrap()
}

/// Get the user simple name from the local storage.
pub fn resolve_user_from_storage(local_storage: &Storage) -> String {
    let user = local_storage
        .get_item(USER_SESSION)
        .expect("Failed to load user from storage");
    match user {
        None => "".to_string(),
        Some(user) => resolve_simple_user_name(user),
    }
}

pub fn resolve_simple_user_name(user: String) -> String {
    let user_name = &mut String::new();
    let _ = &user[AFTER_UUID_POSITION..].clone_into(user_name);
    user_name.to_string()
}
