use std::fs;

pub fn solution(path_to_input: String) {
    let input = fs::read_to_string(path_to_input)
        .expect("Failed to read input");
}
