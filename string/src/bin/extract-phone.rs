use once_cell::sync::Lazy;
use regex::Regex;
use shared::io::process_stdin;

static PHONE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\+?\d[\d -]{8,}\d").expect("Invalid phone regex pattern"));

fn extract_phone(input: &str) -> Option<String> {
    if let Some(cap) = PHONE_REGEX.captures_iter(input).next() {
        let phone = cap.get(0).map_or("", |m| m.as_str());
        let normalized_phone = phone
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();

        return Some(normalized_phone.to_string());
    }

    None
}

fn main() {
    process_stdin(Box::new(extract_phone));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_phone() {
        assert_eq!(
            extract_phone("Call me at +123 456 7890 or +987 654 3210."),
            Some("1234567890".to_string())
        );
        assert_eq!(
            extract_phone("My number is 1234567890."),
            Some("1234567890".to_string())
        );
        assert_eq!(
            extract_phone("My number is 123-456-7890."),
            Some("1234567890".to_string())
        );
        assert_eq!(extract_phone("123-456"), None);
        assert_eq!(extract_phone("No phone number here."), None);
    }
}
