//! Vehicle Identification Number (VIN) processing functions for ClickHouse.
//!
//! This crate provides utilities for parsing, validating, and extracting information
//! from Vehicle Identification Numbers (VINs). It includes functions to:
//!
//! - Clean and normalize VIN strings
//! - Extract manufacturer information from World Manufacturer Identifier (WMI)
//! - Determine vehicle model year
//! - Identify continent of manufacture
//!
//! # Examples
//!
//! ```ignore
//! use vin::vin::{vin_cleaner, vin_manuf, vin_year};
//!
//! let vin = "1G1JC1249Y7150000";
//! let cleaned = vin_cleaner(vin);
//! let manufacturer = vin_manuf(vin);
//! let year = vin_year(vin);
//! ```

pub mod vin;
