use sdl2::rect::Point;
use sdl2::render::Renderer;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

pub struct Turtle {
    position: Point,
    angle: i32,
    stack: Vec<(Point, i32)>,
}

impl Turtle {
    pub fn new(position: Point, angle: i32) -> Turtle {
        Turtle {
            position: position,
            angle: angle,
            stack: Vec::new(),
        }
    }

    fn move_forward(&mut self, steps: i32, renderer: &mut Renderer) {
        let new_x = self.position.x as f32 + steps as f32 * (self.angle as f32).to_radians().cos();
        let new_y = self.position.y as f32 + steps as f32 * (self.angle as f32).to_radians().sin();
        let new_x = new_x.round() as i32;
        let new_y = new_y.round() as i32;
        // TODO Handle this result better
        renderer.aa_line(self.position.x as i16, self.position.y as i16, new_x as i16, new_y as i16, Color::RGB(0, 0, 0)).unwrap();
        self.position = Point::new(new_x, new_y);
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

    pub fn process_sequence(&mut self, sequence: String, angle: i32, renderer: &mut Renderer) {
        for c in sequence.chars() {
            match c {
                'F' | 'G' | 'A' | 'B' | '1' | '0' => {
                    self.move_forward(16, renderer);
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
                _ => {}
            }
        }
    }
}
