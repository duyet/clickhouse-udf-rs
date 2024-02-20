use anyhow::Result;
use geo_types::{CoordNum, LineString};
use shared::io::process_stdin;
use wkt::TryFromWkt;

/// Converts a linestring to a string in format
fn to_string<T: CoordNum + std::fmt::Display>(linestring: LineString<T>) -> String {
    let mut result = "".to_string();

    for point in linestring {
        result.push_str(&format!("({},{}),", point.x, point.y));
    }

    // Remove trailing comma and space
    result.pop();

    format!("[{}]", result)
}

fn parse_wkt(s: &str) -> Option<String> {
    match LineString::<f64>::try_from_wkt_str(s) {
        Ok(linestring) => Some(to_string(linestring)),
        Err(_err) => Some("".to_string()),
    }
}

fn main() -> Result<()> {
    process_stdin(parse_wkt);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input() {
        let input = "LINESTRING(0 0,    1 1,    2 2)";
        assert_eq!(parse_wkt(input).unwrap(), "[(0,0),(1,1),(2,2)]");
    }

    #[test]
    fn test_invalid_input() {
        let input = "LINESTRING(0 0, 1 1, 2 2";
        assert_eq!(parse_wkt(input).unwrap(), "");
    }
}
