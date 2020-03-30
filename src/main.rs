use std::env;
use std::fs;

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


pub fn parse_input(input_file: &str) -> Result<(Vec<path::PathElement>, Vec<path::PathElement>), MyError> {
    let content = fs::read_to_string(input_file)?;

    let paths: Vec<&str> = content.lines().collect();
    if paths.len() != 2 {
        return Err(MyError::new("2 paths required"));
    }

    let path1 = path::parse_path(paths[0])?;
    let path2 = path::parse_path(paths[1])?;
    Ok((path1, path2))
}


fn main() -> Result<(), MyError> {
    let input_file = input_file()?;

    let (path1, path2) = parse_input(&input_file)?;

    let (path1, path2) = (path::transform(path1), path::transform(path2));

    let intersections = path::find_intersections(&path1, &path2);

    if intersections.is_empty() {
        println!("no crossings");
        return Ok(());
    }

    let closest = point::get_closest(intersections.into_iter()).unwrap();
    println!("{:?}", closest.manhaten_distance());
    return Ok(())
}
