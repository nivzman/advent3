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