use sdl2::rect::Point;
use std::cmp;

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

pub fn get_endpoint(p: Point, angle: i32, distance: i32) -> Point {
    let new_x = p.x as f32 + distance as f32 * (angle as f32).to_radians().cos();
    let new_y = p.y as f32 + distance as f32 * (angle as f32).to_radians().sin();
    let new_x = new_x.round() as i32;
    let new_y = new_y.round() as i32;
    Point::new(new_x, new_y)
}
