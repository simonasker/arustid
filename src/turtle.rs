
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

    fn draw_line(&mut self, p1: Point, p2: Point) {
        // TODO Handle this result better
        if let Some(renderer) = self.renderer {
            renderer
                .aa_line(p1.x as i16,
                         p1.y as i16,
                         p2.x as i16,
                         p2.y as i16,
                         Color::RGB(0, 0, 0))
                .unwrap();
        }
    }

    fn draw_rectangle(&mut self, a: Point, b: Point, c: Point, d: Point) {
        if let Some(renderer) = self.renderer {
            renderer
                .aa_polygon(&[a.x as i16, b.x as i16, c.x as i16, d.x as i16],
                            &[a.y as i16, b.y as i16, c.y as i16, d.y as i16],
                            Color::RGB(0, 0, 0))
                .unwrap();
        }
    }

    fn move_forward(&mut self, steps: i32) {
        let p1 = self.position;
        let p2 = geom::get_endpoint(p1, self.angle, steps);
        let (a, b, c, d) = geom::get_rectangle(p1, self.angle, steps);
        self.draw_line(p1, p2);
        self.draw_rectangle(a, b, c ,d);
        self.position = p2;
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
                    self.move_forward(100);
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
