//! Input/output processing utilities for ClickHouse User-Defined Functions (UDFs).
//!
//! This module provides the core I/O infrastructure for building ClickHouse UDF binaries.
//! It handles stdin/stdout communication between ClickHouse and Rust executables, supporting
//! both standard line-by-line processing and ClickHouse's chunk-based protocol.
//!
//! # Architecture
//!
//! All UDF binaries follow a consistent pattern:
//! 1. Read input from stdin (either line-by-line or in chunks)
//! 2. Apply a transformation function to each input
//! 3. Write results to stdout
//!
//! # Examples
//!
//! ```no_run
//! use shared::io::{process_stdin, ProcessFn};
//!
//! let transformer: ProcessFn = Box::new(|input| {
//!     Some(input.to_uppercase())
//! });
//!
//! process_stdin(transformer);
//! ```

use std::io::{self, BufRead, Write};

/// Type alias for UDF processing functions.
///
/// A `ProcessFn` takes a string slice as input and returns an `Option<String>`:
/// - `Some(String)` indicates successful processing and provides the output
/// - `None` indicates processing failure, which will be logged to stderr
///
/// # Examples
///
/// ```
/// use shared::io::ProcessFn;
///
/// let processor: ProcessFn = Box::new(|input| {
///     if input.is_empty() {
///         None
///     } else {
///         Some(format!("Processed: {}", input))
///     }
/// });
/// ```
pub type ProcessFn = Box<dyn Fn(&str) -> Option<String>>;

/// Retrieves command-line arguments passed to the UDF binary.
///
/// Returns all arguments except the program name (i.e., `args[1..]`).
/// This is typically used to extract configuration parameters like `k` in topk functions.
///
/// # Returns
///
/// A vector of strings containing all command-line arguments after the program name.
/// Returns an empty vector if no arguments were provided.
///
/// # Examples
///
/// ```no_run
/// use shared::io::args;
///
/// // If called with: ./my-udf 100 --verbose
/// let arguments = args();
/// assert_eq!(arguments, vec!["100", "--verbose"]);
/// ```
#[inline]
pub fn args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    args[1..].to_vec()
}

/// Processes stdin line-by-line using the provided transformation function.
///
/// This is the standard processing mode for ClickHouse UDFs. Each line from stdin
/// is passed to the processing function `f`, and the result is written to stdout.
///
/// # Arguments
///
/// * `f` - A boxed function that transforms each input line into an optional output string
///
/// # Behavior
///
/// - Reads stdin line-by-line until EOF
/// - Applies the transformation function to each line
/// - Writes successful results to stdout
/// - Logs errors to stderr for failed reads or transformations
/// - Continues processing remaining lines even after errors
///
/// # Error Handling
///
/// - I/O errors reading stdin are logged with line numbers
/// - Processing failures (when `f` returns `None`) are logged with input details
/// - The function continues processing after errors rather than panicking
///
/// # Examples
///
/// ```no_run
/// use shared::io::{process_stdin, ProcessFn};
///
/// // Create a UDF that converts input to uppercase
/// let uppercase_processor: ProcessFn = Box::new(|input| {
///     Some(input.to_uppercase())
/// });
///
/// process_stdin(uppercase_processor);
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

/// Processes stdin using ClickHouse's chunk-based protocol with chunk headers.
///
/// This is an alternative processing mode for ClickHouse UDFs that supports batch processing.
/// Input is received in chunks, where each chunk starts with a header line containing the
/// number of items in the chunk, followed by that many data lines.
///
/// # Protocol Format
///
/// ```text
/// <chunk_length>
/// <item_1>
/// <item_2>
/// ...
/// <item_n>
/// <chunk_length>
/// <item_1>
/// ...
/// ```
///
/// # Arguments
///
/// * `f` - A boxed function that transforms each input line into an optional output string
///
/// # Behavior
///
/// - Reads chunk length from a header line
/// - Processes exactly that many data lines
/// - Flushes stdout after each chunk
/// - Repeats until EOF
/// - Logs errors for malformed chunks or processing failures
///
/// # Error Handling
///
/// - Invalid chunk headers (non-numeric) are logged and skipped
/// - I/O errors reading data lines are logged but don't halt chunk processing
/// - Processing failures (when `f` returns `None`) are logged with context
/// - Incomplete chunks (EOF before all items read) generate warnings
/// - Stdout flush failures are logged
///
/// # Examples
///
/// ```no_run
/// use shared::io::{process_stdin_send_chunk_header, ProcessFn};
///
/// // Create a UDF that doubles numbers
/// let doubler: ProcessFn = Box::new(|input| {
///     input.parse::<i32>()
///         .ok()
///         .map(|n| (n * 2).to_string())
/// });
///
/// // Input format:
/// // 3
/// // 10
/// // 20
/// // 30
/// // Outputs: 20, 40, 60 (then flushes stdout)
/// process_stdin_send_chunk_header(doubler);
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
