use std::collections::HashSet;
use std::str::FromStr;

use crate::myerror::MyError;
use crate::line::Line;
use crate::point::Point;

pub type Path = Vec<Line>;
pub type PathRef = [Line];

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
    for section in path_data.split(',') {
        match PathElement::from_str(section) {
            Ok(element) => parsed.push(element),
            Err(e) => return Err(e),
        }
    }
    Ok(parsed)
}


pub fn find_intersections(path1: &PathRef, path2: &PathRef) -> HashSet<Point> {
    let mut crossings: HashSet<Point> = HashSet::new();
    for line in path1.iter() {
        for point in line.iter() {
            if is_on_path(point, path2) {
                crossings.insert(point);
            }
        }
    }
    crossings
}


pub fn transform(path_elements: Vec<PathElement>) -> Path {
    let mut path: Path = Vec::new();
    let mut current_pos = Point {x: 0, y: 0};

    for path_element in path_elements.iter() {
        let end_pos = match path_element.direction {
            Direction::Right => Point::new(current_pos.x + path_element.length as i32, current_pos.y),
            Direction::Left => Point::new(current_pos.x - path_element.length as i32, current_pos.y),
            Direction::Up => Point::new(current_pos.x, current_pos.y + path_element.length as i32),
            Direction::Down => Point::new(current_pos.x, current_pos.y - path_element.length as i32),
        };

        path.push(Line::new(current_pos, end_pos).expect("could not create a valid line"));
        current_pos = end_pos;
    }

    path
}

fn is_on_path(p: Point, path: &PathRef) -> bool {
    for line in path.iter() {
        if line.is_on(&p) {
            return true;
        }
    }
    false
}