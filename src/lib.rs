extern crate image;

mod turtle;
mod geom;
mod lsystem;

use geom::Point;
use lsystem::{LSystem, Rule};
use std::env;
use std::fs::File;
use std::path::Path;

pub struct Config {
    iterations: u32,
    output_filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        Ok(Config {
               iterations: 4,
               output_filename: String::from("output.png"),
           })
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let base_pixel = image::Rgb([255, 255, 255]);
    let mut imgbuf = image::ImageBuffer::from_pixel(1000, 500, base_pixel);

    let mut turtle = turtle::Turtle::new(Point::new(0, 450), 0);

    let koch = LSystem {
        variables: vec!['F'],
        constants: vec!['+', '-'],
        axiom: String::from("F"),
        rules: vec![Rule::new('F', "F+F-F-F+F")],
        angle: 90,
    };

    let sequence = koch.generate(config.iterations);

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

    let ref mut fout = File::create(&Path::new(&config.output_filename)).unwrap();

    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);

    Ok(())
}
