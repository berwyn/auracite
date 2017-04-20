use std::env::var_os;

pub fn load_var(key: &'static str, default: &'static str) -> String {
    match var_os(key) {
        Some(value) => value.into_string().unwrap(),
        None => String::from(default),
    }
}
