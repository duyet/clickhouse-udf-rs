use std::io::{self, BufRead};

pub fn process_stdin(f: fn(&str) -> Option<String>) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // Getting input from stdin line
        let input = line.unwrap();

        // Processing input
        let output = f(&input).unwrap_or_default();

        // Stdout
        println!("{}", output);
    }
}
