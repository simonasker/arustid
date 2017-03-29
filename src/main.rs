extern crate image;

use std::fs::File;
use std::path::Path;

fn main() {
    let mut imgbuf = image::ImageBuffer::new(100, 100);

    {
        for i in 0..100 {
            imgbuf.put_pixel(30, i, image::Rgb([255 as u8, 255 as u8, 255 as u8]));
        }
    }

    let ref mut fout = File::create(&Path::new("output.png")).unwrap();

    let _  = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
