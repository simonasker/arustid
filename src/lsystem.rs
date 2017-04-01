
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
    pub variables: Vec<char>,
    pub constants: Vec<char>,
    pub axiom: String,
    pub rules: Vec<Rule>,
    pub angle: u16,
}


pub fn iterate(sequence: String) -> String {
    let mut result = String::new();
    for c in sequence.chars() {
        match c {
            'F' => result.push_str("F+F-F-F+F"),
            c @ _ => result.push(c),
        }
    }
    result
}
