extern crate image;

use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point { x: x, y: y }
    }
}

struct Turtle {
    position: Point,
    angle: u32,
}

impl Turtle {
    fn new(position: Point, angle: u32) -> Turtle {
        Turtle {
            position: position,
            angle: angle,
        }
    }

    fn move_forward(&mut self, steps: u32) -> &Point {
        let new_x = self.position.x as f32 + steps as f32 * (self.angle as f32).to_radians().cos();
        let new_y = self.position.y as f32 + steps as f32 * (self.angle as f32).to_radians().sin();
        self.position = Point::new(new_x as u32, new_y as u32);
        &self.position
    }

    fn process_sequence(&mut self, sequence: String) -> Vec<Point> {
        let mut result = vec![self.position.clone()];
        for c in sequence.chars() {
            match c {
                'F' => {
                    self.move_forward(15);
                    result.push(self.position.clone());
                },
                _ => {},
            }
        }
        result
    }
}

fn calculate_line(p1: &Point, p2: &Point) -> Vec<Point> {
    let mut line = Vec::new();
    if p1.x == p2.x {
        for y in p1.y..p2.y {
            line.push(Point::new(p1.x, y));
        }
    } else if p1.y == p2.y {
        for x in p1.x..p2.x {
            line.push(Point::new(x, p1.y));
        }
    } else {
        panic!("Line not straight");
    }
    line
}



fn main() {
    let base_pixel = image::Rgb([255, 255, 255]);
    let mut imgbuf = image::ImageBuffer::from_pixel(500, 500, base_pixel);

    let mut turtle = Turtle::new(Point::new(250, 250), 0);

    let path = turtle.process_sequence(String::from("FFFFF"));
    let mut path_iter = path.iter();

    let prev = path_iter.next().unwrap();

    loop {
        let current = match path_iter.next() {
            Some(p) => p,
            None => break,
        };

        for Point { x, y } in calculate_line(&prev, &current) {
            imgbuf.put_pixel(x, y, image::Rgb([0, 0, 0]));
        }
    }


    let ref mut fout = File::create(&Path::new("output.png")).unwrap();

    let _  = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
