use geom::Point;

pub struct Turtle {
    position: Point,
    angle: i32,
}

impl Turtle {
    pub fn new(position: Point, angle: i32) -> Turtle {
        Turtle {
            position: position,
            angle: angle,
        }
    }

    fn move_forward(&mut self, steps: i32) -> &Point {
        let new_x = self.position.x as f32 + steps as f32 * (self.angle as f32).to_radians().cos();
        let new_y = self.position.y as f32 + steps as f32 * (self.angle as f32).to_radians().sin();
        self.position = Point::new(new_x as i32, new_y as i32);
        &self.position
    }

    fn turn_left(&mut self, angle: i32) {
        self.angle = (self.angle - angle) % 360;
    }

    fn turn_right(&mut self, angle: i32) {
        self.angle = (self.angle + angle) % 360;
    }

    pub fn process_sequence(&mut self, sequence: String) -> Vec<Point> {
        let mut result = vec![self.position.clone()];
        for c in sequence.chars() {
            match c {
                'F' => {
                    self.move_forward(10);
                    result.push(self.position.clone());
                }
                '+' => {
                    self.turn_left(90);
                }
                '-' => {
                    self.turn_right(90);
                }
                _ => {}
            }
        }
        result
    }
}
