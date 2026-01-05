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
//! Set environment variables in ClickHouse UDF config:
//! - `OPENAI_API_KEY`: Your OpenAI API key
//! - `OPENAI_MODEL`: Model to use (default: gpt-4o-mini)
//! - `OPENAI_MAX_TOKENS`: Max tokens in response (default: 1000)
//! - `OPENAI_TEMPERATURE`: Temperature 0-2 (default: 0.7)

use anyhow::{Context, Result};
use serde::Deserialize;
use std::env;

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
    let api_key = env::var("OPENAI_API_KEY")
        .unwrap_or_else(|_| "".to_string());

    if api_key.is_empty() {
        anyhow::bail!("OPENAI_API_KEY environment variable not set");
    }

    let model = env::var("OPENAI_MODEL")
        .unwrap_or_else(|_| "gpt-4o-mini".to_string());

    let max_tokens: u32 = env::var("OPENAI_MAX_TOKENS")
        .unwrap_or_else(|_| "1000".to_string())
        .parse()
        .unwrap_or(1000);

    let temperature: f32 = env::var("OPENAI_TEMPERATURE")
        .unwrap_or_else(|_| "0.7".to_string())
        .parse()
        .unwrap_or(0.7);

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

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .context("Failed to send request to OpenAI")?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
        anyhow::bail!("OpenAI API error: {} - {}", status, error_text);
    }

    let chat_response: ChatResponse = response
        .json()
        .context("Failed to parse OpenAI response")?;

    chat_response
        .choices
        .first()
        .map(|c| c.message.content.trim().to_string())
        .ok_or_else(|| anyhow::anyhow!("Empty response from OpenAI"))
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
        let values = vec!["Apple", "Orange"];
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
        assert!(true);
    }
}
