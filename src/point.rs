
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


pub fn get_closest<I>(mut points: I) -> Option<Point>
        where I: Iterator<Item = Point> {

    let mut min_point = points.next()?;
    let mut min_distance = min_point.manhaten_distance();

    for p in points {
        if p.x == 0 && p.y == 0 {
            continue;
        }
        let d = p.manhaten_distance();
        if d < min_distance {
            min_distance = d;
            min_point = p;
        }
    }

    Some(min_point)
}