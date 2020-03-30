use std::cmp::min;

use crate::point::Point;
use crate::myerror::MyError;
use crate::path::{self, Path};

enum Orientation {
    Horizontal,
    Vertical,
}

pub struct Line {
    orientation: Orientation,
    start: Point,
    len: usize,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Result<Line, MyError> {
        if start == end {
            return Err(MyError::new("identical points can not make a line"));
        }

        let (orientation, start, len) = if start.x == end.x {
            (Orientation::Vertical,
             Point::new(start.x, min(start.y, end.y)),
             (end.y - start.y).abs() as usize)
        } else if start.y == end.y {
            (Orientation::Horizontal,
             Point::new(min(start.x, end.x), start.y),
             (end.x - start.x).abs() as usize)
        } else {
            return Err(MyError::new("line must be horizontal or vertical"));
        };

        Ok(
            Line {
                orientation,
                start,
                len,
            }
        )
    }

    pub fn is_on(&self, point: &Point) -> bool {
        let d = match self.orientation {
            Orientation::Horizontal => {
                if point.y != self.start.y {
                    return false;
                }
                point.x - self.start.x
            },
            Orientation::Vertical => {
                if point.x != self.start.x {
                    return false;
                }
                point.y - self.start.y
            }
        };
        d >= 0 && d as usize <= self.len
    }

    pub fn iter(&self) -> LineIter {
        LineIter { _ref: self, current_pos: 0}
    }
}

pub struct LineIter<'a> {
    _ref: &'a Line,
    current_pos: usize,
}

impl Iterator for LineIter<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos > self._ref.len {
            return None;
        }

        let mut p = self._ref.start.clone();
        match self._ref.orientation {
            Orientation::Horizontal => p.x += self.current_pos as i32,
            Orientation::Vertical => p.y += self.current_pos as i32,
        }
        self.current_pos += 1;

        Some(p)
    }
}



pub fn transform(path: Vec<path::PathElement>) -> Path {
    let mut lines: Path = Vec::new();
    let mut current_pos = Point {x: 0, y: 0};

    for path_element in path.iter() {
        let end_pos = match path_element.direction {
            path::Direction::Right => Point::new(current_pos.x + path_element.length as i32, current_pos.y),
            path::Direction::Left => Point::new(current_pos.x - path_element.length as i32, current_pos.y),
            path::Direction::Up => Point::new(current_pos.x, current_pos.y + path_element.length as i32),
            path::Direction::Down => Point::new(current_pos.x, current_pos.y - path_element.length as i32),
        };

        lines.push(Line::new(current_pos, end_pos).expect("could not create a valid line"));
        current_pos = end_pos;
    }

    lines
}
