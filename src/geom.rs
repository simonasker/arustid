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
