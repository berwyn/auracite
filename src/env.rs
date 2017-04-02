use std::env::var_os;

pub fn load_var(key: &'static str) -> String {
    let var = var_os(key).unwrap();
    var.into_string().unwrap()
}
