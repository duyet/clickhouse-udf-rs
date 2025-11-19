# ClickHouse UDF written in Rust

[![Build Status](https://github.com/duyet/clickhouse-udf-rs/workflows/build-test/badge.svg)](https://github.com/duyet/clickhouse-udf-rs/actions)
[![Clippy](https://github.com/duyet/clickhouse-udf-rs/workflows/cargo-clippy/badge.svg)](https://github.com/duyet/clickhouse-udf-rs/actions)
[![rustfmt](https://github.com/duyet/clickhouse-udf-rs/workflows/cargo-fmt/badge.svg)](https://github.com/duyet/clickhouse-udf-rs/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Collection of high-performance UDFs (User-Defined Functions) for ClickHouse written in Rust.

## Features

- 🚀 **High Performance** - Compiled Rust binaries for maximum speed
- 🔒 **Type Safe** - Rust's type system prevents common errors
- 📦 **Multiple UDF Packages** - VIN parsing, URL extraction, array operations, and more
- 🧪 **Well Tested** - Comprehensive test coverage
- 📚 **Well Documented** - Inline documentation and examples

## Quick Start

```bash
# Build all UDFs in release mode
cargo build --release

# List built binaries
ls -lhp target/release | grep -v '/\|\.d'
```

## Available UDFs

1. [wkt](#1-wkt) - Well-Known Text geometry parsing
2. [vin](#2-vin) - Vehicle Identification Number processing  
3. [url](#3-url) - URL extraction and detection
4. [array](#4-array) - Array manipulation (top-k)
5. [string](#5-string) - String processing
6. [tiktoken](#6-tiktoken) - GPT tokenization

## Usage

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



# Generate README

```bash
RELEASE_VERSION=0.1.8 cargo run --bin readme-generator . > README.md
```

# License

MIT

Done
