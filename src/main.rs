extern crate image;

use std::fs::File;
use std::path::Path;

struct Turtle {
    pos_x: u32,
    pos_y: u32,
}

impl Turtle {
    fn new(pos_x: u32, pos_y: u32) -> Turtle {
        Turtle {
            pos_x: pos_x,
            pos_y: pos_y,
        }
    }

    fn move_right(&mut self, steps: u32) -> Vec<(u32, u32)> {
        let mut result = Vec::new();
        for i in self.pos_x..self.pos_x+steps {
            result.push((i, self.pos_y));
        }
        self.pos_x = self.pos_x + steps;
        result
    }

}

fn main() {
    let base_pixel = image::Rgb([255, 255, 255]);
    let mut imgbuf = image::ImageBuffer::from_pixel(100, 100, base_pixel);

    let mut turtle = Turtle::new(10, 10);

    for &(x, y) in turtle.move_right(30).iter() {;
        imgbuf.put_pixel(x, y, image::Rgb([0, 0, 0]));
    }

    let ref mut fout = File::create(&Path::new("output.png")).unwrap();

    let _  = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
