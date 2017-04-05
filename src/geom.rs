

use sdl2::rect::Point;
use std::cmp;

#[allow(unused)]
pub fn find_limits(path: &Vec<Point>) -> (i32, i32, i32, i32) {
    let mut min_x = i32::max_value();
    let mut max_x = i32::min_value();
    let mut min_y = i32::max_value();
    let mut max_y = i32::min_value();

    for &point in path {
        min_x = cmp::min(min_x, point.x);
        max_x = cmp::max(max_x, point.x);
        min_y = cmp::min(min_y, point.y);
        max_y = cmp::max(max_y, point.y);
    }

    (min_x, max_x, min_y, max_y)
}

#[allow(unused)]
pub fn translate(path: &mut Vec<Point>, dx: i32, dy: i32) {
    for ref mut point in path {
        point.x += dx;
        point.y += dy;
    }
}
