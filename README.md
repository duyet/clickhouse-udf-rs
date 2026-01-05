# ClickHouse UDF written in Rust

[![CI/CD](https://github.com/duyet/clickhouse-udf-rs/workflows/CI%2FCD/badge.svg)](https://github.com/duyet/clickhouse-udf-rs/actions)
[![Cargo](https://img.shields.io/crates/v/clickhouse-udf-rs)](https://crates.io/crates/clickhouse-udf-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.73%2B-orange.svg)](https://www.rust-lang.org/)

Collection of some useful UDFs for ClickHouse written in Rust.

Compile into binary

```bash
$ cargo build --release

$ ls -lhp target/release | grep -v '/\|\.d'
-rwxr-xr-x    1 duet  staff   434K Feb 24 21:26 read-wkt-linestring
-rwxr-xr-x    1 duet  staff   434K Feb 24 21:26 vin-cleaner
-rwxr-xr-x    1 duet  staff   434K Feb 24 21:26 vin-cleaner-chunk-header
-rwxr-xr-x    1 duet  staff   434K Feb 24 21:26 vin-manuf
-rwxr-xr-x    1 duet  staff   434K Feb 24 21:26 vin-manuf-chunk-header
-rwxr-xr-x    1 duet  staff   434K Feb 24 21:26 vin-year
-rwxr-xr-x    1 duet  staff   434K Feb 24 21:26 vin-year-chunk-header
-rwxr-xr-x    1 duet  staff   434K Feb 24 21:26 extract-url
-rwxr-xr-x    1 duet  staff   434K Feb 24 21:26 has-url
-rwxr-xr-x    1 duet  staff   434K Feb 24 21:26 array-topk
-rwxr-xr-x    1 duet  staff   434K Feb 24 21:26 tiktoken-count
-rwxr-xr-x    1 duet  staff   434K Feb 24 21:26 tiktoken-encode
-rwxr-xr-x    1 duet  staff   434K Feb 24 21:26 extract-phone
-rwxr-xr-x    1 duet  staff   434K Feb 24 21:26 string-format
-rwxr-xr-x    1 duet  staff   434K Feb 24 21:26 llm

```

## Quick Start

### 1. Build

```bash
cargo build --release
```

### 2. Install

```bash
# Download pre-built binaries (recommended for production)
cd /var/lib/clickhouse/user_scripts/
wget https://github.com/duyet/clickhouse-udf-rs/releases/latest/download/clickhouse_udf_v0.1.8_x86_64-unknown-linux-musl.tar.gz
tar zxvf clickhouse_udf_v0.1.8_x86_64-unknown-linux-musl.tar.gz
```

### 3. Configure

Create `/etc/clickhouse-server/custom_udf_function.xml`:

```xml
<functions>
    <function>
        <name>vinCleaner</name>
        <type>executable_pool</type>
        <command>vin-cleaner</command>
        <format>TabSeparated</format>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
    </function>
</functions>
```

### 4. Restart ClickHouse

```bash
sudo systemctl restart clickhouse-server
```

### 5. Use in SQL

```sql
SELECT vinCleaner('1G1JC1249Y7150000');
-- Output: 1G1JC1249Y7150000
```

## Available UDFs

1. [wkt](#1-wkt)
2. [vin](#2-vin)
3. [url](#3-url)
4. [array](#4-array)
5. [tiktoken](#5-tiktoken)
6. [string](#6-string)
7. [llm](#7-llm)


# Usage

## 1. `wkt`


<details>
  <summary>
    Put the <strong>wkt</strong> binaries into <code>user_scripts</code> folder (<code>/var/lib/clickhouse/user_scripts/</code> with default path settings).
  </summary>

  ```bash
  $ cd /var/lib/clickhouse/user_scripts/
  $ wget https://github.com/duyet/clickhouse-udf-rs/releases/download/0.1.8/clickhouse_udf_wkt_v0.1.8_x86_64-unknown-linux-musl.tar.gz
  $ tar zxvf clickhouse_udf_wkt_v0.1.8_x86_64-unknown-linux-musl.tar.gz

  read-wkt-linestring
  
  ```
</details>

<details>
  <summary>
    Creating UDF using XML configuration <code>custom_udf_wkt_function.xml</code>
  </summary>

  define udf config file `wkt_udf_function.xml` (`/etc/clickhouse-server/custom_udf_wkt_function.xml` with default path settings,
  file name must be matched `*_function.xml`).


  ```xml
  <functions>
    <!-- wkt -->
    <function>
        <name>readWktLineString</name>
        <type>executable_pool</type>
        <command>read-wkt-linestring</command>
        <format>TabSeparated</format>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
    </function>
    
  </functions>
  ```
</details>




<details>
  <summary>ClickHouse example queries</summary>

  ```sql
  SELECT readWktLineString("LINESTRING (30 10, 10 30, 40 40)")
  ```
</details>

## 2. `vin`


<details>
  <summary>
    Put the <strong>vin</strong> binaries into <code>user_scripts</code> folder (<code>/var/lib/clickhouse/user_scripts/</code> with default path settings).
  </summary>

  ```bash
  $ cd /var/lib/clickhouse/user_scripts/
  $ wget https://github.com/duyet/clickhouse-udf-rs/releases/download/0.1.8/clickhouse_udf_vin_v0.1.8_x86_64-unknown-linux-musl.tar.gz
  $ tar zxvf clickhouse_udf_vin_v0.1.8_x86_64-unknown-linux-musl.tar.gz

  vin-cleaner
  vin-cleaner-chunk-header
  vin-manuf
  vin-manuf-chunk-header
  vin-year
  vin-year-chunk-header
  
  ```
</details>

<details>
  <summary>
    Creating UDF using XML configuration <code>custom_udf_vin_function.xml</code>
  </summary>

  define udf config file `vin_udf_function.xml` (`/etc/clickhouse-server/custom_udf_vin_function.xml` with default path settings,
  file name must be matched `*_function.xml`).


  ```xml
  <functions>
    <!-- vin -->
    <function>
        <name>vinCleaner</name>
        <type>executable_pool</type>
        <command>vin-cleaner</command>
        <format>TabSeparated</format>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
    </function>
    <function>
        <name>vinManuf</name>
        <type>executable_pool</type>
        <command>vin-manuf</command>
        <format>TabSeparated</format>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
    </function>
    <function>
        <name>vinYear</name>
        <type>executable_pool</type>
        <command>vin-year</command>
        <format>TabSeparated</format>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
    </function>
    
  </functions>
  ```
</details>








<details>
  <summary>UDF config with <code>&lt;send_chunk_header&gt;1&lt;&#x2F;send_chunk_header&gt;</code></summary>

  ```xml
  <functions>
      <!-- vin -->
      
      <function>
          <name>vinCleaner</name>
          <type>executable_pool</type>

          <command>vin-cleaner-chunk-header</command>
          <send_chunk_header>1</send_chunk_header>

          <format>TabSeparated</format>
          <argument>
              <type>String</type>
              <name>value</name>
          </argument>
          <return_type>String</return_type>
      </function>
      
      <function>
          <name>vinManuf</name>
          <type>executable_pool</type>

          <command>vin-manuf-chunk-header</command>
          <send_chunk_header>1</send_chunk_header>

          <format>TabSeparated</format>
          <argument>
              <type>String</type>
              <name>value</name>
          </argument>
          <return_type>String</return_type>
      </function>
      
      <function>
          <name>vinYear</name>
          <type>executable_pool</type>

          <command>vin-year-chunk-header</command>
          <send_chunk_header>1</send_chunk_header>

          <format>TabSeparated</format>
          <argument>
              <type>String</type>
              <name>value</name>
          </argument>
          <return_type>String</return_type>
      </function>
      </functions>
  ```

</details>


<details>
  <summary>ClickHouse example queries</summary>

  ```sql
  SELECT vinCleaner("1G1JC1249Y7150000")
  SELECT vinCleaner("1G1JC1249Y7150000 ...")
  
  SELECT vinManuf("1G1JC1249Y7150000")
  
  SELECT vinYear("1G1JC1249Y7150000")
  ```
</details>

## 3. `url`


<details>
  <summary>
    Put the <strong>url</strong> binaries into <code>user_scripts</code> folder (<code>/var/lib/clickhouse/user_scripts/</code> with default path settings).
  </summary>

  ```bash
  $ cd /var/lib/clickhouse/user_scripts/
  $ wget https://github.com/duyet/clickhouse-udf-rs/releases/download/0.1.8/clickhouse_udf_url_v0.1.8_x86_64-unknown-linux-musl.tar.gz
  $ tar zxvf clickhouse_udf_url_v0.1.8_x86_64-unknown-linux-musl.tar.gz

  extract-url
  has-url
  
  ```
</details>

<details>
  <summary>
    Creating UDF using XML configuration <code>custom_udf_url_function.xml</code>
  </summary>

  define udf config file `url_udf_function.xml` (`/etc/clickhouse-server/custom_udf_url_function.xml` with default path settings,
  file name must be matched `*_function.xml`).


  ```xml
  <functions>
    <!-- url -->
    <function>
        <name>extractUrl</name>
        <type>executable_pool</type>
        <command>extract-url</command>
        <format>TabSeparated</format>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
    </function>
    <function>
        <name>hasUrl</name>
        <type>executable_pool</type>
        <command>has-url</command>
        <format>TabSeparated</format>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
    </function>
    
  </functions>
  ```
</details>





<details>
  <summary>ClickHouse example queries</summary>

  ```sql
  SELECT extractUrl("extract from this https://duyet.net")
  
  SELECT hasUrl("extract from this https://duyet.net")
  SELECT hasUrl("no url here")
  ```
</details>

## 4. `array`


<details>
  <summary>
    Put the <strong>array</strong> binaries into <code>user_scripts</code> folder (<code>/var/lib/clickhouse/user_scripts/</code> with default path settings).
  </summary>

  ```bash
  $ cd /var/lib/clickhouse/user_scripts/
  $ wget https://github.com/duyet/clickhouse-udf-rs/releases/download/0.1.8/clickhouse_udf_array_v0.1.8_x86_64-unknown-linux-musl.tar.gz
  $ tar zxvf clickhouse_udf_array_v0.1.8_x86_64-unknown-linux-musl.tar.gz

  array-topk
  
  ```
</details>

<details>
  <summary>
    Creating UDF using XML configuration <code>custom_udf_array_function.xml</code>
  </summary>

  define udf config file `array_udf_function.xml` (`/etc/clickhouse-server/custom_udf_array_function.xml` with default path settings,
  file name must be matched `*_function.xml`).


  ```xml
  <functions>
    <!-- array -->
    <function>
        <name>arrayTopK</name>
        <type>executable_pool</type>
        <command>array-topk</command>
        <format>TabSeparated</format>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
    </function>
    
  </functions>
  ```
</details>




<details>
  <summary>ClickHouse example queries</summary>

  ```sql
  SELECT arrayTopK(3)([1, 1, 2, 2, 3, 4, 5])
  SELECT arrayTopK(1)([2, 3, 4, 5])
  ```
</details>

## 5. `tiktoken`

Token counting and encoding for GPT models using tiktoken.

<details>
  <summary>
    Put the <strong>tiktoken</strong> binaries into <code>user_scripts</code> folder.
  </summary>

  ```bash
  $ cd /var/lib/clickhouse/user_scripts/
  $ wget https://github.com/duyet/clickhouse-udf-rs/releases/download/0.1.8/clickhouse_udf_tiktoken_v0.1.8_x86_64-unknown-linux-musl.tar.gz
  $ tar zxvf clickhouse_udf_tiktoken_v0.1.8_x86_64-unknown-linux-musl.tar.gz

  tiktoken-count
  tiktoken-encode
  ```
</details>

<details>
  <summary>
    Creating UDF using XML configuration <code>custom_udf_tiktoken_function.xml</code>
  </summary>

  ```xml
  <functions>
    <!-- tiktoken -->
    <function>
        <name>tiktokenCount</name>
        <type>executable_pool</type>
        <command>tiktoken-count</command>
        <format>TabSeparated</format>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
    </function>
    <function>
        <name>tiktokenEncode</name>
        <type>executable_pool</type>
        <command>tiktoken-encode</command>
        <format>TabSeparated</format>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
    </function>
  </functions>
  ```
</details>

<details>
  <summary>ClickHouse example queries</summary>

  ```sql
  -- Count tokens for GPT models
  SELECT tiktokenCount('Hello, world!')

  -- Encode text to token IDs
  SELECT tiktokenEncode('Hello, world!')
  ```
</details>

## 6. `string`

String processing utilities.

<details>
  <summary>
    Put the <strong>string</strong> binaries into <code>user_scripts</code> folder.
  </summary>

  ```bash
  $ cd /var/lib/clickhouse/user_scripts/
  $ wget https://github.com/duyet/clickhouse-udf-rs/releases/download/0.1.8/clickhouse_udf_string_v0.1.8_x86_64-unknown-linux-musl.tar.gz
  $ tar zxvf clickhouse_udf_string_v0.1.8_x86_64-unknown-linux-musl.tar.gz

  extract-phone
  string-format
  ```
</details>

<details>
  <summary>
    Creating UDF using XML configuration <code>custom_udf_string_function.xml</code>
  </summary>

  ```xml
  <functions>
    <!-- string -->
    <function>
        <name>extractPhone</name>
        <type>executable_pool</type>
        <command>extract-phone</command>
        <format>TabSeparated</format>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
    </function>
    <function>
        <name>stringFormat</name>
        <type>executable_pool</type>
        <command>string-format</command>
        <format>TabSeparated</format>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
    </function>
  </functions>
  ```
</details>

<details>
  <summary>ClickHouse example queries</summary>

  ```sql
  -- Extract phone numbers
  SELECT extractPhone('Call me at 555-123-4567 or 555.987.6543')

  -- Format strings (remove extra whitespace)
  SELECT stringFormat('Hello    world')
  ```
</details>

## 7. `llm`

Generic LLM function for any prompt-based task using OpenAI API.

<details>
  <summary>
    Put the <strong>llm</strong> binary into <code>user_scripts</code> folder.
  </summary>

  ```bash
  $ cd /var/lib/clickhouse/user_scripts/
  $ wget https://github.com/duyet/clickhouse-udf-rs/releases/latest/download/clickhouse_udf_llm_v0.1.8_x86_64-unknown-linux-musl.tar.gz
  $ tar zxvf clickhouse_udf_llm_v0.1.8_x86_64-unknown-linux-musl.tar.gz

  llm
  ```
</details>

<details>
  <summary>
    Creating UDF using XML configuration <code>custom_udf_llm_function.xml</code>
  </summary>

  ```xml
  <functions>
    <!-- llm -->
    <function>
        <name>llm</name>
        <type>executable_pool</type>
        <command>llm</command>
        <format>TabSeparated</format>
        <argument>
            <type>String</type>
            <name>prompt</name>
        </argument>
        <return_type>String</return_type>
        <environment>
            <!-- Choose ONE method for API key (tried in order): -->

            <!-- Method 1: Read from file (recommended for production) -->
            <OPENAI_API_KEY_FILE>/run/secrets/openai-key</OPENAI_API_KEY_FILE>

            <!-- Method 2: Direct environment variable -->
            <!-- <OPENAI_API_KEY>sk-proj-...</OPENAI_API_KEY> -->

            <!-- Method 3: Execute command to get key (for secret managers) -->
            <!-- <OPENAI_API_KEY_CMD>/usr/local/bin/vault kv get -field=value secret/openai</OPENAI_API_KEY_CMD> -->

            <!-- Other settings -->
            <OPENAI_MODEL>gpt-4o-mini</OPENAI_MODEL>
            <OPENAI_MAX_TOKENS>1000</OPENAI_MAX_TOKENS>
            <OPENAI_TEMPERATURE>0.7</OPENAI_TEMPERATURE>
        </environment>
    </function>
  </functions>
  ```
</details>

<details>
  <summary>ClickHouse example queries</summary>

  ```sql
  -- Simple summarization
  SELECT llm('Summarize this: {0}' || '\t' || article_content)
  FROM articles;

  -- Translation
  SELECT llm('Translate to Spanish: {0}' || '\t' || text)
  FROM messages;

  -- Sentiment analysis
  SELECT llm('Classify sentiment as positive/negative/neutral: {0}' || '\t' || review)
  FROM reviews;

  -- Multiple values
  SELECT llm('Compare {0} and {1}: {2}' || '\t' || product_a || '\t' || product_b || '\t' || criteria)
  FROM products;

  -- Text extraction
  SELECT llm('Extract email addresses from: {0}' || '\t' || text)
  FROM logs;
  ```
</details>

**Note**: The prompt template uses `{0}`, `{1}`, `{2}`... as placeholders. Values are passed tab-separated.

**Secret Configuration Options:**

1. **File-based** (Recommended): Mount secrets from Kubernetes/Docker secrets
   ```xml
   <OPENAI_API_KEY_FILE>/run/secrets/openai-key</OPENAI_API_KEY_FILE>
   ```

2. **Direct value**: For testing or simple setups
   ```xml
   <OPENAI_API_KEY>sk-proj-...</OPENAI_API_KEY>
   ```

3. **Command execution**: For secret managers (Vault, AWS Secrets Manager, etc.)
   ```xml
   <OPENAI_API_KEY_CMD>/usr/local/bin/get-secret openai</OPENAI_API_KEY_CMD>
   ```

The UDF tries each method in order and uses the first one that succeeds.

## Performance

UDF binaries are compiled with optimizations and are approximately 434KB each. Performance characteristics:

- **Startup time**: ~1-2ms per process
- **Throughput**: ~100K-500K rows/second (depending on function complexity)
- **Memory**: ~2-5MB per process
- **Concurrency**: ClickHouse automatically manages process pooling

For high-performance scenarios, use the `*-chunk-header` variants which enable batch processing:

```xml
<send_chunk_header>1</send_chunk_header>
```

This reduces per-row overhead by processing multiple rows in a single batch.

## Architecture

This project is organized as a Cargo workspace with the following structure:

- **shared**: Core I/O processing functions
- **vin**: Vehicle Identification Number (VIN) processing
- **wkt**: Well-Known Text (WKT) geometry parsing
- **url**: URL extraction and detection
- **array**: Array manipulation (top-k using FilteredSpaceSaving)
- **tiktoken**: GPT tokenization
- **string**: String processing utilities
- **llm**: Generic LLM function using OpenAI API

Each package compiles to standalone executables that can be used as ClickHouse UDFs.

See [CLAUDE.md](CLAUDE.md) for detailed architecture documentation.

## Development

```bash
# Clone the repository
git clone https://github.com/duyet/clickhouse-udf-rs.git
cd clickhouse-udf-rs

# Build
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench

# Format code
cargo fmt

# Run linter
cargo clippy
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed contribution guidelines.

## Future Ideas

Looking for more UDF ideas? Check out [docs/LLM_UDF_IDEAS.md](docs/LLM_UDF_IDEAS.md) for LLM-based UDF concepts including:
- Text embeddings for semantic search
- Sentiment analysis
- PII detection and redaction
- Named entity recognition
- And more...

# Generate README

```bash
RELEASE_VERSION=0.1.8 cargo run --bin readme-generator . > README.md
```

# License

MIT

Done
