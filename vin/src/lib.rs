pub mod vin;

// Re-export public functions for easier doctest and external access
pub use vin::{get_wmicsv, vin_cleaner, vin_continent, vin_manuf, vin_year, wmi};
