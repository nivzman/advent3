
#[derive(Debug)]
pub struct MyError {
    pub msg: String,
}

impl MyError {
    pub fn new(msg: &str) -> MyError {
        MyError { msg: String::from(msg)}
    }
}