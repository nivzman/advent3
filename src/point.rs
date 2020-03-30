
use crate::line::Line;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }

    pub fn is_on_path(&self, path: &[Line]) -> bool {
        for line in path.iter() {
            if line.is_on(self) {
                return true;
            }
        }
        false
    }

    pub fn manhaten_distance(&self) -> u32 {
        self.x.abs() as u32 + self.y.abs() as u32
    }
}


pub fn get_closest<I>(points: I) -> Option<Point>
        where I: Iterator<Item = Point> {

    let mut points = points.peekable();
    if points.peek().is_none() {
        return None;
    }

    let mut min_point: Option<Point> = None;
    let mut min_distance: u32 = u32::max_value();

    for p in points {
        if p.x == 0 && p.y == 0 {
            continue;
        }
        let d = p.manhaten_distance();
        if d < min_distance {
            min_distance = d;
            min_point = Some(p);
        }
    }

    min_point
}