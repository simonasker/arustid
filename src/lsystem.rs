
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
