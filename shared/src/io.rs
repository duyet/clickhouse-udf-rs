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
        let length: usize = match line.trim().parse() {
            Ok(len) => len,
            Err(_) => {
                eprintln!("Failed to parse chunk length: {}", line);
                continue;
            }
        };

        for _ in 0..length {
            if let Some(Ok(line)) = lines.next() {
                let output = f(&line).unwrap_or_default();
                println!("{}", output);
            }
        }

        // Flush stdout
        if let Err(e) = io::stdout().flush() {
            eprintln!("Warning: Failed to flush stdout: {}", e);
        }
    }
}
