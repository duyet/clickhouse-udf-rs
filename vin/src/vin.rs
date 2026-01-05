//! Vehicle Identification Number (VIN) parsing and processing utilities.
//!
//! This module provides functions for extracting and validating information from VINs,
//! including manufacturer identification, model year, continent of origin, and VIN cleaning/validation.
//!
//! # VIN Structure
//!
//! A standard VIN is a 17-character code that encodes information about a vehicle:
//! - Characters 1-3: World Manufacturer Identifier (WMI)
//! - Character 10: Model year (North America and some other regions)
//! - Character 11: Model year (Europe)
//!
//! # Examples
//!
//! ```
//! use vin::*;
//!
//! // Clean and validate a VIN
//! let cleaned = vin_cleaner("1G1ND52F14M712344").unwrap();
//! assert_eq!(cleaned, "1G1ND52F14M712344");
//!
//! // Get manufacturer
//! let manufacturer = vin_manuf("1G1ND52F14M712344").unwrap();
//! assert_eq!(manufacturer, "Chevrolet USA");
//!
//! // Get model year
//! let year = vin_year("1G1ND52F14M712344").unwrap();
//! assert_eq!(year, "2004");
//!
//! // Get continent
//! let continent = vin_continent("1G1ND52F14M712344").unwrap();
//! assert_eq!(continent, "North America");
//! ```

use chrono::{Datelike, Local};
use csv::ReaderBuilder;
use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

static WMI_DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "src/wmi.csv"));

static VIN_REGEXES: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        Regex::new(r"[A-HJ-NPR-Z0-9]{17}").expect("Invalid VIN regex pattern 1"),
        Regex::new(r"[A-HJ-NPR-Z0-9]{16}").expect("Invalid VIN regex pattern 2"),
        Regex::new(r"[A-HJ-NPR-Z0-9]{18}").expect("Invalid VIN regex pattern 3"),
        Regex::new(r"[A-Z0-9]{17}").expect("Invalid VIN regex pattern 4"),
    ]
});

static WMI_MAP: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(WMI_DATA.as_bytes());

    for record in reader.records().flatten() {
        if record.len() >= 2 {
            let wmi = record[0].trim().to_string();
            let manuf = record[1].trim().to_string();
            map.insert(wmi, manuf);
        }
    }
    map
});

/// Returns a reference to the World Manufacturer Identifier (WMI) lookup table.
///
/// The WMI table maps manufacturer codes to human-readable manufacturer names.
/// The data is loaded from an embedded CSV file at compile time.
///
/// # Returns
///
/// A reference to a `HashMap` where:
/// - Keys are WMI codes (typically 2-3 characters)
/// - Values are manufacturer names (e.g., "Chevrolet USA", "Toyota")
///
/// # Examples
///
/// ```
/// use vin::get_wmicsv;
///
/// let wmi_map = get_wmicsv();
/// assert_eq!(wmi_map.get("1G1").unwrap(), "Chevrolet USA");
/// ```
#[inline]
pub fn get_wmicsv() -> &'static HashMap<String, String> {
    &WMI_MAP
}

/// Extracts the World Manufacturer Identifier (WMI) from a VIN.
///
/// The WMI is typically the first 3 characters of a VIN. However, when the 3rd
/// character is '9', the WMI may extend to include characters 12-14 as well,
/// representing smaller manufacturers.
///
/// # Arguments
///
/// * `vin` - The VIN string to extract the WMI from (will be cleaned first)
///
/// # Returns
///
/// * `Some(String)` - The WMI code (3 or 6 characters)
/// * `None` - If the VIN is invalid or too short (less than 15 characters)
///
/// # Examples
///
/// ```
/// use vin::wmi;
///
/// // Standard WMI (3 characters)
/// assert_eq!(wmi("1G1ND52F14M712344").unwrap(), "1G1");
///
/// // Extended WMI with '9' at position 3
/// // assert_eq!(wmi("939ABCDEFGH123456").unwrap(), "939GH1");
/// ```
pub fn wmi(vin: &str) -> Option<String> {
    let vin = vin_cleaner(vin).unwrap_or_default();

    // Validate VIN length before any character access
    if vin.len() < 15 {
        return None;
    }

    // Convert to Vec<char> once for efficient indexing
    let chars: Vec<char> = vin.chars().collect();

    // Safely check position 2 (3rd character)
    if chars.get(2) == Some(&'9') {
        // For WMI with '9' at position 2, combine positions 0-2 and 11-13
        if chars.len() >= 14 {
            Some(chars[0..3].iter().chain(&chars[11..14]).collect())
        } else {
            Some(chars[0..3].iter().collect())
        }
    } else {
        Some(chars[0..3].iter().collect())
    }
}

/// Extracts the manufacturer name from a VIN.
///
/// This function combines VIN cleaning, WMI extraction, and manufacturer lookup
/// to return the human-readable manufacturer name.
///
/// # Arguments
///
/// * `vin` - The VIN string (can be dirty/unformatted, will be cleaned)
///
/// # Returns
///
/// * `Some(String)` - The manufacturer name (e.g., "Chevrolet USA", "Toyota")
/// * `None` - If the VIN is invalid or the manufacturer is not found
///
/// # Lookup Strategy
///
/// The function attempts to match:
/// 1. First, tries the first 2 characters of the WMI
/// 2. Falls back to the full WMI if no match
///
/// # Examples
///
/// ```
/// use vin::vin_manuf;
///
/// assert_eq!(vin_manuf("1G1ND52F14M712344").unwrap(), "Chevrolet USA");
/// assert_eq!(vin_manuf("JTDKN3DU9F0421684").unwrap(), "Toyota");
/// assert_eq!(vin_manuf("WF05XXGCC5FJ00000").unwrap(), "Ford Germany");
///
/// // Handles dirty input
/// assert_eq!(vin_manuf("  1G1ND52F14M712344  ").unwrap(), "Chevrolet USA");
/// ```
pub fn vin_manuf(vin: &str) -> Option<String> {
    let vin = vin_cleaner(vin).unwrap_or_default();

    match wmi(&vin) {
        Some(w) if !w.is_empty() => {
            // Safely extract first 2 characters if available
            if w.len() >= 2 {
                // Try 2-character WMI first, then fall back to full WMI
                let chars: Vec<char> = w.chars().collect();
                let two_char_wmi: String = chars[0..2].iter().collect();

                WMI_MAP
                    .get(&two_char_wmi)
                    .or_else(|| WMI_MAP.get(&w))
                    .cloned()
            } else {
                WMI_MAP.get(&w).cloned()
            }
        }
        _ => None,
    }
}

/// Cleans and validates a VIN string.
///
/// This function extracts a valid VIN from potentially dirty input, handling:
/// - Leading/trailing whitespace
/// - Mixed case letters
/// - Extra characters before or after the VIN
///
/// # Arguments
///
/// * `vin` - The potentially dirty VIN string
///
/// # Returns
///
/// * `Some(String)` - A cleaned, uppercase VIN (16, 17, or 18 characters)
/// * `None` - If no valid VIN pattern is found
///
/// # Validation Patterns
///
/// The function tries multiple VIN patterns in order:
/// 1. Standard 17-character VIN: `[A-HJ-NPR-Z0-9]{17}` (excludes I, O, Q)
/// 2. 16-character VIN: `[A-HJ-NPR-Z0-9]{16}`
/// 3. 18-character VIN: `[A-HJ-NPR-Z0-9]{18}`
/// 4. Permissive 17-character VIN: `[A-Z0-9]{17}` (allows I, O, Q)
///
/// # Examples
///
/// ```
/// use vin::vin_cleaner;
///
/// // Clean VIN
/// assert_eq!(vin_cleaner("1G1ND52F14M712344").unwrap(), "1G1ND52F14M712344");
///
/// // Lowercase input
/// assert_eq!(vin_cleaner("1g1nd52f14m712344").unwrap(), "1G1ND52F14M712344");
///
/// // With extra text
/// assert_eq!(
///     vin_cleaner("VIN: 1G1ND52F14M712344 (new)").unwrap(),
///     "1G1ND52F14M712344"
/// );
///
/// // With whitespace
/// assert_eq!(
///     vin_cleaner("  1G1ND52F14M712344  ").unwrap(),
///     "1G1ND52F14M712344"
/// );
///
/// // Invalid input
/// assert!(vin_cleaner("123").is_none());
/// assert!(vin_cleaner("").is_none());
/// ```
pub fn vin_cleaner(vin: &str) -> Option<String> {
    if vin.is_empty() {
        return None;
    }

    let vin = vin.trim().to_uppercase();

    for re in VIN_REGEXES.iter() {
        if let Some(mat) = re.find(&vin) {
            return Some(mat.as_str().to_string());
        }
    }

    None
}

/// Determines the continent of origin from a VIN.
///
/// The first character of a VIN indicates the geographic region where
/// the vehicle was manufactured.
///
/// # Arguments
///
/// * `vin` - The VIN string (will be cleaned first)
///
/// # Returns
///
/// * `Some(&str)` - The continent name
/// * `None` - If the VIN is invalid or the first character is unrecognized
///
/// # Continent Mapping
///
/// | First Character | Continent       |
/// |----------------|-----------------|
/// | A-H            | Africa          |
/// | J-N, P, R      | Asia            |
/// | S-Z            | Europe          |
/// | 1-5            | North America   |
/// | 6-7            | Oceania         |
/// | 8-9            | South America   |
///
/// Note: The letter 'O' is not used in VINs to avoid confusion with zero.
///
/// # Examples
///
/// ```
/// use vin::vin_continent;
///
/// assert_eq!(vin_continent("1G1ND52F14M712344").unwrap(), "North America");
/// assert_eq!(vin_continent("WF05XXGCC5FJ00000").unwrap(), "Europe");
/// assert_eq!(vin_continent("JTDKN3DU9F0421684").unwrap(), "Asia");
/// assert_eq!(vin_continent("6G1ND52F14M712344").unwrap(), "Oceania");
/// assert_eq!(vin_continent("8G1ND52F14M712344").unwrap(), "South America");
/// ```
pub fn vin_continent(vin: &str) -> Option<&'static str> {
    let vin = vin_cleaner(vin).unwrap_or_default();

    // First character of VIN
    let x = vin.chars().next().unwrap_or_default();

    match x {
        'A'..='H' => Some("Africa"),
        'J'..='N' | 'P' | 'R' => Some("Asia"),
        'S'..='Z' => Some("Europe"),
        '1'..='5' => Some("North America"),
        '6' | '7' => Some("Oceania"),
        '8' | '9' => Some("South America"),
        _ => None,
    }
}

/// Extracts the model year from a VIN.
///
/// The model year is encoded at different positions depending on the region:
/// - Position 10 (character index 9): North America and most regions
/// - Position 11 (character index 10): Europe
///
/// The interpretation also depends on position 7 for North American VINs.
///
/// # Arguments
///
/// * `vin` - The VIN string (must be exactly 17 characters after cleaning)
///
/// # Returns
///
/// * `Some(String)` - The 4-digit model year
/// * `None` - If the VIN is invalid, not 17 characters, or year cannot be determined
///
/// # Year Encoding Rules
///
/// For North American VINs:
/// - If position 7 is ALPHABETIC: year range 2010-2039
/// - If position 7 is NUMERIC: year range 1980-2009
///
/// For European and other VINs:
/// - Year range defaults to 2010-2039
///
/// # Year Characters
///
/// Years are encoded using: `ABCDEFGHJKLMNPRSTUVWXYZ1234567890`
/// (excludes I, O, Q, U, Z, 0 in some positions)
///
/// # Future Year Handling
///
/// If the decoded year is more than 1 year in the future, 30 years are
/// subtracted to account for the cyclical nature of VIN year encoding.
///
/// # Examples
///
/// ```
/// use vin::vin_year;
///
/// // Standard North American VIN
/// assert_eq!(vin_year("1G1ND52F14M712344").unwrap(), "2004");
/// assert_eq!(vin_year("1FTEW1CM9BFA74557").unwrap(), "2011");
///
/// // VINs from different years
/// assert_eq!(vin_year("3C3CFFAR2CT212308").unwrap(), "2012");
/// assert_eq!(vin_year("5LMJJ3J57EEL08671").unwrap(), "2014");
///
/// // Invalid: not 17 characters
/// assert!(vin_year("123").is_none());
/// ```
pub fn vin_year(vin: &str) -> Option<String> {
    let year_chars = "ABCDEFGHJKLMNPRSTUVWXYZ1234567890".chars();
    let vin = vin_cleaner(vin).unwrap_or_default();

    // Validate VIN length (must be exactly 17 for year extraction)
    if vin.len() != 17 || vin.is_empty() {
        return None;
    }

    // Convert to Vec<char> for safe indexing
    let vin_chars: Vec<char> = vin.chars().collect();

    // This pos 7 check was introduced in US for NA autos and not valid for EU, Asia Cars
    let year_ch = year_chars.filter(|&c| c != 'U' && c != 'Z' && c != '0');

    let is_north_america = vin_continent(&vin).unwrap_or_default() == "North America";
    let is_eu = vin_continent(&vin).unwrap_or_default() == "Europe";

    // Define possible model year ranges based on VIN type
    let years = if is_north_america {
        // If position seven is ALPHABETIC, the model year in position 10
        // of VIN refers to a year in the range 2010–2039
        // If position seven is NUMERIC, the model year in position 10
        // of the VIN refers to a year in the range 1980–2009
        let pos6_char = vin_chars.get(6)?;
        if "ABCDEFGHJKLMNPRSTUVWXYZ".contains(*pos6_char) {
            2010..2040
        } else {
            1980..2010
        }
    } else {
        2010..2040
    };

    // Determine the character in VIN that represents the model year
    let year_model_char = if is_eu {
        *vin_chars.get(10)?
    } else {
        *vin_chars.get(9)?
    };

    let current_year = Local::now().year();

    for (c, y) in year_ch.zip(years) {
        if c == year_model_char {
            // Check for model years in the future
            return Some(if y > current_year + 1 {
                (y - 30).to_string()
            } else {
                y.to_string()
            });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test source: https://github.com/nighcoder/vin-parser/blob/master/test/vin_parser_test.py

    #[test]
    fn test_get_wmicsv() {
        let map = get_wmicsv();
        assert_eq!(map.get("1G1").unwrap(), "Chevrolet USA");
        assert!(!map.is_empty());
    }

    #[test]
    fn test_wmi() {
        assert_eq!(wmi("1G1ND52F14M712344").unwrap(), "1G1");
        assert_eq!(wmi("1G1ND52F14M712344").unwrap(), "1G1");
    }

    #[test]
    fn test_wmi_bad_input() {
        assert_eq!(wmi("1g1nd52f14m712344").unwrap(), "1G1");
        assert_eq!(wmi("   1G1ND52F14M712344").unwrap(), "1G1");

        assert!(wmi("123").is_none());
        assert!(wmi("").is_none());
    }

    #[test]
    fn test_manuf() {
        let vinl = [
            "1GKKRNED9EJ262581",
            "2A4GP54L16R805929",
            "JM1BL1M72C1587426",
            "1FTEW1CM9BFA74557",
            "1FAFP34P63W132895",
            "1J4GL48K05W616251",
            "3VWDX7AJ2BM339496",
            "5LMJJ3J57EEL08671",
            "WMWMF7C56ATZ69847",
            "JTDKN3DU9F0421684",
            "3C3CFFAR2CT212308",
            "KNMCSHLMS6P600875",
            "Y7A6135B660801530",
            "TRUZZZSNZY1063363",
        ];
        let resl = [
            "General Motors USA",
            "Chrysler Canada",
            "Mazda",
            "Ford Motor Company",
            "Ford Motor Company",
            "Jeep",
            "Volkswagen Mexico",
            "Lincoln",
            "MINI",
            "Toyota",
            "Chrysler Mexico",
            "Kia",
            "Kraz",
            "Audi Hungary",
        ];

        for (v, r) in vinl.iter().zip(resl.iter()) {
            assert_eq!(
                vin_manuf(v).unwrap_or_default(),
                *r,
                "vin_manuf({}) == {}",
                v,
                r
            );
        }
    }

    #[test]
    fn test_manuf_bad_input() {
        let vinl = [
            "1GKKRNED9EJ262581",
            " 1GKKRNED9EJ262581",
            " 1GKKRNED9EJ262581 ",
            " 1GKKRNED9EJ262581 (ok) ",
            "123456",
            "123",
            "",
            "abc",
            "...............",
        ];
        let resl = [
            "General Motors USA",
            "General Motors USA",
            "General Motors USA",
            "General Motors USA",
            "",
            "",
            "",
            "",
            "",
        ];

        for (v, r) in vinl.iter().zip(resl.iter()) {
            assert_eq!(
                vin_manuf(v).unwrap_or_default(),
                *r,
                "checking if vin_manuf({}) == {}",
                v,
                r
            );
        }
    }

    #[test]
    fn test_vin_year() {
        let vinl = [
            "1GKKRNED9EJ262581",
            "2A4GP54L16R805929",
            "JM1BL1M72C1587426",
            "1FTEW1CM9BFA74557",
            "1FAFP34P63W132895",
            "1J4GL48K05W616251",
            "3VWDX7AJ2BM339496",
            "5LMJJ3J57EEL08671",
            "JTDKN3DU9F0421684",
            "3C3CFFAR2CT212308",
            "KNMCSHLMS6P600875",
        ];
        let yearsl = [
            2014, 2006, 2012, 2011, 2003, 2005, 2011, 2014, 2015, 2012, 2006,
        ];

        for (v, r) in vinl.iter().zip(yearsl.iter()) {
            assert_eq!(
                vin_year(v).unwrap_or_default(),
                format!("{}", r),
                "checking if vin_year({}) == {}",
                v,
                r
            );
        }
    }

    #[test]
    fn test_vin_year_bad_input() {
        let vinl = [
            " 1GKKRNED9EJ262581",
            "2a4gp54l16r805929 ",
            "JM1BL1M72C1587426 (ok)",
            "... JM1BL1M72C1587426 (ok)",
            "...............",
            "123",
            "",
        ];
        let yearsl = ["2014", "2006", "2012", "2012", "", "", ""];

        for (v, r) in vinl.iter().zip(yearsl.iter()) {
            assert_eq!(
                vin_year(v).unwrap_or_default(),
                format!("{}", r),
                "checking if vin_year({}) == {}",
                v,
                r
            );
        }
    }

    #[test]
    fn test_vin_continent() {
        assert_eq!(vin_continent("1G1ND52F14M712344").unwrap(), "North America");
        assert_eq!(vin_continent("WF05XXGCC5FJ00000").unwrap(), "Europe");
        assert_eq!(vin_continent("JTFHX02PX00000000").unwrap(), "Asia");
    }

    #[test]
    fn test_vin_continent_bad_input() {
        assert_eq!(
            vin_continent(" 1G1ND52F14M712344 ").unwrap(),
            "North America"
        );
        assert_eq!(
            vin_continent("WF05XXGCC5FJ00000 (ahihi)").unwrap(),
            "Europe"
        );
        assert_eq!(vin_continent("JTFHX02PX0000000000 ... ").unwrap(), "Asia");
    }

    #[test]
    fn test_vin_cleaner() {
        assert_eq!(
            vin_cleaner("1G1ND52F14M712344").unwrap(),
            "1G1ND52F14M712344"
        );
        assert_eq!(
            vin_cleaner("1g1nd52f14m712344").unwrap(),
            "1G1ND52F14M712344"
        );
        assert_eq!(
            vin_cleaner("1G1ND52F14M700000").unwrap(),
            "1G1ND52F14M700000"
        );
        assert_eq!(
            vin_cleaner("1g1ND52F14m700000").unwrap(),
            "1G1ND52F14M700000"
        );

        assert_eq!(
            vin_cleaner("1G1ND52F14M712344000").unwrap(),
            "1G1ND52F14M712344"
        );
        assert_eq!(
            vin_cleaner("1g1nd52f14m712344000").unwrap(),
            "1G1ND52F14M712344"
        );
        assert_eq!(
            vin_cleaner("1G1ND52F14M700000000").unwrap(),
            "1G1ND52F14M700000"
        );
        assert_eq!(
            vin_cleaner("1g1ND52F14m700000000").unwrap(),
            "1G1ND52F14M700000"
        );
        assert_eq!(
            vin_cleaner("JTFHX02PX00032390 (ijdfhsdfs)").unwrap(),
            "JTFHX02PX00032390"
        );
        assert_eq!(
            vin_cleaner("JTFSX22P700000000NEW").unwrap(),
            "JTFSX22P700000000"
        );
        assert_eq!(
            vin_cleaner("JTFSX22P700000000 - NEW").unwrap(),
            "JTFSX22P700000000"
        );
        assert_eq!(
            vin_cleaner("JTFSX22P700000000.").unwrap(),
            "JTFSX22P700000000"
        );
        assert_eq!(
            vin_cleaner("JTFSX22P700000000-").unwrap(),
            "JTFSX22P700000000"
        );
        assert_eq!(
            vin_cleaner(":JTFSX22P700000000-").unwrap(),
            "JTFSX22P700000000"
        );
        assert_eq!(
            vin_cleaner("JTFSX22P700000000/RE").unwrap(),
            "JTFSX22P700000000"
        );
        assert_eq!(
            vin_cleaner("JTFSX22P700000000(MF)").unwrap(),
            "JTFSX22P700000000"
        );
    }

    #[test]
    fn test_vin_cleaner_bad_input() {
        assert!(vin_cleaner("123").is_none());
        assert!(vin_cleaner("INVALID-123").is_none());
        assert!(vin_cleaner("-").is_none());
        assert!(vin_cleaner("").is_none());
    }
}
