use shared::io::process_stdin;
use regex::Regex;

fn extract_phone(input: &str) -> Option<String> {
    let phone_regex = Regex::new(r"\+?\d[\d -]{8,}\d").unwrap();
    let mut numbers = Vec::new();

    for cap in phone_regex.captures_iter(input) {
        let phone = cap.get(0).map_or("", |m| m.as_str());
        let normalized_phone = phone.chars().filter(|c| c.is_digit(10)).collect::<String>();
        numbers.push(normalized_phone);
    }

    numbers.first().map(|number| number.to_string())
}

fn main() {
    process_stdin(Box::new(|input| {
        extract_phone(input)
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_phone() {
        assert_eq!(extract_phone("Call me at +123 456 7890 or +987 654 3210."), Some("1234567890".to_string()));
        assert_eq!(extract_phone("My number is 123-456-7890."), Some("1234567890".to_string()));
        assert_eq!(extract_phone("No phone number here."), None);
    }
}
