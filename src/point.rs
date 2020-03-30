use std::borrow::Borrow;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }

    pub fn manhaten_distance(&self) -> u32 {
        self.x.abs() as u32 + self.y.abs() as u32
    }
}


pub fn get_closest<I, T>(points: I) -> Option<T>
        where I: Iterator<Item = T>,
              T: Borrow<Point> {

    let mut points = points.peekable();
    if points.peek().is_none() {
        return None;
    }

    let mut min_point: Option<T> = None;
    let mut min_distance: u32 = u32::max_value();

    for p in points {
        let p2 = p.borrow();
        if p2.x == 0 && p2.y == 0 {
            continue;
        }
        let d = p2.manhaten_distance();
        if d < min_distance {
            min_distance = d;
            min_point = Some(p);
        }
    }

    min_point
}

#[cfg(test)]
mod tests {
    use crate::point::{Point, get_closest};


    #[test]
    fn test_closest() {
        struct TestCase {
            name: &'static str,
            points: Vec<(i32, i32)>,
            expected: Option<Point>,
        }

        let test_cases = [
            TestCase {
                name: "empty",
                points: vec![],
                expected: None,
            },
            TestCase {
                name: "sanity",
                points: vec![(-3, 0), (1, 2), (1, 1), (1, 3)],
                expected: Some(Point::new(1, 1)),
            },
            TestCase {
                name: "zero point only",
                points: vec![(0, 0)],
                expected: None,
            },
            TestCase {
                name: "zero point not returned",
                points: vec![(0, 0), (1, 1), (2, 2)],
                expected: Some(Point::new(1, 1)),
        }

        ];

        for test_case in test_cases.iter() {
            let mut points: Vec<Point> = Vec::new();
            for p in test_case.points.iter() {
                points.push(Point::new(p.0, p.1));
            }

            assert_eq!(get_closest(points.into_iter()), test_case.expected, "{}", test_case.name);
        }
    }

    #[test]
    fn test_manhaten_distance() {
        struct TestCase {
            name: &'static str,
            p: Point,
            expected: u32,
        }

        let test_cases = [
            TestCase {
                name: "zero",
                p: Point::new(0,0),
                expected: 0,
            },
            TestCase {
                name: "both positive",
                p: Point::new(4,8),
                expected: 12,
            },
            TestCase {
                name: "both negative",
                p: Point::new(-3,-7),
                expected: 10,
            },
            TestCase {
                name: "one negative one positive",
                p: Point::new(-3,12),
                expected: 15,
            },
        ];

        for test_case in test_cases.iter() {
            assert_eq!(test_case.p.manhaten_distance(), test_case.expected, "{}", test_case.name)
        }
    }
}