extern crate image;

use std::fs::File;
use std::path::Path;

fn main() {
    let base_pixel = image::Rgb([255, 255, 255]);
    let mut imgbuf = image::ImageBuffer::from_pixel(100, 100, base_pixel);

    for i in 0..100 {
        imgbuf.put_pixel(30, i, image::Rgb([0, 0, 0]));
    }

    let ref mut fout = File::create(&Path::new("output.png")).unwrap();

    let _  = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
