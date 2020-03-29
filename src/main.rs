use std::process::exit;
use std::env;

mod point;
mod line;
mod path;
mod myerror;

use myerror::MyError;

pub fn input_file() -> Result<String, MyError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(MyError::new("expecting 1 argument (input file path)"));
    }
    Ok(args[1].clone())
}

fn main() {
    let input_file = match input_file() {
        Ok(path) => path,
        Err(e) => {
            println!("{}", e.msg);
            exit(1);
        }
    };

    let (path1, path2) = match path::parse_input(&input_file) {
        Ok((p1, p2)) => (p1, p2),
        Err(e) => {
            println!("{}", e.msg);
            exit(1);
        }
    };

    let (path1, path2) = (line::transform(path1), line::transform(path2));
    let intersections = path::find_intersections(path1, path2);
    if intersections.is_empty() {
        println!("no crossings");
        return;
    }

    let closest = point::get_closest(intersections).unwrap();
    println!("{:?}", closest.manhaten_distance());
}
