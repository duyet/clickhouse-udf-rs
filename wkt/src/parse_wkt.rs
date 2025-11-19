use geo_types::{CoordNum, LineString};
use wkt::TryFromWkt;

/// Converts a linestring to a string in coordinate array format.
///
/// # Arguments
///
/// * `linestring` - The LineString geometry to convert
///
/// # Returns
///
/// A string representation in the format `[(x1,y1),(x2,y2),...]`
pub fn to_string<T: CoordNum + std::fmt::Display>(linestring: LineString<T>) -> String {
    let points: Vec<_> = linestring.into_iter().collect();

    // Pre-allocate with estimated capacity: each point ~16 chars
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

/// Parses a WKT LINESTRING into coordinate array format.
///
/// # Arguments
///
/// * `s` - WKT string in format "LINESTRING(x1 y1, x2 y2, ...)"
///
/// # Returns
///
/// * `Some(String)` - Coordinate array like "[(x1,y1),(x2,y2),...]"
/// * `None` - If parsing fails or input is invalid
///
/// # Examples
///
/// ```ignore
/// use wkt::parse_wkt::parse_wkt;
///
/// let wkt = "LINESTRING(0 0, 1 1, 2 2)";
/// assert_eq!(parse_wkt(wkt), Some("[(0,0),(1,1),(2,2)]".to_string()));
/// ```
pub fn parse_wkt(s: &str) -> Option<String> {
    match LineString::<f64>::try_from_wkt_str(s) {
        Ok(linestring) => Some(to_string(linestring)),
        Err(_) => None, // Return None for invalid input instead of empty string
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
        assert_eq!(parse_wkt(input), None);
    }

    #[test]
    fn test_empty_linestring() {
        let input = "LINESTRING EMPTY";
        let result = parse_wkt(input);
        // Should either be None or empty array
        assert!(result.is_none() || result == Some("[]".to_string()));
    }

    #[test]
    fn test_single_point_linestring() {
        let input = "LINESTRING(5 10)";
        let result = parse_wkt(input);
        assert!(result.is_some());
    }

    #[test]
    fn test_floating_point_coordinates() {
        let input = "LINESTRING(1.5 2.5, 3.7 4.9)";
        let result = parse_wkt(input);
        assert!(result.is_some());
        assert!(result.unwrap().contains("1.5"));
    }

    #[test]
    fn test_negative_coordinates() {
        let input = "LINESTRING(-1 -2, 3 4, -5 6)";
        let result = parse_wkt(input);
        assert!(result.is_some());
    }

    #[test]
    fn test_invalid_format() {
        assert_eq!(parse_wkt(""), None);
        assert_eq!(parse_wkt("POINT(0 0)"), None);
        assert_eq!(parse_wkt("POLYGON((0 0, 1 1, 0 1, 0 0))"), None);
        assert_eq!(parse_wkt("invalid"), None);
    }
}
