extern crate sdl2;

mod turtle;
mod geom;
mod lsystem;

use geom::Point;
use sdl2::gfx::primitives::DrawRenderer;
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

    let surface = Surface::new(width as u32,
                               height as u32,
                               sdl2::pixels::PixelFormatEnum::RGB888)
            .unwrap();
    let mut surface_renderer = sdl2::render::Renderer::from_surface(surface).unwrap();
    surface_renderer.set_draw_color(Color::RGB(255, 255, 255));
    surface_renderer.clear();

    loop {
        let current = match path_iter.next() {
            Some(p) => p,
            None => break,
        };

        surface_renderer.aa_line(prev.x as i16,
                                 prev.y as i16,
                                 current.x as i16,
                                 current.y as i16,
                                 Color::RGB(0, 0, 0))
            .unwrap();

        prev = current;
    }

    let surface = surface_renderer.into_surface().unwrap();
    surface.save(Path::new(&config.output_filename)).unwrap();

    Ok(())
}
