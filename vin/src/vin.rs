use chrono::{Datelike, Local};
use csv::ReaderBuilder;
use regex::Regex;
use std::collections::HashMap;

static WMI_DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "src/wmi.csv"));

pub fn get_wmicsv() -> HashMap<String, String> {
    let mut map = HashMap::new();

    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(WMI_DATA.as_bytes());

    for record in reader.records().flatten() {
        let wmi = record[0].trim().to_string();
        let manuf = record[1].trim().to_string();
        map.insert(wmi, manuf);
    }

    map
}

pub fn wmi(vin: &str) -> Option<String> {
    let vin = vin_cleaner(vin).unwrap_or_default();

    if vin.len() < 3 {
        return None;
    }

    if vin.chars().nth(2) == Some('9') {
        Some(
            vin.chars()
                .take(3)
                .chain(vin.chars().skip(11).take(3))
                .collect(),
        )
    } else {
        Some(vin.chars().take(3).collect())
    }
}

pub fn vin_manuf(vin: &str) -> Option<String> {
    let vin = vin_cleaner(vin).unwrap_or_default();
    let manfs = get_wmicsv();
    let w = wmi(&vin)?;

    if w.is_empty() {
        return None;
    }

    manfs
        .get(&w[..2].to_string())
        .or_else(|| manfs.get(&w.to_string()))
        .cloned()
}

pub fn vin_cleaner(vin: &str) -> Option<String> {
    if vin.is_empty() {
        return None;
    }

    let vin = vin.trim().to_uppercase();

    let vin_patterns = [
        // Standard VIN pattern (17 characters)
        r"[A-HJ-NPR-Z0-9]{17}",
        // 16 and 18 character patterns
        r"[A-HJ-NPR-Z0-9]{16}",
        r"[A-HJ-NPR-Z0-9]{18}",
        // Non-standard pattern (17 characters, allowing all letters)
        r"[A-Z0-9]{17}",
    ];

    for pattern in vin_patterns.iter() {
        let re = Regex::new(pattern).ok()?;
        if let Some(mat) = re.find(&vin) {
            return Some(mat.as_str().to_uppercase());
        }
    }

    Some(vin.to_string())
}

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

pub fn vin_year(vin: &str) -> Option<String> {
    let chars = "ABCDEFGHJKLMNPRSTUVWXYZ1234567890".chars();
    let vin = vin_cleaner(vin).unwrap_or_default();

    if vin.len() < 11 || vin.is_empty() {
        return None;
    }

    // This pos 7 check was introduced in US for NA autos and not valid for EU, Asia Cars
    let year_ch = chars.filter(|&c| c != 'U' && c != 'Z' && c != '0');

    let is_north_america = vin_continent(&vin).unwrap_or_default() == "North America";
    let is_eu = vin_continent(&vin).unwrap_or_default() == "Europe";

    // Define possible model year ranges based on VIN type
    let years = if is_north_america {
        // If position seven is ALPHABETIC, the model year in position 10
        // of VIN refers to a year in the range 2010–2039
        // If position seven is NUMERIC, the model year in position 10
        // of the VIN refers to a year in the range 1980–2009
        if "ABCDEFGHJKLMNPRSTUVWXYZ".contains(vin.chars().nth(6).unwrap()) {
            2010..2040
        } else {
            1980..2010
        }
    } else {
        2010..2040
    };

    // Determine the character in VIN that represents the model year
    let year_model_char = if is_eu {
        vin.chars().nth(10).unwrap()
    } else {
        vin.chars().nth(9).unwrap()
    };

    let current_year = Local::now().year();

    if vin.len() == 17 {
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

        assert_eq!(wmi("123").unwrap(), "123");
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
    fn test_vin_cleaner_17() {
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
    }

    #[test]
    fn test_vin_cleaner_bad_input() {
        assert_eq!(
            vin_cleaner("JTFHX02PX00032390 (ijdfhsdfs)").unwrap(),
            "JTFHX02PX00032390"
        );
        assert_eq!(vin_cleaner("123").unwrap(), "123");
        assert!(vin_cleaner("").is_none());
    }
}
