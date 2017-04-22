use std::error::Error;

#[derive(Debug)]
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

    pub fn from_string(string: &str) -> Result<Rule, Box<Error>> {
        let mut split = string.splitn(2, ':');

        let pred = match split.next() {
            Some(s) => s.chars().nth(0).ok_or("No predecessor")?,
            None => return Err(From::from("No predecessor")),
        };

        let succ = match split.next() {
            Some(s) => String::from(s),
            None => return Err(From::from("No successor")),
        };

        Ok(
            Rule {
                predecessor: pred,
                successor: succ,
            },
        )
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
