use std::collections::HashSet;


#[derive(Hash, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point { x: self.x, y: self.y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}


pub fn get_closest(points: HashSet<Point>) -> Option<Point> {
    if points.is_empty() {
        return None;
    }

    let mut min_distance: u32 = u32::max_value();
    let mut min_point: Option<&Point> = None;

    for p in points.iter() {
        if p.x == 0 && p.y == 0 {
            continue;
        }
        let d = manhaten_distance(p);
        if d < min_distance {
            min_distance = d;
            min_point = Some(&p);
        }
    }

    Some(min_point.unwrap().clone())
}

pub fn manhaten_distance(p: &Point) -> u32 {
    p.x.abs() as u32 + p.y.abs() as u32
}