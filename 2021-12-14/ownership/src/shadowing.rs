use std::env::var;

pub fn get_shadowed_value() -> String {
    let list = vec!["42".to_string(), "43".to_string()];

    match list.first() {
        Some(value) => value.to_owned(),
        None => "".to_string(),
    }
}
