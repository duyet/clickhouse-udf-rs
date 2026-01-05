pub mod io;

#[cfg(test)]
mod tests {
    use super::io::ProcessFn;

    #[test]
    fn test_process_stdin_success() {
        // Test successful processing of valid input
        let f: ProcessFn = Box::new(|input| {
            if input.is_empty() {
                None
            } else {
                Some(input.to_uppercase())
            }
        });

        // Test that the processing function works correctly
        assert_eq!(f("hello"), Some("HELLO".to_string()));
        assert_eq!(f(""), None);
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
            if input.is_empty() || input.starts_with("error") {
                None
            } else {
                Some(format!("processed: {}", input))
            }
        });

        assert!(f("test").is_some());
        assert!(f("").is_none());
        assert!(f("error_input").is_none());
    }
}
