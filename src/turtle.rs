use geom;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Renderer;
use std::error::Error;

pub struct Turtle<'a> {
    renderer: Option<&'a Renderer<'a>>,
    position: Point,
    angle: i32,
    length: i32,
    stack: Vec<(Point, i32)>,
    path: Vec<Point>,
}


impl<'a> Turtle<'a> {
    pub fn new(position: Point, angle: i32, length: i32) -> Turtle<'a> {
        Turtle {
            renderer: None,
            position: position,
            angle: angle,
            length: length,
            stack: Vec::new(),
            path: vec![position],
        }
    }

    pub fn set_renderer(&mut self, renderer: &'a mut Renderer) {
        self.renderer = Some(renderer);
    }

    pub fn get_path(&self) -> &Vec<Point> {
        &self.path
    }

    fn draw_line(&mut self, p1: Point, p2: Point) -> Result<(), Box<Error>> {
        if let Some(r) = self.renderer {
            r.aa_line( p1.x as i16, p1.y as i16, p2.x as i16, p2.y as i16, Color::RGB(0, 0, 0))?;
        }
        Ok(())
    }

    fn move_forward(&mut self) -> Result<(), Box<Error>> {
        let p1 = self.position;
        let p2 = geom::get_endpoint(p1, self.angle, self.length);
        self.draw_line(p1, p2)?;
        self.position = p2;
        self.path.push(self.position);
        Ok(())
    }

    fn turn(&mut self, angle: i32) {
        let mut new_angle = self.angle + angle;

        if new_angle >= 360 {
            new_angle = new_angle - 360;
        } else if new_angle < 0 {
            new_angle = 360 + new_angle
        }

        self.angle = new_angle;
    }

    pub fn process_sequence(&mut self, sequence: &str, angle: i32) -> Result<(), Box<Error>> {
        for c in sequence.chars() {
            match c {
                'F' | 'G' => {
                    self.move_forward()?;
                }
                '+' => {
                    self.turn(-angle);
                }
                '-' => {
                    self.turn(angle);
                }
                '[' => {
                    self.stack.push((self.position.clone(), self.angle.clone()));
                }
                ']' => {
                    let (old_position, old_angle) = self.stack.pop().ok_or("Empty stack")?;
                    self.position = old_position;
                    self.angle = old_angle;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
