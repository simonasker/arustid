extern crate image;

use std::fs::File;
use std::path::Path;

fn main() {
    let mut imgbuf = image::ImageBuffer::new(100, 100);

    {
        let mut pixel = imgbuf.get_pixel_mut(50, 50);

        *pixel = image::Luma([255 as u8]);
    }

    let ref mut fout = File::create(&Path::new("output.png")).unwrap();

    let _  = image::ImageLuma8(imgbuf).save(fout, image::PNG);
}
