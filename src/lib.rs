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
               iterations: 10,
               output_filename: String::from("output.png"),
           })
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let _koch = LSystem {
        variables: vec!['F'],
        constants: vec!['+', '-'],
        axiom: String::from("F"),
        rules: vec![Rule::new('F', "F+F-F-F+F")],
        angle: 90,
    };

    let _dragon = LSystem {
        variables: vec!['X', 'Y'],
        constants: vec!['F', '+', '-'],
        axiom: String::from("FX"),
        rules: vec![Rule::new('X', "X+YF+"), Rule::new('Y', "-FX-Y")],
        angle: 90,
    };

    let sequence = _dragon.generate(config.iterations);

    let mut turtle = turtle::Turtle::new(Point::new(500, 250), _dragon.angle as i32);
    let mut path = turtle.process_sequence(sequence);

    let (min_x, max_x, min_y, max_y) = geom::find_limits(&path);

    let margin = 20;

    let dx = -min_x + margin;
    let dy = -min_y + margin;

    let width = max_x - min_x + 2 * margin;
    let height = max_y - min_y + 2 * margin;

    geom::translate(&mut path, dx, dy);

    let mut path_iter = path.iter();

    let mut prev = path_iter.next().unwrap();

    let base_pixel = image::Rgb([255, 255, 255]);
    let mut imgbuf = image::ImageBuffer::from_pixel(width as u32, height as u32, base_pixel);

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
