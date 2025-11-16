//! Well-Known Text (WKT) geometry parsing for ClickHouse.
//!
//! This crate provides utilities for parsing WKT geometry format into
//! structured coordinate arrays. Currently supports LINESTRING geometry.
//!
//! # Examples
//!
//! ```ignore
//! use wkt::parse_wkt::parse_wkt;
//!
//! let wkt = "LINESTRING(0 0, 1 1, 2 2)";
//! let coords = parse_wkt(wkt); // "[(0,0),(1,1),(2,2)]"
//! ```

pub mod parse_wkt;
