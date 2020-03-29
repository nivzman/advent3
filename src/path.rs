use std::fs;
use std::collections::HashSet;

use crate::myerror::MyError;
use crate::line::Line;
use crate::point::Point;

pub type Path = Vec<Line>;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct PathElement {
    pub direction: Direction,
    pub length: u32,
}

impl PathElement {
    pub fn from_str(data: &str) -> Result<PathElement, MyError> {
        if !data.is_ascii() {
            return Err(MyError::new("path data must be ascii encoded"));
        }

        if data.is_empty() {
            return Err(MyError::new("empty line data found"));
        }

        let direction: Direction = match data.as_bytes()[0] as char {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => return Err(MyError::new("invalid direction"))
        };

        let length = match data[1..].parse() {
            Ok(n) => n,
            Err(_) => return Err(MyError::new("invalid length"))
        };

        Ok(PathElement {direction, length})
    }
}

pub fn parse_input(input_file: &str) -> Result<(Vec<PathElement>, Vec<PathElement>), MyError> {
    let content = fs::read_to_string(input_file)?;

    let paths: Vec<&str> = content.lines().collect();
    if paths.len() != 2 {
        return Err(MyError::new("2 paths required"));
    }

    let path1 = parse_path(paths[0])?;
    let path2 = parse_path(paths[1])?;
    Ok((path1, path2))
}

pub fn parse_path(path_data: &str) -> Result<Vec<PathElement>, MyError> {
    let mut parsed:Vec<PathElement> = Vec::new();
    for line_data in path_data.split(',') {
        match PathElement::from_str(line_data) {
            Ok(element) => parsed.push(element),
            Err(e) => return Err(e),
        }
    }
    Ok(parsed)
}


pub fn find_intersections(path1: &[Line], path2: &[Line]) -> HashSet<Point> {
    let mut crossings: HashSet<Point> = HashSet::new();
    for line in path1.iter() {
        for point in line.iter() {
            if point.is_on_path(path2) {
                crossings.insert(point);
            }
        }
    }
    crossings
}
