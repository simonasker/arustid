use std::cmp;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

pub fn calculate_line(p1: &Point, p2: &Point) -> Vec<Point> {
    let mut line = Vec::new();
    if p1.x == p2.x {
        for y in cmp::min(p1.y, p2.y)..cmp::max(p1.y, p2.y) {
            line.push(Point::new(p1.x, y));
        }
    } else if p1.y == p2.y {
        for x in cmp::min(p1.x, p2.x)..cmp::max(p1.x, p2.x) {
            line.push(Point::new(x, p1.y));
        }
    } else {
        panic!("Line not straight");
    }
    line
}
