use std::collections::HashSet;

use crate::line::Line;

#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }

    pub fn is_on_path(&self, path: &Vec<Line>) -> bool {
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
        let d = p.manhaten_distance();
        if d < min_distance {
            min_distance = d;
            min_point = Some(&p);
        }
    }

    Some(min_point.unwrap().clone())
}