use std::cmp::min;

use crate::point::Point;
use crate::myerror::MyError;

#[derive(Debug, PartialEq, Eq)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Line {
    orientation: Orientation,
    start: Point,
    len: usize,
}

static IDENTICAL_POINTS_ERR: &str = "identical points can not make a line";
static LINE_ORIENTATION_ERR: &str = "line must be horizontal or vertical";

impl Line {
    pub fn new(start: Point, end: Point) -> Result<Line, MyError> {
        if start == end {
            return Err(MyError::new(IDENTICAL_POINTS_ERR));
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
            return Err(MyError::new(LINE_ORIENTATION_ERR));
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


#[cfg(test)]
mod tests {
    use crate::line::{Line, IDENTICAL_POINTS_ERR, LINE_ORIENTATION_ERR, Orientation};
    use crate::point::Point;
    use crate::myerror::MyError;

    #[test]
    fn test_new_line() {
        struct TestCase {
            name: &'static str,
            points: (Point, Point),
            expected: Result<Line, MyError>,
        }

        let test_cases = [
            TestCase {
                name: "same point",
                points: (Point::new(1, 1), Point::new(1, 1)),
                expected: Err(MyError::new(IDENTICAL_POINTS_ERR)),
            },
            TestCase {
                name: "invalid orientation",
                points: (Point::new(1, 1), Point::new(2, 2)),
                expected: Err(MyError::new(LINE_ORIENTATION_ERR)),
            },
            TestCase {
                name: "positive-positive vert line",
                points: (Point::new(1, 1), Point::new(1, 2)),
                expected: Ok(Line {
                    orientation: Orientation::Vertical,
                    start: Point::new(1, 1),
                    len: 1
                }),
            },
            TestCase {
                name: "negative-positive vert line",
                points: (Point::new(1, -2), Point::new(1, 5)),
                expected: Ok(Line {
                    orientation: Orientation::Vertical,
                    start: Point::new(1, -2),
                    len: 7
                }),
            },
            TestCase {
                name: "negative-negative vert line",
                points: (Point::new(1, -2), Point::new(1, -5)),
                expected: Ok(Line {
                    orientation: Orientation::Vertical,
                    start: Point::new(1, -5),
                    len: 3
                }),
            },
            TestCase {
                name: "positive-positive horiz line",
                points: (Point::new(1, 1), Point::new(3, 1)),
                expected: Ok(Line {
                    orientation: Orientation::Horizontal,
                    start: Point::new(1, 1),
                    len: 2
                }),
            },
            TestCase {
                name: "negative-positive horiz line",
                points: (Point::new(1, 1), Point::new(-9, 1)),
                expected: Ok(Line {
                    orientation: Orientation::Horizontal,
                    start: Point::new(-9, 1),
                    len: 10
                }),
            },
            TestCase {
                name: "negative-positive horiz line",
                points: (Point::new(-6, 1), Point::new(-9, 1)),
                expected: Ok(Line {
                    orientation: Orientation::Horizontal,
                    start: Point::new(-9, 1),
                    len: 3
                }),
            },
        ];

        for test_case in test_cases.iter() {
            assert_eq!(Line::new(test_case.points.0, test_case.points.1), test_case.expected, "{}", test_case.name);
        }
    }

    #[test]
    fn test_is_on() {
        struct TestCase {
            name: &'static str,
            line: Line,
            point: Point,
            expected: bool,
        }

        let test_cases = [
            TestCase {
                name: "sanity on line",
                line: Line::new(Point::new(1, 1), Point::new(1, 5)).unwrap(),
                point: Point::new(1, 3),
                expected: true,
            },
            TestCase {
                name: "sanity off line",
                line: Line::new(Point::new(1, 1), Point::new(1, 5)).unwrap(),
                point: Point::new(1, 6),
                expected: false,
            },
            TestCase {
                name: "last point included",
                line: Line::new(Point::new(1, 1), Point::new(1, 5)).unwrap(),
                point: Point::new(1, 5),
                expected: true,
            },
            TestCase {
                name: "first point included",
                line: Line::new(Point::new(1, 1), Point::new(1, 5)).unwrap(),
                point: Point::new(1, 1),
                expected: true,
            },
            // note: there are a lot off possible test cases to check, I am not going to write them all
            //       for this is for practice
        ];

        for test_case in test_cases.iter() {
            assert_eq!(test_case.line.is_on(&test_case.point), test_case.expected, "{}", test_case.name)
        }
    }
}