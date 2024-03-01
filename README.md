# ClickHouse UDF written in Rust 

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

```

1. [wkt](#1-wkt)
2. [vin](#2-vin)
3. [url](#3-url)


# Usage

## 1. `wkt`


<details>
  <summary>
    Put the <strong>wkt</strong> binaries into <code>user_scripts</code> folder (<code>/var/lib/clickhouse/user_scripts/</code> with default path settings).
  </summary>

  ```bash
  $ cd /var/lib/clickhouse/user_scripts/
  $ wget https://github.com/duyet/clickhouse-udf-rs/releases/download/0.1.6/clickhouse_udf_wkt_v0.1.6_x86_64-unknown-linux-musl.tar.gz
  $ tar zxvf clickhouse_udf_wkt_v0.1.6_x86_64-unknown-linux-musl.tar.gz

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
        <name>readWktLinestring</name>
        <type>executable_pool</type>
        <command>read-wkt-linestring</command>
        <format>tabseparated</format>
        <argument>
            <type>string</type>
            <name>value</name>
        </argument>
        <return_type>string</return_type>
    </function>
  </functions>
  ```
</details>

<details>
  <summary>With <code>send_chunk_header=1</code></summary>

  ```xml
  <functions>
      <!-- wkt -->
      <function>
          <name>readWktLinestring</name>
          <type>executable_pool</type>

          <command>read-wkt-linestring-chunk-header</command>
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
  SELECT readWktLinestring('value');
  ```
</details>

## 2. `vin`


<details>
  <summary>
    Put the <strong>vin</strong> binaries into <code>user_scripts</code> folder (<code>/var/lib/clickhouse/user_scripts/</code> with default path settings).
  </summary>

  ```bash
  $ cd /var/lib/clickhouse/user_scripts/
  $ wget https://github.com/duyet/clickhouse-udf-rs/releases/download/0.1.6/clickhouse_udf_vin_v0.1.6_x86_64-unknown-linux-musl.tar.gz
  $ tar zxvf clickhouse_udf_vin_v0.1.6_x86_64-unknown-linux-musl.tar.gz

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
        <format>tabseparated</format>
        <argument>
            <type>string</type>
            <name>value</name>
        </argument>
        <return_type>string</return_type>
    </function><function>
        <name>vinCleanerChunkHeader</name>
        <type>executable_pool</type>
        <command>vin-cleaner-chunk-header</command>
        <format>tabseparated</format>
        <argument>
            <type>string</type>
            <name>value</name>
        </argument>
        <return_type>string</return_type>
    </function><function>
        <name>vinManuf</name>
        <type>executable_pool</type>
        <command>vin-manuf</command>
        <format>tabseparated</format>
        <argument>
            <type>string</type>
            <name>value</name>
        </argument>
        <return_type>string</return_type>
    </function><function>
        <name>vinManufChunkHeader</name>
        <type>executable_pool</type>
        <command>vin-manuf-chunk-header</command>
        <format>tabseparated</format>
        <argument>
            <type>string</type>
            <name>value</name>
        </argument>
        <return_type>string</return_type>
    </function><function>
        <name>vinYear</name>
        <type>executable_pool</type>
        <command>vin-year</command>
        <format>tabseparated</format>
        <argument>
            <type>string</type>
            <name>value</name>
        </argument>
        <return_type>string</return_type>
    </function><function>
        <name>vinYearChunkHeader</name>
        <type>executable_pool</type>
        <command>vin-year-chunk-header</command>
        <format>tabseparated</format>
        <argument>
            <type>string</type>
            <name>value</name>
        </argument>
        <return_type>string</return_type>
    </function>
  </functions>
  ```
</details>

<details>
  <summary>With <code>send_chunk_header=1</code></summary>

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
          <name>vinCleanerChunkHeader</name>
          <type>executable_pool</type>

          <command>vin-cleaner-chunk-header-chunk-header</command>
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
          <name>vinManufChunkHeader</name>
          <type>executable_pool</type>

          <command>vin-manuf-chunk-header-chunk-header</command>
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
      <function>
          <name>vinYearChunkHeader</name>
          <type>executable_pool</type>

          <command>vin-year-chunk-header-chunk-header</command>
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
  SELECT vinCleaner('value');
  SELECT vinCleanerChunkHeader('value');
  SELECT vinManuf('value');
  SELECT vinManufChunkHeader('value');
  SELECT vinYear('value');
  SELECT vinYearChunkHeader('value');
  ```
</details>

## 3. `url`


<details>
  <summary>
    Put the <strong>url</strong> binaries into <code>user_scripts</code> folder (<code>/var/lib/clickhouse/user_scripts/</code> with default path settings).
  </summary>

  ```bash
  $ cd /var/lib/clickhouse/user_scripts/
  $ wget https://github.com/duyet/clickhouse-udf-rs/releases/download/0.1.6/clickhouse_udf_url_v0.1.6_x86_64-unknown-linux-musl.tar.gz
  $ tar zxvf clickhouse_udf_url_v0.1.6_x86_64-unknown-linux-musl.tar.gz

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
        <format>tabseparated</format>
        <argument>
            <type>string</type>
            <name>value</name>
        </argument>
        <return_type>string</return_type>
    </function><function>
        <name>hasUrl</name>
        <type>executable_pool</type>
        <command>has-url</command>
        <format>tabseparated</format>
        <argument>
            <type>string</type>
            <name>value</name>
        </argument>
        <return_type>string</return_type>
    </function>
  </functions>
  ```
</details>

<details>
  <summary>With <code>send_chunk_header=1</code></summary>

  ```xml
  <functions>
      <!-- url -->
      <function>
          <name>extractUrl</name>
          <type>executable_pool</type>

          <command>extract-url-chunk-header</command>
          <send_chunk_header>1</send_chunk_header>

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

          <command>has-url-chunk-header</command>
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
  SELECT extractUrl('value');
  SELECT hasUrl('value');
  ```
</details>



# License

MIT

