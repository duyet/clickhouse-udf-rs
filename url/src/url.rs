//! URL detection and extraction utilities.
//!
//! This module provides functions for detecting, extracting, and validating URLs
//! within text strings. It supports common protocols including HTTP, HTTPS, FTP,
//! FTPS, and FILE.
//!
//! # Supported Protocols
//!
//! - `http://`
//! - `https://`
//! - `ftp://`
//! - `ftps://`
//! - `file://`
//!
//! # Examples
//!
//! ```ignore
//! use url::url::{detect_url, extract_url, has_url};
//!
//! // Detect URL boundaries
//! let (start, end) = detect_url("Visit https://example.org for more").unwrap();
//! assert_eq!((start, end), (6, 25));
//!
//! // Extract the URL
//! let url = extract_url("Visit https://example.org for more").unwrap();
//! assert_eq!(url, "https://example.org");
//!
//! // Check if text contains a URL
//! assert_eq!(has_url("https://example.org").unwrap(), "true");
//! assert_eq!(has_url("no url here").unwrap(), "false");
//! ```

/// Static array of URL protocol patterns
const URL_PATTERNS: &[&str] = &["http://", "https://", "ftp://", "ftps://", "file://"];

/// Detects the position of a URL within a string.
///
/// Searches for the first occurrence of a recognized URL protocol and determines
/// the boundaries of the complete URL by finding the next whitespace character.
///
/// # Arguments
///
/// * `s` - The input string to search for URLs
///
/// # Returns
///
/// * `Some((start, end))` - Tuple of byte indices marking the URL boundaries
///   - `start`: Index where the URL begins (start of protocol)
///   - `end`: Index where the URL ends (at whitespace or end of string)
/// * `None` - If no URL is found
///
/// # URL Detection Logic
///
/// 1. Searches for the first matching protocol from `URL_PATTERNS`
/// 2. Finds the end of the URL by locating the next whitespace character
/// 3. If no whitespace is found, extends to the end of the string
///
/// # Examples
///
/// ```ignore
/// use url::detect_url;
///
/// // URL at the start
/// assert_eq!(detect_url("https://example.org"), Some((0, 19)));
///
/// // URL in the middle
/// assert_eq!(
///     detect_url("Visit https://example.org today"),
///     Some((6, 25))
/// );
///
/// // URL with path
/// assert_eq!(
///     detect_url("https://example.org/path/to/page something"),
///     Some((0, 28))
/// );
///
/// // No URL found
/// assert_eq!(detect_url("no url here"), None);
/// ```
pub fn detect_url(s: &str) -> Option<(usize, usize)> {
    for &pattern in URL_PATTERNS {
        match s.find(pattern) {
            Some(pos) => {
                let remaining = &s[pos + pattern.len()..];
                let end_offset = remaining
                    .find(char::is_whitespace)
                    .unwrap_or(remaining.len());
                return Some((pos, pos + pattern.len() + end_offset));
            }
            None => continue,
        }
    }

    None
}

/// Extracts the URL from a string, if one exists.
///
/// This is a convenience function that combines `detect_url()` with string slicing
/// to return the actual URL text rather than its position.
///
/// # Arguments
///
/// * `s` - The input string to search for a URL
///
/// # Returns
///
/// * `Some(String)` - The extracted URL (including protocol)
/// * `None` - If no URL is found in the input string
///
/// # Examples
///
/// ```ignore
/// use url::extract_url;
///
/// assert_eq!(
///     extract_url("Visit https://example.org today"),
///     Some("https://example.org".to_string())
/// );
///
/// assert_eq!(
///     extract_url("https://example.org/path/to/page something"),
///     Some("https://example.org/path/to/page".to_string())
/// );
///
/// assert_eq!(extract_url("no url here"), None);
///
/// // Works with different protocols
/// assert_eq!(
///     extract_url("Download from ftp://files.example.org now"),
///     Some("ftp://files.example.org".to_string())
/// );
/// ```
pub fn extract_url(s: &str) -> Option<String> {
    detect_url(s).map(|(start, end)| s[start..end].to_string())
}

/// Checks whether a string contains a URL.
///
/// Returns a string representation of a boolean value ("true" or "false")
/// indicating whether the input contains a recognized URL.
///
/// This function is designed for ClickHouse UDF usage where string outputs
/// are preferred over boolean types.
///
/// # Arguments
///
/// * `s` - The input string to check for URLs
///
/// # Returns
///
/// * `Some("true")` - If the string contains a URL
/// * `Some("false")` - If the string does not contain a URL
///
/// Note: This function always returns `Some`, never `None`.
///
/// # Examples
///
/// ```ignore
/// use url::has_url;
///
/// assert_eq!(has_url("https://example.org"), Some("true".to_string()));
/// assert_eq!(has_url("Visit https://example.org"), Some("true".to_string()));
/// assert_eq!(has_url("no url here"), Some("false".to_string()));
/// assert_eq!(has_url(""), Some("false".to_string()));
///
/// // Works with all supported protocols
/// assert_eq!(has_url("ftp://example.org"), Some("true".to_string()));
/// assert_eq!(has_url("file:///path/to/file"), Some("true".to_string()));
/// ```
pub fn has_url(s: &str) -> Option<String> {
    match detect_url(s).is_some() {
        true => Some("true".to_string()),
        false => Some("false".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASES: [(&str, Option<&str>); 16] = [
        ("http://example.org", Some("http://example.org")),
        ("https://example.org", Some("https://example.org")),
        ("ftp://example.org", Some("ftp://example.org")),
        ("ftps://example.org", Some("ftps://example.org")),
        ("file://example.org", Some("file://example.org")),
        (
            "aaa http://example.org something",
            Some("http://example.org"),
        ),
        (
            "aaa https://example.org something",
            Some("https://example.org"),
        ),
        ("aaa ftp://example.org something", Some("ftp://example.org")),
        (
            "aaa ftps://example.org something",
            Some("ftps://example.org"),
        ),
        (
            "aaa file://example.org something",
            Some("file://example.org"),
        ),
        ("https://example.org something", Some("https://example.org")),
        (
            "https://example.org/abc/deff something",
            Some("https://example.org/abc/deff"),
        ),
        ("aaa ftp://example.org something", Some("ftp://example.org")),
        ("file://example.org", Some("file://example.org")),
        ("aaa http not an url", None),
        ("", None),
    ];

    #[test]
    fn test_extract_url() {
        for (input, expected) in TEST_CASES.iter() {
            assert_eq!(
                extract_url(input),
                expected.map(|s| s.to_string()),
                "expected extract_url({:?}) to be {:?} but got {:?}",
                input,
                expected,
                extract_url(input)
            );
        }
    }

    #[test]
    fn test_has_url() {
        for (input, expected) in TEST_CASES.iter() {
            assert_eq!(
                has_url(input).unwrap(),
                expected.map_or("false", |_| "true"),
                "expected has_url({:?}) to be {:?} but got {:?}",
                input,
                expected,
                extract_url(input)
            );
        }
    }
}
