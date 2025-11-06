use anyhow::Result;
use shared::io::{ProcessFn, args, process_stdin};
use std::boxed::Box;
use topk::FilteredSpaceSaving;

fn topk_fn(k: usize) -> ProcessFn {
    Box::new(move |s| -> Option<String> {
        if k == 0 {
            return Some("[]".to_string());
        }

        // s = [1,2,3.4]
        let array = s
            .split(',')
            .map(|i| i.trim_start_matches('[').trim_end_matches(']').trim())
            .collect::<Vec<&str>>();

        if array.is_empty() {
            return Some("[]".to_string());
        }

        let mut topk = FilteredSpaceSaving::new(k);
        for i in array {
            topk.insert(i, 1);
        }

        let mut topk_result = topk.into_sorted_vec();

        // Stable sort: first by frequency (descending), then by value (ascending) for deterministic ordering
        topk_result.sort_by(|a, b| {
            a.1.estimated_count()
                .cmp(&b.1.estimated_count())
                .reverse()
                .then_with(|| a.0.cmp(&b.0))
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
        Some(k) => k.parse::<usize>()?,
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
}
