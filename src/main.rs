extern crate image;

use std::fs::File;
use std::path::Path;

struct Point {
    x: u32,
    y: u32,
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
        self.position = Point { x: new_x as u32, y: new_y as u32 };
        &self.position
    }
}


fn main() {
    let base_pixel = image::Rgb([255, 255, 255]);
    let mut imgbuf = image::ImageBuffer::from_pixel(500, 500, base_pixel);

    let mut turtle = Turtle::new(Point { x: 250, y: 250 }, 0);

    imgbuf.put_pixel(turtle.position.x, turtle.position.y, image::Rgb([0, 0, 0]));
    turtle.move_forward(100);
    imgbuf.put_pixel(turtle.position.x, turtle.position.y, image::Rgb([0, 0, 0]));

    let ref mut fout = File::create(&Path::new("output.png")).unwrap();

    let _  = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
