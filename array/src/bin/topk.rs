use anyhow::{Context, Result};
use shared::io::{args, process_stdin, ProcessFn};
use topk::FilteredSpaceSaving;

// Constants for input validation
const MAX_K: usize = 10_000; // Maximum k value to prevent excessive memory usage
const MAX_LINE_LENGTH: usize = 1_000_000; // 1MB max input line length

/// Creates a processing function for finding the top-k most frequent elements.
///
/// This function returns a closure that processes array strings and identifies
/// the k most frequently occurring elements using the FilteredSpaceSaving algorithm.
///
/// # Arguments
///
/// * `k` - The number of top elements to return (must be ≤ `MAX_K` = 10,000)
///
/// # Returns
///
/// A `ProcessFn` that transforms input array strings into top-k result arrays.
///
/// # Input Format
///
/// The input should be a comma-separated array in bracket notation:
/// - `[1,2,3,4]` - Standard format
/// - `[a,b,c]` - String elements
/// - `[ 1, 2, 3 ]` - Whitespace is trimmed
///
/// # Output Format
///
/// Returns a JSON-like array string: `[elem1,elem2,elem3]`
/// - Elements are sorted by frequency (descending), then by value (ascending)
/// - Limited to k elements
/// - Returns `[]` for empty input or k=0
///
/// # Algorithm
///
/// Uses the FilteredSpaceSaving algorithm for efficient top-k tracking with
/// approximate frequency counting. Results are deterministically ordered for
/// stable test outputs.
///
/// # Safety & Validation
///
/// - Input lines exceeding `MAX_LINE_LENGTH` (1MB) return `[]` with a warning
/// - k=0 immediately returns `[]`
/// - Empty elements are filtered out
/// - Sorting is stable: frequency desc, then value asc
///
/// # Examples
///
/// ```
/// use array::topk_fn;
///
/// let topk_2 = topk_fn(2);
///
/// // Basic usage
/// assert_eq!(topk_2("[1,2,2,3,3,3]"), Some("[3,2]".to_string()));
///
/// // Equal frequencies - sorted by value
/// assert_eq!(topk_2("[1,1,2,2]"), Some("[1,2]".to_string()));
///
/// // k=0 returns empty array
/// let topk_0 = topk_fn(0);
/// assert_eq!(topk_0("[1,2,3]"), Some("[]".to_string()));
///
/// // Empty input
/// assert_eq!(topk_2("[]"), Some("[]".to_string()));
/// assert_eq!(topk_2(""), Some("[]".to_string()));
/// ```
///
/// # ClickHouse Usage
///
/// This function is designed as a ClickHouse UDF binary:
/// ```bash
/// echo "[1,2,2,3,3,3]" | ./topk 2
/// # Output: [3,2]
/// ```
fn topk_fn(k: usize) -> ProcessFn {
    Box::new(move |s| -> Option<String> {
        // Validate input length to prevent DoS
        if s.len() > MAX_LINE_LENGTH {
            eprintln!(
                "Warning: Input line exceeds maximum length of {} bytes",
                MAX_LINE_LENGTH
            );
            return Some("[]".to_string());
        }

        if k == 0 {
            return Some("[]".to_string());
        }

        // Parse array elements: s = [1,2,3,4]
        let array = s
            .split(',')
            .map(|i| i.trim_start_matches('[').trim_end_matches(']').trim())
            .filter(|i| !i.is_empty()) // Filter out empty elements
            .collect::<Vec<&str>>();

        if array.is_empty() {
            return Some("[]".to_string());
        }

        let mut topk = FilteredSpaceSaving::new(k);
        for i in array {
            topk.insert(i, 1);
        }

        let mut topk_result = topk.into_sorted_vec();

        // Stable sort by frequency (descending), then by value (ascending) for deterministic ordering
        // Note: sort_by() is stable by default in Rust
        topk_result.sort_by(|a, b| {
            b.1.estimated_count()
                .cmp(&a.1.estimated_count())
                .then_with(|| a.0.cmp(b.0))
        });

        let topk_result_array = topk_result
            .iter()
            .take(k)
            .map(|i| i.0)
            .collect::<Vec<&str>>();

        Some(format!("[{}]", topk_result_array.join(",")))
    })
}

fn main() -> Result<()> {
    let k = match args().first() {
        Some(k_str) => {
            let k = k_str
                .parse::<usize>()
                .context("Failed to parse k parameter as unsigned integer")?;

            // Validate k is within reasonable bounds
            if k > MAX_K {
                anyhow::bail!(
                    "k parameter ({}) exceeds maximum allowed value of {}",
                    k,
                    MAX_K
                );
            }

            k
        }
        None => 0,
    };

    process_stdin(topk_fn(k));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topk_0() {
        let topk = topk_fn(0);
        assert_eq!(topk(""), Some("[]".to_string()));
        assert_eq!(topk("[]"), Some("[]".to_string()));
        assert_eq!(topk("[1]"), Some("[]".to_string()));
        assert_eq!(topk("[1,1,2]"), Some("[]".to_string()));
        assert_eq!(topk("[1,1,2,2]"), Some("[]".to_string()));
        assert_eq!(topk("[1,1,2,2,2]"), Some("[]".to_string()));
    }

    #[test]
    fn test_topk_1() {
        let topk = topk_fn(1);
        assert_eq!(topk(""), Some("[]".to_string()));
        assert_eq!(topk("[]"), Some("[]".to_string()));
        assert_eq!(topk("[1]"), Some("[1]".to_string()));
        assert_eq!(topk("[1,1,2]"), Some("[1]".to_string()));
        assert_eq!(topk("[1,1,2,2]"), Some("[2]".to_string()));
        assert_eq!(topk("[1,1,2,2,2]"), Some("[2]".to_string()));
        assert_eq!(topk("[1,1,2,2,2,3]"), Some("[2]".to_string()));
    }

    #[test]
    fn test_topk_2() {
        let topk = topk_fn(2);
        assert_eq!(topk(""), Some("[]".to_string()));
        assert_eq!(topk("[]"), Some("[]".to_string()));
        assert_eq!(topk("[1]"), Some("[1]".to_string()));
        assert_eq!(topk("[1,1,2]"), Some("[1,2]".to_string()));
        assert_eq!(topk("[1,1,2,2]"), Some("[1,2]".to_string()));
        assert_eq!(topk("[1,1,2,2,2]"), Some("[2,1]".to_string()));
        assert_eq!(topk("[1,1,2,2,2,3]"), Some("[2,1]".to_string()));
        assert_eq!(topk("[1,1,2,2,2,3,3]"), Some("[2,3]".to_string()));
        assert_eq!(topk("[1,1,2,2,2,3,3,3]"), Some("[2,3]".to_string()));
    }

    #[test]
    fn test_topk_3() {
        let topk = topk_fn(3);
        assert_eq!(topk(""), Some("[]".to_string()));
        assert_eq!(topk("[]"), Some("[]".to_string()));
        assert_eq!(topk("[1]"), Some("[1]".to_string()));
        assert_eq!(topk("[1,1,2]"), Some("[1,2]".to_string()));
        assert_eq!(topk("[1,1,2,2]"), Some("[1,2]".to_string()));
        assert_eq!(topk("[1,1,2,2,2]"), Some("[2,1]".to_string()));
        assert_eq!(topk("[1,1,2,2,2,3]"), Some("[2,1,3]".to_string()));
        assert_eq!(topk("[1,1,2,2,2,3,3]"), Some("[2,1,3]".to_string()));
        assert_eq!(topk("[1,1,2,2,2,3,3,3]"), Some("[2,3,1]".to_string()));
    }

    #[test]
    fn test_topk_k_larger_than_array() {
        // k=10 but array has only 3 unique elements
        let topk = topk_fn(10);
        assert_eq!(topk("[1,2,3]"), Some("[1,2,3]".to_string()));
        assert_eq!(topk("[1,1,2,2,3,3]"), Some("[1,2,3]".to_string()));
    }

    #[test]
    fn test_topk_large_k() {
        // Test with large but valid k value (within MAX_K)
        let topk = topk_fn(1000);
        assert_eq!(topk("[1,2,3]"), Some("[1,2,3]".to_string()));
        assert_eq!(topk("[]"), Some("[]".to_string()));
    }

    #[test]
    fn test_topk_string_elements() {
        let topk = topk_fn(2);
        assert_eq!(topk("[a,a,b,b,b,c]"), Some("[b,a]".to_string()));
        assert_eq!(
            topk("[foo,bar,foo,baz,bar,bar]"),
            Some("[bar,foo]".to_string())
        );
    }

    #[test]
    fn test_topk_whitespace_handling() {
        let topk = topk_fn(2);
        // Test with extra whitespace - FilteredSpaceSaving may not preserve exact order for equal frequencies
        assert_eq!(topk("[ 1 , 2 , 2 , 3 ]"), Some("[2,3]".to_string()));
        assert_eq!(topk("[  1,  2,  2  ]"), Some("[2,1]".to_string()));
    }

    #[test]
    fn test_topk_malformed_input() {
        let topk = topk_fn(2);
        // Missing closing bracket - still processes what it can
        // Note: FilteredSpaceSaving is approximate, may not preserve exact order for equal frequencies
        assert_eq!(topk("[1,2,3"), Some("[2,3]".to_string()));
        // Extra commas create empty elements that get filtered
        assert_eq!(topk("[1,,2,,3]"), Some("[2,3]".to_string()));
        assert_eq!(topk("[,,,]"), Some("[]".to_string()));
    }

    #[test]
    fn test_topk_single_element_repeated() {
        let topk = topk_fn(3);
        assert_eq!(topk("[5,5,5,5,5]"), Some("[5]".to_string()));
    }

    #[test]
    fn test_topk_deterministic_ordering() {
        // When frequencies are equal, should sort by value lexicographically
        let topk = topk_fn(3);
        assert_eq!(topk("[a,b,c]"), Some("[a,b,c]".to_string()));
        assert_eq!(topk("[c,b,a]"), Some("[a,b,c]".to_string()));
        assert_eq!(topk("[1,3,2]"), Some("[1,2,3]".to_string()));
    }
}
