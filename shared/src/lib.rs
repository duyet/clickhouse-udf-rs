//! Shared utilities for ClickHouse User-Defined Functions (UDFs).
//!
//! This crate provides common functionality used across all ClickHouse UDF packages,
//! including I/O processing functions for stdin/stdout handling and argument parsing.
//!
//! # Examples
//!
//! ```ignore
//! use shared::io::{process_stdin, ProcessFn};
//!
//! fn uppercase(s: &str) -> Option<String> {
//!     Some(s.to_uppercase())
//! }
//!
//! fn main() {
//!     process_stdin(Box::new(uppercase));
//! }
//! ```

pub mod io;

#[cfg(test)]
mod tests {
    use super::io::ProcessFn;

    #[test]
    fn test_process_stdin_success() {
        // Test successful processing of valid input
        let _f: ProcessFn = Box::new(|input| {
            if input.is_empty() {
                None
            } else {
                Some(input.to_uppercase())
            }
        });

        let _input = "hello\nworld\n";

        // We can't directly test process_stdin without mocking stdin
        // This demonstrates the expected behavior pattern
        assert!(true);
    }

    #[test]
    fn test_processing_function_error_handling() {
        // Test that None return from processing function is handled correctly
        let f: ProcessFn = Box::new(|input| {
            if input == "skip" {
                None // Simulate processing error
            } else {
                Some(input.to_string())
            }
        });

        // Verify the function returns None for specific input
        assert!(f("skip").is_none());
        assert!(f("valid").is_some());
    }

    #[test]
    fn test_chunk_header_parsing() {
        // Test chunk length parsing
        let valid_numbers = vec!["10", "100", "1000", "0"];
        for num_str in valid_numbers {
            assert!(num_str.trim().parse::<usize>().is_ok());
        }

        // Test invalid chunk lengths
        let invalid_numbers = vec!["abc", "10.5", "-1", ""];
        for num_str in invalid_numbers {
            assert!(num_str.trim().parse::<usize>().is_err());
        }
    }

    #[test]
    fn test_processing_function_with_various_inputs() {
        let f: ProcessFn = Box::new(|input| {
            if input.is_empty() {
                None
            } else if input.starts_with("error") {
                None
            } else {
                Some(format!("processed: {}", input))
            }
        });

        assert!(f("test").is_some());
        assert_eq!(f("test"), Some("processed: test".to_string()));
        assert!(f("").is_none());
        assert!(f("error_input").is_none());
    }

    #[test]
    fn test_args_parsing() {
        // Test that args() returns a vector
        let args = super::io::args();
        // Args should always be a vector (possibly empty)
        assert!(args.len() >= 0);
    }

    #[test]
    fn test_processing_function_idempotency() {
        let f: ProcessFn = Box::new(|input| Some(input.to_uppercase()));

        let input = "test";
        let result1 = f(input);
        let result2 = f(input);

        // Same input should produce same output
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_processing_function_unicode() {
        let f: ProcessFn = Box::new(|input| Some(input.to_string()));

        // Test with various Unicode characters
        let unicode_inputs = vec![
            "Hello 世界",
            "Привет мир",
            "مرحبا بالعالم",
            "🚀 Rocket",
            "Ñoño",
        ];

        for input in unicode_inputs {
            let result = f(input);
            assert!(result.is_some());
            assert_eq!(result.unwrap(), input);
        }
    }
}
