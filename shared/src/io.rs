use std::io::{self, BufRead, Write};

pub type ProcessFn = Box<dyn Fn(&str) -> Option<String>>;

pub fn args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    args[1..].to_vec()
}

pub fn process_stdin(f: ProcessFn) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // Getting input from stdin line
        let input = line.unwrap_or_default();

        // Processing input
        let output = f(&input).unwrap_or_default();

        // Stdout
        println!("{}", output);
    }
}

pub fn process_stdin_send_chunk_header(f: ProcessFn) {
    let stdin = io::stdin();

    let mut lines = stdin.lock().lines();

    // Read chunk length
    while let Some(Ok(line)) = lines.next() {
        let length: usize = line.trim().parse().expect("Failed to parse chunk length");

        for _ in 0..length {
            if let Some(Ok(line)) = lines.next() {
                let output = f(&line).unwrap_or_default();
                println!("{}", output);
            }
        }

        // Flush stdout
        io::stdout().flush().expect("Error flushing stdout");
    }
}
