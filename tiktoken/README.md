# Tiktoken UDF for ClickHouse

ClickHouse User-Defined Functions (UDFs) for GPT tokenization using [tiktoken](https://github.com/openai/tiktoken).

## Overview

This package provides UDFs for working with GPT tokenization in ClickHouse. It uses the `cl100k_base` encoding, which is used by:
- GPT-3.5-turbo
- GPT-4
- text-embedding-ada-002

## Functions

### `tiktokenCount`

Count the number of tokens in a text string.

**Usage:**
```sql
SELECT tiktokenCount('Hello, world!');
-- Returns: 4

SELECT tiktokenCount('The quick brown fox jumps over the lazy dog');
-- Returns: 10
```

**Use cases:**
- Calculate API costs before making GPT requests
- Validate text fits within model token limits
- Monitor token usage in analytics

### `tiktokenEncode`

Encode text into a comma-separated list of token IDs.

**Usage:**
```sql
SELECT tiktokenEncode('Hello');
-- Returns: 9906

SELECT tiktokenEncode('Hello, world!');
-- Returns: 9906,11,1917,0
```

**Use cases:**
- Debug tokenization behavior
- Analyze how text is tokenized
- Create custom token-based text processing

## Installation

### 1. Build the binaries

```bash
# Build in release mode
cargo build --release -p tiktoken

# The binaries will be in target/release/:
# - tiktoken-count
# - tiktoken-encode
```

### 2. Copy to ClickHouse

```bash
# Copy binaries to ClickHouse user scripts directory
sudo cp target/release/tiktoken-count /var/lib/clickhouse/user_scripts/
sudo cp target/release/tiktoken-encode /var/lib/clickhouse/user_scripts/

# Make them executable
sudo chmod +x /var/lib/clickhouse/user_scripts/tiktoken-*
```

### 3. Configure ClickHouse

Create `/etc/clickhouse-server/tiktoken_function.xml`:

```xml
<clickhouse>
    <function>
        <type>executable</type>
        <name>tiktokenCount</name>
        <return_type>String</return_type>
        <argument>
            <type>String</type>
        </argument>
        <format>TabSeparated</format>
        <command>tiktoken-count</command>
    </function>

    <function>
        <type>executable</type>
        <name>tiktokenEncode</name>
        <return_type>String</return_type>
        <argument>
            <type>String</type>
        </argument>
        <format>TabSeparated</format>
        <command>tiktoken-encode</command>
    </function>
</clickhouse>
```

### 4. Restart ClickHouse

```bash
sudo systemctl restart clickhouse-server
```

## Examples

### Calculate token costs

```sql
-- Assuming GPT-4 pricing: $0.03 per 1K tokens (input)
SELECT
    text,
    tiktokenCount(text) AS tokens,
    (toFloat64(tokens) / 1000) * 0.03 AS cost_usd
FROM my_prompts_table
ORDER BY cost_usd DESC;
```

### Find texts exceeding token limits

```sql
-- Find texts that exceed GPT-3.5-turbo's 4096 token limit
SELECT
    id,
    text,
    tiktokenCount(text) AS tokens
FROM documents
WHERE toInt32(tokens) > 4096;
```

### Batch analysis

```sql
-- Analyze token distribution
SELECT
    count() AS count,
    avg(toFloat64(tokens)) AS avg_tokens,
    min(toInt32(tokens)) AS min_tokens,
    max(toInt32(tokens)) AS max_tokens,
    sum(toFloat64(tokens)) AS total_tokens
FROM (
    SELECT tiktokenCount(content) AS tokens
    FROM articles
);
```

### Debug tokenization

```sql
-- See how a specific text is tokenized
SELECT
    'Hello, world!' AS text,
    tiktokenCount('Hello, world!') AS token_count,
    tiktokenEncode('Hello, world!') AS token_ids;
```

## Technical Details

### Encoding

This package uses the `cl100k_base` encoding from tiktoken-rs. This is the same encoding used by:
- `gpt-3.5-turbo` models
- `gpt-4` models
- `text-embedding-ada-002`

### Performance

The tokenizer is initialized once and reused across function calls within the same process. Token counting is fast and suitable for high-throughput scenarios.

### Limitations

- Currently only supports `cl100k_base` encoding
- Returns `NULL` if tokenization fails
- Token IDs are returned as comma-separated strings (not arrays)

## Testing

```bash
# Run unit tests
cargo test -p tiktoken

# Test manually
echo "Hello, world!" | cargo run --release --bin tiktoken-count
echo "Hello, world!" | cargo run --release --bin tiktoken-encode
```

## See Also

- [OpenAI Tiktoken](https://github.com/openai/tiktoken)
- [tiktoken-rs](https://github.com/zurawiki/tiktoken-rs)
- [ClickHouse Executable UDFs](https://clickhouse.com/docs/en/sql-reference/functions/udf)
