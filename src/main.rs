extern crate image;

use std::cmp;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

struct Turtle {
    position: Point,
    angle: i32,
}

impl Turtle {
    fn new(position: Point, angle: i32) -> Turtle {
        Turtle {
            position: position,
            angle: angle,
        }
    }

    fn move_forward(&mut self, steps: i32) -> &Point {
        let new_x = self.position.x as f32 + steps as f32 * (self.angle as f32).to_radians().cos();
        let new_y = self.position.y as f32 + steps as f32 * (self.angle as f32).to_radians().sin();
        self.position = Point::new(new_x as i32, new_y as i32);
        &self.position
    }

    fn turn_left(&mut self, angle: i32) {
        self.angle = (self.angle - angle) % 360;
    }

    fn turn_right(&mut self, angle: i32) {
        self.angle = (self.angle + angle) % 360;
    }

    fn process_sequence(&mut self, sequence: String) -> Vec<Point> {
        let mut result = vec![self.position.clone()];
        for c in sequence.chars() {
            match c {
                'F' => {
                    self.move_forward(10);
                    result.push(self.position.clone());
                }
                '+' => {
                    self.turn_left(90);
                }
                '-' => {
                    self.turn_right(90);
                }
                _ => {}
            }
        }
        result
    }
}

fn calculate_line(p1: &Point, p2: &Point) -> Vec<Point> {
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

fn iterate_sequence(sequence: String) -> String {
    let mut result = String::new();
    for c in sequence.chars() {
        match c {
            'F' => result.push_str("F+F-F-F+F"),
            c @ _ => result.push(c),
        }
    }

    result
}

fn main() {
    let base_pixel = image::Rgb([255, 255, 255]);
    let mut imgbuf = image::ImageBuffer::from_pixel(500, 500, base_pixel);

    let mut turtle = Turtle::new(Point::new(0, 250), 0);

    let mut sequence = String::from("F+F-F-F+F");

    for _ in 0..2 {
        sequence = iterate_sequence(sequence);
    }

    let path = turtle.process_sequence(sequence);
    let mut path_iter = path.iter();

    let mut prev = path_iter.next().unwrap();

    loop {
        let current = match path_iter.next() {
            Some(p) => p,
            None => break,
        };

        for Point { x, y } in calculate_line(&prev, &current) {
            imgbuf.put_pixel(x as u32, y as u32, image::Rgb([0, 0, 0]));
        }

        prev = current;
    }

    let ref mut fout = File::create(&Path::new("output.png")).unwrap();

    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
