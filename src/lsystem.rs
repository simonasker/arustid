
struct Rule {
    predecessor: char,
    successor: String,
}

struct System {
    variables: Vec<char>,
    constants: Vec<char>,
    axiom: String,
    rules: Vec<Rule>,
    angle: u16,
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
