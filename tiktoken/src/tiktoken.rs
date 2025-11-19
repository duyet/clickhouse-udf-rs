use anyhow::Result;
use tiktoken_rs::CoreBPE;

/// Get the tokenizer for the cl100k_base encoding (used by GPT-3.5-turbo, GPT-4, text-embedding-ada-002)
fn get_tokenizer() -> Result<CoreBPE> {
    tiktoken_rs::cl100k_base()
}

/// Count the number of tokens in the input text using cl100k_base encoding.
/// This encoding is used by GPT-3.5-turbo, GPT-4, and text-embedding-ada-002.
///
/// # Arguments
/// * `s` - Input text to tokenize
///
/// # Returns
/// * `Some(String)` - Number of tokens as a string, or None if encoding fails
///
/// # Examples
/// ```ignore
/// use tiktoken::tiktoken::tiktoken_count;
/// let count = tiktoken_count("Hello, world!");
/// assert_eq!(count, Some("4".to_string()));
/// ```
pub fn tiktoken_count(s: &str) -> Option<String> {
    match get_tokenizer() {
        Ok(bpe) => {
            let tokens = bpe.encode_with_special_tokens(s);
            Some(tokens.len().to_string())
        }
        Err(_) => None,
    }
}

/// Encode the input text to a comma-separated list of token IDs using cl100k_base encoding.
/// This encoding is used by GPT-3.5-turbo, GPT-4, and text-embedding-ada-002.
///
/// # Arguments
/// * `s` - Input text to encode
///
/// # Returns
/// * `Some(String)` - Comma-separated token IDs, or None if encoding fails
///
/// # Examples
/// ```ignore
/// use tiktoken::tiktoken::tiktoken_encode;
/// let encoded = tiktoken_encode("Hello");
/// // Returns something like "9906"
/// ```
pub fn tiktoken_encode(s: &str) -> Option<String> {
    match get_tokenizer() {
        Ok(bpe) => {
            let tokens = bpe.encode_with_special_tokens(s);
            let token_str = tokens
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(",");
            Some(token_str)
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiktoken_count_simple() {
        let result = tiktoken_count("Hello, world!");
        assert!(
            result.is_some(),
            "tiktoken_count should return Some for valid input"
        );

        let count_str = result.expect("tiktoken_count returned None");
        let count: usize = count_str.parse().expect("Failed to parse count as usize");
        assert!(count > 0, "Token count should be greater than 0");
        assert!(count < 10, "Token count should be less than 10"); // Should be around 4 tokens
    }

    #[test]
    fn test_tiktoken_count_empty() {
        let result = tiktoken_count("");
        assert_eq!(result, Some("0".to_string()));
    }

    #[test]
    fn test_tiktoken_count_long_text() {
        let text =
            "The quick brown fox jumps over the lazy dog. This is a test sentence to count tokens.";
        let result = tiktoken_count(text);
        assert!(
            result.is_some(),
            "tiktoken_count should return Some for long text"
        );

        let count_str = result.expect("tiktoken_count returned None");
        let count: usize = count_str.parse().expect("Failed to parse count as usize");
        assert!(count > 10, "Token count should be greater than 10"); // Should be multiple tokens
        assert!(count < 50, "Token count should be less than 50");
    }

    #[test]
    fn test_tiktoken_encode_simple() {
        let result = tiktoken_encode("Hello");
        assert!(
            result.is_some(),
            "tiktoken_encode should return Some for valid input"
        );

        let encoded = result.expect("tiktoken_encode returned None");
        assert!(!encoded.is_empty(), "Encoded result should not be empty");
        // Should be comma-separated numbers
        assert!(
            encoded.chars().all(|c| c.is_ascii_digit() || c == ','),
            "Encoded result should only contain digits and commas"
        );
    }

    #[test]
    fn test_tiktoken_encode_empty() {
        let result = tiktoken_encode("");
        assert_eq!(result, Some("".to_string()));
    }

    #[test]
    fn test_tiktoken_encode_multiple_tokens() {
        let result = tiktoken_encode("Hello, world!");
        assert!(
            result.is_some(),
            "tiktoken_encode should return Some for multi-word input"
        );

        let encoded = result.expect("tiktoken_encode returned None");
        // Should contain commas (multiple tokens)
        assert!(
            encoded.contains(','),
            "Encoded result should contain commas for multiple tokens"
        );
    }

    #[test]
    fn test_consistency_count_and_encode() {
        let text = "Test consistency";

        let count_result = tiktoken_count(text);
        let encode_result = tiktoken_encode(text);

        assert!(count_result.is_some(), "tiktoken_count should return Some");
        assert!(
            encode_result.is_some(),
            "tiktoken_encode should return Some"
        );

        let count_str = count_result.expect("tiktoken_count returned None");
        let count: usize = count_str.parse().expect("Failed to parse count as usize");
        let encoded = encode_result.expect("tiktoken_encode returned None");
        let token_count = if encoded.is_empty() {
            0
        } else {
            encoded.split(',').count()
        };

        assert_eq!(
            count, token_count,
            "Count and encoded token count should match"
        );
    }
}
