//! GPT tokenization functions for ClickHouse using tiktoken.
//!
//! This crate provides utilities for encoding and counting tokens using OpenAI's
//! tiktoken library. It uses the cl100k_base encoding, which is used by:
//! - GPT-3.5-turbo
//! - GPT-4
//! - text-embedding-ada-002
//!
//! # Examples
//!
//! ```ignore
//! use tiktoken::tiktoken::{tiktoken_count, tiktoken_encode};
//!
//! let text = "Hello, world!";
//! let count = tiktoken_count(text); // Number of tokens as string
//! let tokens = tiktoken_encode(text); // Comma-separated token IDs
//! ```

pub mod tiktoken;
