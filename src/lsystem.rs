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
    pub angle: u16,
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
        "koch" => LSystem {
            axiom: String::from("F"),
            rules: vec![Rule::new('F', "F+F-F-F+F")],
            angle: 90,
        },
        "dragon" => LSystem {
            axiom: String::from("FX"),
            rules: vec![Rule::new('X', "X+YF+"), Rule::new('Y', "-FX-Y")],
            angle: 90,
        },
        _ => {
            // TODO Handle this nicer
            panic!("No such system");
        },
    }
}
