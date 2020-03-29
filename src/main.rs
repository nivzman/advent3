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

fn main() -> Result<(), MyError> {
    let input_file = input_file()?;

    let (path1, path2) = path::parse_input(&input_file)?;

    let (path1, path2) = (line::transform(path1), line::transform(path2));

    let intersections = path::find_intersections(path1, path2);

    if intersections.is_empty() {
        println!("no crossings");
        return Ok(());
    }

    let closest = point::get_closest(intersections).unwrap();
    println!("{:?}", closest.manhaten_distance());
    return Ok(())
}
