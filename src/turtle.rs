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
        let new_position = match self.angle {
            0 => Point::new(self.position.x + steps, self.position.y),
            90 => Point::new(self.position.x, self.position.y + steps),
            180 => Point::new(self.position.x - steps, self.position.y),
            270 => Point::new(self.position.x, self.position.y - steps),
            _ => {
                let new_x = self.position.x as f32 + steps as f32 * (self.angle as f32).to_radians().cos();
                let new_y = self.position.y as f32 + steps as f32 * (self.angle as f32).to_radians().sin();
                Point::new(new_x as i32, new_y as i32)
            }
        };

        self.position = new_position;
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

    pub fn process_sequence(&mut self, sequence: String) -> Vec<Point> {
        let mut result = vec![self.position.clone()];
        for c in sequence.chars() {
            match c {
                'F' => {
                    self.move_forward(10);
                    result.push(self.position.clone());
                }
                '+' => {
                    self.turn(-90);
                }
                '-' => {
                    self.turn(90);
                }
                _ => {}
            }
        }
        result
    }
}
