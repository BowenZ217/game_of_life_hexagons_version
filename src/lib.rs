use std::io::stdin;

pub fn is_number(input: String) -> bool {
    for c in input.trim().chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

pub fn read_one() -> String {
    let mut words = String::new();
    stdin()
        .read_line(&mut words)
        .ok()
        .expect("Failed to read line.");
    words
}

pub fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}