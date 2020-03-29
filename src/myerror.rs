use std::error;

#[derive(Debug)]
pub struct MyError {
    pub msg: String,
}

impl MyError {
    pub fn new(msg: &str) -> MyError {
        MyError { msg: String::from(msg)}
    }
}

impl<T: error::Error> From<T> for MyError {
    fn from(error: T) -> Self {
        MyError { msg: String::from(error.to_string())}
    }
}