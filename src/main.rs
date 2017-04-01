extern crate image;

mod turtle;
mod geom;

use geom::Point;
use std::fs::File;
use std::path::Path;

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

    let mut turtle = turtle::Turtle::new(Point::new(0, 250), 0);

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

        for Point { x, y } in geom::calculate_line(&prev, &current) {
            imgbuf.put_pixel(x as u32, y as u32, image::Rgb([0, 0, 0]));
        }

        prev = current;
    }

    let ref mut fout = File::create(&Path::new("output.png")).unwrap();

    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
