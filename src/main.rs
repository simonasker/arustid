extern crate image;

use std::fs::File;
use std::path::Path;

struct Turtle {
    pos_x: u32,
    pos_y: u32,
    angle: u32,
}

impl Turtle {
    fn new(pos_x: u32, pos_y: u32, angle: u32) -> Turtle {
        Turtle {
            pos_x: pos_x,
            pos_y: pos_y,
            angle: angle,
        }
    }

    fn move_forward(&mut self, steps: u32) -> (u32, u32) {
        let new_x = self.pos_x as f32 + steps as f32 * (self.angle as f32).to_radians().cos();
        let new_y = self.pos_y as f32 + steps as f32 * (self.angle as f32).to_radians().sin();
        self.pos_x = new_x as u32;
        self.pos_y = new_y as u32;
        (self.pos_x, self.pos_y)
    }


}

fn main() {
    let base_pixel = image::Rgb([255, 255, 255]);
    let mut imgbuf = image::ImageBuffer::from_pixel(100, 100, base_pixel);

    let mut turtle = Turtle::new(50, 50, 0);

    imgbuf.put_pixel(turtle.pos_x, turtle.pos_y, image::Rgb([0, 0, 0]));
    turtle.move_forward(30);
    imgbuf.put_pixel(turtle.pos_x, turtle.pos_y, image::Rgb([0, 0, 0]));

    let ref mut fout = File::create(&Path::new("output.png")).unwrap();

    let _  = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
