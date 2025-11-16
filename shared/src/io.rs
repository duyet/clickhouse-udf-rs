use std::io::{self, BufRead, Write};

/// Type alias for the processing function used by UDFs.
///
/// The function takes a string slice as input and returns an optional String.
/// Returning `None` indicates a processing error for that input.
pub type ProcessFn = Box<dyn Fn(&str) -> Option<String>>;

/// Retrieves command-line arguments excluding the program name.
///
/// # Returns
///
/// A vector of command-line arguments (excluding args[0] which is the program name).
///
/// # Examples
///
/// ```ignore
/// let args = shared::io::args();
/// if let Some(first_arg) = args.first() {
///     println!("First argument: {}", first_arg);
/// }
/// ```
pub fn args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    args[1..].to_vec()
}

/// Processes stdin line-by-line using the provided processing function.
///
/// This is the standard UDF processing mode. Each line is read from stdin,
/// processed by the provided function, and the result is written to stdout.
/// Errors during reading or processing are logged to stderr and processing continues.
///
/// # Arguments
///
/// * `f` - A boxed function that takes a string slice and returns an optional String.
///         Return `None` to indicate processing failure for a specific line.
///
/// # Examples
///
/// ```ignore
/// use shared::io::{process_stdin, ProcessFn};
///
/// fn uppercase(s: &str) -> Option<String> {
///     Some(s.to_uppercase())
/// }
///
/// fn main() {
///     process_stdin(Box::new(uppercase));
/// }
/// ```
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

/// Processes stdin in ClickHouse chunk mode with chunk headers.
///
/// This mode is used when ClickHouse sends data in batches with a chunk header
/// indicating the number of lines in each chunk. The format is:
/// - First line: number of items in the chunk (as a decimal number)
/// - Following N lines: the actual data to process
/// - After processing N lines, stdout is flushed
/// - Repeat for next chunk
///
/// This mode is enabled in ClickHouse UDF configuration with:
/// `<send_chunk_header>1</send_chunk_header>`
///
/// # Arguments
///
/// * `f` - A boxed function that takes a string slice and returns an optional String.
///         Return `None` to indicate processing failure for a specific line.
///
/// # Examples
///
/// ```ignore
/// use shared::io::{process_stdin_send_chunk_header, ProcessFn};
///
/// fn uppercase(s: &str) -> Option<String> {
///     Some(s.to_uppercase())
/// }
///
/// fn main() {
///     process_stdin_send_chunk_header(Box::new(uppercase));
/// }
/// ```
///
/// # Protocol
///
/// Input format:
/// ```text
/// 3
/// hello
/// world
/// test
/// 2
/// foo
/// bar
/// ```
///
/// Output format (after processing each chunk):
/// ```text
/// HELLO
/// WORLD
/// TEST
/// <flush>
/// FOO
/// BAR
/// <flush>
/// ```
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
