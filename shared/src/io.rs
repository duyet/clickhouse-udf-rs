use std::io::{self, BufRead, Write};

pub type ProcessFn = Box<dyn Fn(&str) -> Option<String>>;

pub fn args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    args[1..].to_vec()
}

pub fn process_stdin(f: ProcessFn) {
    let stdin = io::stdin();
    let mut line_number = 0;

    for line_result in stdin.lock().lines() {
        line_number += 1;

        // Getting input from stdin line
        let input = match line_result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("ERROR: Failed to read line {}: {}", line_number, e);
                continue;
            }
        };

        // Processing input
        let output = match f(&input) {
            Some(result) => result,
            None => {
                eprintln!(
                    "ERROR: Processing failed for line {}: input={:?}",
                    line_number, input
                );
                continue;
            }
        };

        // Stdout
        println!("{}", output);
    }
}

pub fn process_stdin_send_chunk_header(f: ProcessFn) {
    let stdin = io::stdin();

    let mut lines = stdin.lock().lines();
    let mut chunk_number = 0;

    // Read chunk length
    while let Some(chunk_header) = lines.next() {
        chunk_number += 1;

        let length: usize = match chunk_header {
            Ok(line) => match line.trim().parse() {
                Ok(len) => len,
                Err(e) => {
                    eprintln!(
                        "ERROR: Failed to parse chunk {} length: {} (error: {})",
                        chunk_number, line, e
                    );
                    continue;
                }
            },
            Err(e) => {
                eprintln!("ERROR: Failed to read chunk {} header: {}", chunk_number, e);
                continue;
            }
        };

        let mut items_processed = 0;

        for item_index in 0..length {
            match lines.next() {
                Some(Ok(line)) => {
                    let output = match f(&line) {
                        Some(result) => result,
                        None => {
                            eprintln!(
                                "ERROR: Processing failed in chunk {} item {}: input={:?}",
                                chunk_number,
                                item_index + 1,
                                line
                            );
                            continue;
                        }
                    };
                    println!("{}", output);
                    items_processed += 1;
                }
                Some(Err(e)) => {
                    eprintln!(
                        "ERROR: Failed to read chunk {} item {}: {}",
                        chunk_number,
                        item_index + 1,
                        e
                    );
                }
                None => {
                    eprintln!(
                        "ERROR: Unexpected EOF in chunk {}: expected {} items, got {}",
                        chunk_number, length, items_processed
                    );
                    break;
                }
            }
        }

        if items_processed < length {
            eprintln!(
                "WARNING: Incomplete chunk {}: expected {} items, processed {}",
                chunk_number, length, items_processed
            );
        }

        // Flush stdout
        if let Err(e) = io::stdout().flush() {
            eprintln!(
                "ERROR: Failed to flush stdout after chunk {}: {}",
                chunk_number, e
            );
        }
    }
}
