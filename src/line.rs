use std::cmp::min;

use crate::point::Point;
use crate::myerror::MyError;
use crate::path;


enum LineState {
    Horizontal,
    Vertical,
}

pub struct Line {
    state: LineState,
    start_pos: Point,
    len: i32,
    current_pos: i32,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Result<Line, MyError> {
        if start == end {
            return Err(MyError::new("identical points can not make a line"));
        }

        let (state, start_pos, len) = if start.x == end.x {
            (LineState::Vertical,
             Point::new(start.x, min(start.y, end.y)),
             (end.y - start.y).abs())
        } else if start.y == end.y {
            (LineState::Horizontal,
             Point::new(min(start.x, end.x), start.y),
             (end.x - start.x).abs())
        } else {
            return Err(MyError::new("line must be horizontal or vertical"));
        };

        Ok(
            Line {
                state,
                start_pos,
                len,
                current_pos: 0,
            }
        )
    }

    pub fn is_on(&self, point: &Point) -> bool {
        let d = match self.state {
            LineState::Horizontal => {
                if point.y != self.start_pos.y {
                    return false;
                }
                point.x - self.start_pos.x
            },
            LineState::Vertical => {
                if point.x != self.start_pos.x {
                    return false;
                }
                point.y - self.start_pos.y
            }
        };
        d >= 0 && d <= self.len
    }
}

impl Iterator for Line {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos > self.len {
            return None;
        }

        let mut p = self.start_pos.clone();
        match self.state {
            LineState::Horizontal => p.x += self.current_pos,
            LineState::Vertical => p.y += self.current_pos,
        }
        self.current_pos += 1;

        Some(p)
    }
}


pub fn transform(path: Vec<path::PathElement>) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    let mut current_pos = Point {x: 0, y: 0};

    for path_element in path.iter() {
        let end_pos = match path_element.direction {
            path::Direction::Right => Point::new(current_pos.x + path_element.length as i32, current_pos.y),
            path::Direction::Left => Point::new(current_pos.x - path_element.length as i32, current_pos.y),
            path::Direction::Up => Point::new(current_pos.x, current_pos.y + path_element.length as i32),
            path::Direction::Down => Point::new(current_pos.x, current_pos.y - path_element.length as i32),
        };

        lines.push(Line::new(current_pos.clone(), end_pos.clone()).unwrap());
        current_pos = end_pos;
    }

    lines
}
