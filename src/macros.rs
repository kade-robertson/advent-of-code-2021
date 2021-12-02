#[cfg(debug_assertions)]
macro_rules! get_input {
    ($path:expr) => {
        match crate::util::read_file($path) {
            Some(data) => data,
            None => "".to_string(),
        }
    };
}

#[cfg(not(debug_assertions))]
macro_rules! get_input {
    ($path:expr) => {
        include_str!(concat!(".", $path)).to_string()
    };
}
