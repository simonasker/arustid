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

pub fn find_limits(path: &Vec<Point>) -> (i32, i32, i32, i32) {
    let mut min_x = i32::max_value();
    let mut max_x = i32::min_value();
    let mut min_y = i32::max_value();
    let mut max_y = i32::min_value();

    for &Point { x, y } in path {
        min_x = cmp::min(min_x, x);
        max_x = cmp::max(max_x, x);
        min_y = cmp::min(min_y, y);
        max_y = cmp::max(max_y, y);
    }

    (min_x, max_x, min_y, max_y)
}

pub fn translate(path: &mut Vec<Point>, dx: i32, dy: i32) {
    for ref mut p in path {
        p.x += dx;
        p.y += dy;
    }
}
