/// Get the local storage helper function. Using unwrap is safe here because we know that the local storage is available.
pub fn local_storage() -> web_sys::Storage {
    web_sys::window().unwrap().local_storage().unwrap().unwrap()
}
