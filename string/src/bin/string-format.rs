use anyhow::Result;
use shared::io::process_stdin;
use std::boxed::Box;

fn string_format(s: &str) -> Option<String> {
    let args = s.split('\t').collect::<Vec<&str>>();

    // Input string `s` to be formatted is the first args[0]
    // String args for s is the rest of the args
    let (s, args) = args.split_at(1);

    // Replacing each {} with the corresponding arg
    let mut result = s.get(0).map(|s| s.to_string()).unwrap_or_default();

    for arg in args.iter() {
        result = result.replacen("{}", arg, 1);
    }

    Some(result)
}

fn main() -> Result<()> {
    process_stdin(Box::new(string_format));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_format() {
        assert_eq!(string_format("Hello, {}!"), Some("Hello, {}!".to_string()));

        assert_eq!(
            string_format("Hello, {}!\tWorld"),
            Some("Hello, World!".to_string())
        );

        assert_eq!(
            string_format("Hello, {}!\tWorld\tRust"),
            Some("Hello, World!".to_string())
        );

        assert_eq!(
            string_format("Hello, {} {}!\tWorld\tRust\tis\tawesome"),
            Some("Hello, World Rust!".to_string())
        );
    }
}
