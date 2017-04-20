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

use std::error::Error;

const MARGIN: i32 = 20;

pub struct Config {
    pub system: lsystem::LSystem,
    pub iterations: u32,
    pub length: i32,
    pub output_filename: Option<String>,
}

pub fn run(mode: &str, config: Config) -> Result<(), &'static str> {
    // TODO These modes should be handled as an enum
    match mode {
        "window" => {
            let _ = render_to_window(config);
        }
        "image" => {
            let _ = render_to_image(config);
        }
        _ => panic!("No such mode"),
    }

    Ok(())
}

fn render_to_window(config: Config) -> Result<(), Box<Error>> {
    let system = config.system;
    let sequence = system.generate(config.iterations);
    let (width, height, start_x, start_y) =
        calculate_dimensions(&sequence, config.length, system.angle);
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("arustid", width as u32, height as u32)
        .position_centered()
        .opengl()
        .build()?;
    let mut renderer = window.renderer().build()?;
    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();

    {
        let mut turtle = turtle::Turtle::new(Point::new(start_x, start_y), 270, config.length);
        turtle.set_renderer(&mut renderer);
        turtle.process_sequence(&sequence, system.angle);
    }

    renderer.present();
    let mut event_pump = sdl_context.event_pump()?;

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

fn render_to_image(config: Config) -> Result<(), Box<Error>> {
    let system = config.system;
    let sequence = system.generate(config.iterations);
    let (width, height, start_x, start_y) =
        calculate_dimensions(&sequence, config.length, system.angle);

    let surface = Surface::new(width as u32,
                               height as u32,
                               sdl2::pixels::PixelFormatEnum::RGB888)?;
    let mut surface_renderer = sdl2::render::Renderer::from_surface(surface)?;
    surface_renderer.set_draw_color(Color::RGB(255, 255, 255));
    surface_renderer.clear();

    {
        let mut turtle = turtle::Turtle::new(Point::new(start_x, start_y), 270, config.length);
        turtle.set_renderer(&mut surface_renderer);
        turtle.process_sequence(&sequence, system.angle);
    }

    let surface = surface_renderer.into_surface().ok_or("No surface")?;

    // TODO There has to be a chainable function for this!
    match config.output_filename {
        Some(filename) => surface.save(Path::new(&filename))?,
        None => return Err(From::from("Hello")),
    }

    Ok(())
}

pub fn calculate_dimensions(sequence: &str, length: i32, angle: i32) -> (i32, i32, i32, i32) {
    let mut turtle = turtle::Turtle::new(Point::new(0, 0), 270, length);
    turtle.process_sequence(&sequence, angle);
    let (min_x, max_x, min_y, max_y) = geom::find_limits(turtle.get_path());

    let margin = MARGIN;
    let width = max_x - min_x + 2 * margin;
    let height = max_y - min_y + 2 * margin;
    let start_x = 0 - min_x + margin;
    let start_y = 0 - min_y + margin;

    (width, height, start_x, start_y)
}
