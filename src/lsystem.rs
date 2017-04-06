pub struct Rule {
    pub predecessor: char,
    pub successor: String,
}

impl Rule {
    pub fn new(predecessor: char, successor: &str) -> Rule {
        Rule {
            predecessor: predecessor,
            successor: String::from(successor),
        }
    }
}

pub struct LSystem {
    pub axiom: String,
    pub rules: Vec<Rule>,
    pub angle: i32,
}

impl LSystem {
    pub fn generate(&self, iterations: u32) -> String {
        let mut result = self.axiom.clone();
        for _ in 0..iterations {
            result = self.iterate(result);
        }
        result
    }

    pub fn iterate(&self, sequence: String) -> String {
        let mut result = String::new();
        for c in sequence.chars() {
            // TODO This could probably be nicer
            let mut applied_rule = false;
            for rule in &self.rules {
                if c == rule.predecessor {
                    result.push_str(&rule.successor);
                    applied_rule = true;
                }
            }
            if !applied_rule {
                result.push(c);
            }
        }
        result
    }
}

pub fn get_system(name: &str) -> LSystem {
    match name {
        "turtle" => {
            LSystem {
                axiom: String::from("F"),
                rules: vec![],
                angle: 45,
            }
        }
        "koch" => {
            LSystem {
                axiom: String::from("F"),
                rules: vec![Rule::new('F', "F+F-F-F+F")],
                angle: 90,
            }
        }
        "dragon" => {
            LSystem {
                axiom: String::from("FX"),
                rules: vec![Rule::new('X', "X+YF+"), Rule::new('Y', "-FX-Y")],
                angle: 90,
            }
        }
        "sierpinski" => {
            LSystem {
                axiom: String::from("F-G-G"),
                rules: vec![Rule::new('F', "F-G+F+G-F"), Rule::new('G', "GG")],
                angle: 120,
            }
        }
        "arrowhead" => {
            LSystem {
                axiom: String::from("A"),
                rules: vec![Rule::new('A', "+B-A-B+"), Rule::new('B', "-A+B+A-")],
                angle: 60,
            }
        }
        "gosper" => {
            LSystem {
                axiom: String::from("A"),
                rules: vec![Rule::new('A', "A-B--B+A++AA+B-"), Rule::new('B', "+A-BB--B-A++A+B")],
                angle: 60,
            }
        }
        "pythagora" => {
            LSystem {
                axiom: String::from("0"),
                rules: vec![Rule::new('1', "11"), Rule::new('0', "1[0]0")],
                angle: 60,
            }
        }
        "plant" => {
            LSystem {
                axiom: String::from("X"),
                rules: vec![Rule::new('X', "F-((X)+X)+F(+FX)-X"), Rule::new('F', "FF")],
                angle: 25,
            }
        }
        "plant2" => {
            LSystem {
                axiom: String::from("X"),
                rules: vec![Rule::new('X', "F(+X)(-X)FX"), Rule::new('F', "FF")],
                angle: 25,
            }
        }
        _ => {
            // TODO Handle this nicer
            panic!("No such system");
        }
    }
}
