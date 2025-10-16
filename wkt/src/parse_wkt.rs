use geo_types::{CoordNum, LineString};
use wkt::TryFromWkt;

/// Converts a linestring to a string in format
pub fn to_string<T: CoordNum + std::fmt::Display>(linestring: LineString<T>) -> String {
    let points: Vec<_> = linestring.into_iter().collect();
    let mut result = String::with_capacity(points.len() * 16);
    let mut is_first = true;

    for point in points {
        if !is_first {
            result.push(',');
        }
        result.push('(');
        result.push_str(&point.x.to_string());
        result.push(',');
        result.push_str(&point.y.to_string());
        result.push(')');
        is_first = false;
    }

    format!("[{}]", result)
}

pub fn parse_wkt(s: &str) -> Option<String> {
    match LineString::<f64>::try_from_wkt_str(s) {
        Ok(linestring) => Some(to_string(linestring)),
        Err(_err) => Some("".to_string()),
    }
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
