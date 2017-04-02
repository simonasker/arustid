use std;
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
        // TODO Try adding +1 to end of interval
        for y in cmp::min(p1.y, p2.y)..cmp::max(p1.y, p2.y) {
            line.push(Point::new(p1.x, y));
        }
    } else if p1.y == p2.y {
        // TODO Try adding +1 to end of interval
        for x in cmp::min(p1.x, p2.x)..cmp::max(p1.x, p2.x) {
            line.push(Point::new(x, p1.y));
        }
    } else {
        line = bresenham_line(p1, p2);
    }
    line
}

pub fn bresenham_line(p1: &Point, p2: &Point) -> Vec<Point> {
    let mut line = Vec::<Point>::new();

    let mut x1 = p1.x;
    let mut x2 = p2.x;
    let mut y1 = p1.y;
    let mut y2 = p2.y;

    let is_steep = (y2 - y1).abs() > (x2 - x1).abs();

    if is_steep {
        std::mem::swap(&mut x1, &mut y1);
        std::mem::swap(&mut x2, &mut y2);
    }
    let mut reversed = false;
    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
        std::mem::swap(&mut y1, &mut y2);
        reversed = true;
    }

    let dx = x2 - x1;
    let dy = (y2 - y1).abs();
    let mut err = dx / 2;
    let mut y = y1;
    let ystep: i32;

    if y1 < y2 {
        ystep = 1;
    } else {
        ystep = -1;
    }

    for x in x1..(x2 + 1) {
        if is_steep {
            line.push(Point::new(y, x));
        } else {
            line.push(Point::new(x, y));
        }
        err -= dy;

        if err < 0 {

            y += ystep;
            err += dx;
        }
    }

    if reversed {
        for i in 0..(line.len() / 2) {
            let end = line.len() - 1;
            line.swap(i, end - i);
        }
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
