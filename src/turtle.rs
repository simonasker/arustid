
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Renderer;

use geom;

pub struct Turtle<'a> {
    renderer: Option<&'a Renderer<'a>>,
    position: Point,
    angle: i32,
    stack: Vec<(Point, i32)>,
    path: Vec<Point>,
}

impl<'a> Turtle<'a> {
    pub fn new(position: Point, angle: i32) -> Turtle<'a> {
        Turtle {
            renderer: None,
            position: position,
            angle: angle,
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

    fn move_forward(&mut self, steps: i32) {
        let endpoint = geom::get_endpoint(self.position, self.angle, steps);
        // TODO Handle this result better
        if let Some(renderer) = self.renderer {
            renderer
                .aa_line(self.position.x as i16,
                        self.position.y as i16,
                        endpoint.x as i16,
                        endpoint.y as i16,
                        Color::RGB(0, 0, 0))
                .unwrap();
        }
        self.position = endpoint;
        self.path.push(self.position);
    }

    fn turn(&mut self, angle: i32) {
        // TODO Can probably be nicer
        let mut new_angle = self.angle + angle;

        if new_angle >= 360 {
            new_angle = new_angle - 360;
        } else if new_angle < 0 {
            new_angle = 360 + new_angle
        }

        self.angle = new_angle;
    }

    pub fn process_sequence(&mut self, sequence: &str, angle: i32) {
        for c in sequence.chars() {
            match c {
                'F' | 'G' | 'A' | 'B' | '1' | '0' => {
                    self.move_forward(10);
                }
                '+' => {
                    self.turn(-angle);
                }
                '-' => {
                    self.turn(angle);
                }
                '[' => {
                    self.stack.push((self.position.clone(), self.angle.clone()));
                    self.turn(-angle);
                }
                ']' => {
                    // TODO Dangerous unwrap
                    let (old_position, old_angle) = self.stack.pop().unwrap();
                    self.position = old_position;
                    self.angle = old_angle;
                    self.turn(angle);
                }
                '(' => {
                    self.stack.push((self.position.clone(), self.angle.clone()));
                }
                ')' => {
                    // TODO Dangerous unwrap
                    let (old_position, old_angle) = self.stack.pop().unwrap();
                    self.position = old_position;
                    self.angle = old_angle;
                }
                _ => {}
            }
        }
    }
}
