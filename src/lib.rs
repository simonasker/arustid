extern crate sdl2;

mod turtle;
mod geom;
pub mod lsystem;

use sdl2::event::Event;
use sdl2::image::SaveSurface;
use sdl2::keyboard::Keycode;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::surface::Surface;
use std::path::Path;

pub struct Config {
    pub system: lsystem::LSystem,
    pub iterations: u32,
    pub output_filename: String,
}

pub fn run(config: Config) -> Result<(), &'static str> {
    draw_to_window(config)
}

pub fn draw_to_window(config: Config) -> Result<(), &'static str> {
    let system = config.system;
    let sequence = system.generate(config.iterations);

    let (min_x, max_x, min_y, max_y);

    {
        let mut turtle = turtle::Turtle::new(Point::new(0, 0), 270);
        turtle.process_sequence(&sequence, system.angle);
        let limits = geom::find_limits(turtle.get_path());
        min_x = limits.0;
        max_x = limits.1;
        min_y = limits.2;
        max_y = limits.3;
    }

    let margin = 20;
    let width = max_x - min_x + 2 * margin;
    let height = max_y - min_y + 2 * margin;
    let start_x = 0 - min_x + margin;
    let start_y = 0 - min_y + margin;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("arustid", width as u32, height as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut renderer = window.renderer().build().unwrap();
    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();

    {
        let mut turtle = turtle::Turtle::new(Point::new(start_x, start_y), 270);
        turtle.set_renderer(&mut renderer);
        turtle.process_sequence(&sequence, system.angle);
    }

    renderer.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
    }

    Ok(())
}

pub fn draw_to_image(config: Config) -> Result<(), &'static str> {
    let system = config.system;
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

    {
        let mut turtle = turtle::Turtle::new(Point::new(500, 1000), 270);
        turtle.set_renderer(&mut surface_renderer);
        turtle.process_sequence(&sequence, system.angle);
    }

    let surface = surface_renderer.into_surface().unwrap();
    surface.save(Path::new(&config.output_filename)).unwrap();

    Ok(())
}
