use std::collections::HashSet;

use crate::myerror::MyError;
use crate::line::Line;
use crate::point::Point;
use std::str::FromStr;

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

impl FromStr for PathElement {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(MyError::new("path data must be ascii encoded"));
        }

        if s.is_empty() {
            return Err(MyError::new("empty line data found"));
        }

        let direction: Direction = match s.as_bytes()[0] as char {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => return Err(MyError::new("invalid direction"))
        };

        let length = match s[1..].parse() {
            Ok(n) => n,
            Err(_) => return Err(MyError::new("invalid length"))
        };

        Ok(PathElement {direction, length})
    }
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
