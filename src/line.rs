use std::cmp::min;

use crate::point::Point;
use crate::myerror::MyError;

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

    pub fn is_on(&self, p: &Point) -> bool {
        let d = match self.orientation {
            Orientation::Horizontal => {
                if p.y != self.start.y {
                    return false;
                }
                p.x - self.start.x
            },
            Orientation::Vertical => {
                if p.x != self.start.x {
                    return false;
                }
                p.y - self.start.y
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
