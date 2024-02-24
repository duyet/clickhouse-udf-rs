fn is_whitespace(s: &str) -> bool {
    s.chars().all(char::is_whitespace)
}

/// Returns the index to the start and the end of the URL
/// if the the given string includes a
/// URL or alike. Otherwise, returns `None`.
pub fn detect_url(s: &str) -> Option<(usize, usize)> {
    let patterns = vec!["http://", "https://", "ftp://", "ftps://", "file://"];

    for pattern in patterns {
        match s.find(pattern) {
            Some(pos) => {
                let end = s
                    .chars()
                    .skip(pos + pattern.len())
                    .position(|g| is_whitespace(g.to_string().as_str()))
                    .unwrap_or(s.len() - pos - pattern.len());
                return Some((pos, pos + end + pattern.len()));
            }
            None => continue,
        }
    }

    None
}

pub fn extract_url(s: &str) -> Option<String> {
    detect_url(s).map(|(start, end)| s[start..end].to_string())
}

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
