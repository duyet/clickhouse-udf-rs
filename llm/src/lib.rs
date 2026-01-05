//! Generic LLM UDF for ClickHouse
//!
//! Provides a simple `llm()` function that can handle any LLM use case
//! by using prompt templates with placeholders.
//!
//! # Usage
//!
//! ```sql
//! -- Simple prompt
//! SELECT llm('Summarize: {0}', article_text)
//!
//! -- Multiple values
//! SELECT llm('Compare {0} and {1}: {2}', product_a, product_b, criteria)
//!
//! -- Translation
//! SELECT llm('Translate to Spanish: {0}', 'Hello world')
//!
//! -- Sentiment analysis
//! SELECT llm('Classify sentiment as positive/negative/neutral: {0}', review)
//! ```
//!
//! # Configuration
//!
//! ## API Key Configuration (multiple methods, tried in order):
//!
//! ### Method 1: File (recommended for production)
//! ```xml
//! <environment>
//!     <OPENAI_API_KEY_FILE>/run/secrets/openai-key</OPENAI_API_KEY_FILE>
//!     <!-- Kubernetes/Docker secrets mount point -->
//! </environment>
//! ```
//!
//! ### Method 2: Environment Variable
//! ```xml
//! <environment>
//!     <OPENAI_API_KEY>sk-...</OPENAI_API_KEY>
//! </environment>
//! ```
//!
//! ### Method 3: External Command (for secret managers)
//! ```xml
//! <environment>
//!     <OPENAI_API_KEY_CMD>/usr/local/bin/get-secret openai</OPENAI_API_KEY_CMD>
//!     <!-- Runs command and uses stdout as the key -->
//! </environment>
//! ```
//!
//! ## Other Configuration:
//! - `OPENAI_MODEL`: Model to use (default: gpt-4o-mini)
//! - `OPENAI_MAX_TOKENS`: Max tokens in response (default: 1000)
//! - `OPENAI_TEMPERATURE`: Temperature 0-2 (default: 0.7)
//! - `OPENAI_API_BASE`: Custom API base URL (optional, for Azure/OpenAI-compatible)

use anyhow::{Context, Result};
use serde::Deserialize;
use std::env;
use std::fs;
use std::process::Command;

/// OpenAI API response structure
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
}

/// Generic LLM function that accepts a prompt template and values.
///
/// The template uses {0}, {1}, {2}, ... as placeholders for values.
///
/// # Arguments
///
/// * `input` - Tab-separated string: "template|value1|value2|..."
///
/// # Returns
///
/// * `Option<String>` - LLM response, or None if error occurs
///
/// # Examples
///
/// ```
/// use llm::llm;
///
/// // Single value
/// let result = llm("Summarize: {0}|\nThis is a long text...");
///
/// // Multiple values
/// let result = llm("Compare {0} and {1}|\tApple|\tOrange");
/// ```
pub fn llm(input: &str) -> Option<String> {
    // Parse input: template|value1|value2|...
    let parts: Vec<&str> = input.split('\t').collect();

    if parts.is_empty() {
        eprintln!("llm: empty input");
        return None;
    }

    let template = parts[0];
    let values: Vec<&str> = parts[1..].to_vec();

    // Build prompt by replacing placeholders
    let mut prompt = template.to_string();
    for (i, value) in values.iter().enumerate() {
        prompt = prompt.replace(&format!("{{{}}}", i), value);
    }

    // Call OpenAI API
    match call_openai(&prompt) {
        Ok(response) => Some(response),
        Err(e) => {
            eprintln!("llm error: {}", e);
            None
        }
    }
}

/// Call OpenAI Chat Completions API
fn call_openai(prompt: &str) -> Result<String> {
    // Try multiple methods to get API key (in order of preference)
    let api_key = get_api_key()?;

    let model = env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());

    let max_tokens: u32 = env::var("OPENAI_MAX_TOKENS")
        .unwrap_or_else(|_| "1000".to_string())
        .parse()
        .unwrap_or(1000);

    let temperature: f32 = env::var("OPENAI_TEMPERATURE")
        .unwrap_or_else(|_| "0.7".to_string())
        .parse()
        .unwrap_or(0.7);

    let api_base =
        env::var("OPENAI_API_BASE").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .context("Failed to build HTTP client")?;

    let payload = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "max_tokens": max_tokens,
        "temperature": temperature
    });

    let url = format!("{}/chat/completions", api_base);
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .with_context(|| format!("Failed to send request to {}", url))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .unwrap_or_else(|_| "Unknown error".to_string());
        anyhow::bail!("LLM API error: {} - {}", status, error_text);
    }

    let chat_response: ChatResponse = response.json().context("Failed to parse LLM response")?;

    chat_response
        .choices
        .first()
        .map(|c| c.message.content.trim().to_string())
        .ok_or_else(|| anyhow::anyhow!("Empty response from LLM"))
}

/// Get API key from multiple sources (tried in order):
/// 1. OPENAI_API_KEY_FILE - Read from file
/// 2. OPENAI_API_KEY - Direct environment variable
/// 3. OPENAI_API_KEY_CMD - Execute command and use stdout
fn get_api_key() -> Result<String> {
    // Method 1: Read from file (most secure for production)
    if let Ok(file_path) = env::var("OPENAI_API_KEY_FILE") {
        let key = fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read API key from file: {}", file_path))?;
        let key = key.trim();
        if !key.is_empty() {
            return Ok(key.to_string());
        }
    }

    // Method 2: Direct environment variable
    if let Ok(key) = env::var("OPENAI_API_KEY") {
        let key = key.trim();
        if !key.is_empty() {
            return Ok(key.to_string());
        }
    }

    // Method 3: Execute command to get key (for secret managers)
    if let Ok(cmd_str) = env::var("OPENAI_API_KEY_CMD") {
        let parts: Vec<&str> = cmd_str.split_whitespace().collect();
        if !parts.is_empty() {
            let output = Command::new(parts[0])
                .args(&parts[1..])
                .output()
                .with_context(|| format!("Failed to execute command: {}", cmd_str))?;

            if output.status.success() {
                let key = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !key.is_empty() {
                    return Ok(key);
                }
            }
        }
    }

    anyhow::bail!(
        "No API key found. Set one of:\n\
         - OPENAI_API_KEY_FILE=/path/to/key.txt\n\
         - OPENAI_API_KEY=sk-...\n\
         - OPENAI_API_KEY_CMD=/path/to/get-secret"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_replacement_single() {
        let input = "Summarize: {0}\tThis is text";
        let parts: Vec<&str> = input.split('\t').collect();
        assert_eq!(parts[0], "Summarize: {0}");
        assert_eq!(parts[1], "This is text");
    }

    #[test]
    fn test_template_replacement_multiple() {
        let template = "Compare {0} and {1}";
        let values = ["Apple", "Orange"];
        let mut prompt = template.to_string();
        for (i, value) in values.iter().enumerate() {
            prompt = prompt.replace(&format!("{{{}}}", i), value);
        }
        assert_eq!(prompt, "Compare Apple and Orange");
    }

    #[test]
    fn test_template_no_placeholders() {
        let input = "Just return hello";
        let parts: Vec<&str> = input.split('\t').collect();
        assert_eq!(parts[0], "Just return hello");
        assert!(parts.len() == 1);
    }

    #[test]
    #[ignore] // Requires actual API key
    fn test_call_openai_mock() {
        // This test would require mocking the OpenAI API
        // For now, we just verify the function compiles
    }

    #[test]
    fn test_get_api_key_priority() {
        // Test that environment variable has priority over unset file
        env::set_var("OPENAI_API_KEY", "test-key-from-env");
        env::remove_var("OPENAI_API_KEY_FILE");
        env::remove_var("OPENAI_API_KEY_CMD");

        let result = get_api_key();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-key-from-env");

        // Cleanup
        env::remove_var("OPENAI_API_KEY");
    }

    #[test]
    fn test_get_api_key_fails_when_none_set() {
        env::remove_var("OPENAI_API_KEY");
        env::remove_var("OPENAI_API_KEY_FILE");
        env::remove_var("OPENAI_API_KEY_CMD");

        let result = get_api_key();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No API key found"));
    }
}
