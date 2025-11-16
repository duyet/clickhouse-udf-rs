//! URL extraction and detection functions for ClickHouse.
//!
//! This crate provides utilities for detecting and extracting URLs from text strings.
//! It supports common URL protocols including HTTP, HTTPS, FTP, FTPS, and file:// URLs.
//!
//! # Examples
//!
//! ```ignore
//! use url::url::{extract_url, has_url};
//!
//! let text = "Check out https://example.com for more info";
//! let url = extract_url(text); // Some("https://example.com")
//! let has = has_url(text); // Some("true")
//! ```

pub mod url;
