extern crate sdl2;

mod turtle;
mod geom;
mod lsystem;

use sdl2::rect::Point;
use sdl2::image::SaveSurface;

use sdl2::pixels::Color;
use sdl2::surface::Surface;
use std::env;
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

    let width = 1000;
    let height = 1000;
    let surface = Surface::new(width as u32,
                               height as u32,
                               sdl2::pixels::PixelFormatEnum::RGB888)
            .unwrap();
    let mut surface_renderer = sdl2::render::Renderer::from_surface(surface).unwrap();
    surface_renderer.set_draw_color(Color::RGB(255, 255, 255));
    surface_renderer.clear();

    let mut turtle = turtle::Turtle::new(Point::new(500, 1000), 270);
    turtle.process_sequence(sequence, system.angle, &mut surface_renderer);

    let surface = surface_renderer.into_surface().unwrap();
    surface.save(Path::new(&config.output_filename)).unwrap();

    Ok(())
}
