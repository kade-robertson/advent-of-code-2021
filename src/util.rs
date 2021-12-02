#[cfg(debug_assertions)]
use std::{fs::File, io::Read};

#[cfg(debug_assertions)]
pub fn read_file(path: &str) -> Option<String> {
    let file: Option<File> = match File::open(path) {
        Ok(valid_file) => Some(valid_file),
        Err(_e) => None,
    };

    if file.is_none() {
        return None;
    }

    let mut input = String::new();
    match file.unwrap().read_to_string(&mut input) {
        Ok(_valid) => Some(input),
        Err(_e) => None,
    }
}
