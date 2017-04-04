use sdl2::rect::Point;

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

    fn move_forward(&mut self, steps: i32) -> &Point {
        let new_x = self.position.x as f32 + steps as f32 * (self.angle as f32).to_radians().cos();
        let new_y = self.position.y as f32 + steps as f32 * (self.angle as f32).to_radians().sin();
        self.position = Point::new(new_x.round() as i32, new_y.round() as i32);
        &self.position
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

    pub fn process_sequence(&mut self, sequence: String, angle: i32) -> Vec<Point> {
        let mut result = vec![self.position.clone()];
        for c in sequence.chars() {
            match c {
                'F' | 'G' | 'A' | 'B' | '1' | '0' => {
                    self.move_forward(16);
                    result.push(self.position.clone());
                }
                '+' => {
                    self.turn(-angle);
                }
                '-' => {
                    self.turn(angle);
                }
                '[' => {
                    self.stack.push((self.position.clone(), self.angle.clone()));
                    result.push(self.position.clone());
                    self.turn(-angle);
                }
                ']' => {
                    // TODO Dangerous unwrap
                    let (old_position, old_angle) = self.stack.pop().unwrap();
                    self.position = old_position;
                    self.angle = old_angle;
                    result.push(self.position.clone());
                    self.turn(angle);
                }
                _ => {}
            }
        }
        result
    }
}
