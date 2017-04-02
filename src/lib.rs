extern crate image;

mod turtle;
mod geom;
mod lsystem;

use geom::Point;
use std::env;
use std::fs::File;
use std::path::Path;

pub struct Config {
    mode: String,
    iterations: u32,
    output_filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let mode = args.next().unwrap_or(String::from(""));

        let iterations = args.next().unwrap_or(String::from("1"));
        let iterations = iterations.parse::<u32>().unwrap();

        Ok(Config {
               mode: mode,
               iterations: iterations,
               output_filename: String::from("output.png"),
           })
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let system = lsystem::get_system(&config.mode);

    let sequence = system.generate(config.iterations);

    let mut turtle = turtle::Turtle::new(Point::new(0, 0), 0);
    let mut path = turtle.process_sequence(sequence, system.angle);

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
            // TODO Maybe the delta should be added here instead
            // let x = (x + dx) as u32;
            // let y = (y + dy) as u32;
            imgbuf.put_pixel(x as u32, y as u32, image::Rgb([0, 0, 0]));
        }

        prev = current;
    }

    let ref mut fout = File::create(&Path::new(&config.output_filename)).unwrap();

    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);

    Ok(())
}
