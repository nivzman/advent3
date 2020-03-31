use std::error;
use std::fmt;
use std::io;

#[derive(Debug, PartialEq, Eq)]
pub struct MyError {
    pub msg: String,
}

impl MyError {
    pub fn new(msg: &str) -> MyError {
        MyError { msg: String::from(msg)}
    }
}

impl error::Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError { msg: String::from(error.to_string())}
    }
}