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
-rwxr-xr-x    1 duet  staff   434K Feb 24 21:26 array-topk

```

1. [wkt](#1-wkt)
2. [vin](#2-vin)
3. [url](#3-url)
4. [array](#4-array)


# Usage

## 1. `wkt`


<details>
  <summary>
    Put the <strong>wkt</strong> binaries into <code>user_scripts</code> folder (<code>/var/lib/clickhouse/user_scripts/</code> with default path settings).
  </summary>

  ```bash
  $ cd /var/lib/clickhouse/user_scripts/
  $ wget https://github.com/duyet/clickhouse-udf-rs/releases/download/<version>/clickhouse_udf_wkt_v<version>_x86_64-unknown-linux-musl.tar.gz
  $ tar zxvf clickhouse_udf_wkt_v<version>_x86_64-unknown-linux-musl.tar.gz

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
  $ wget https://github.com/duyet/clickhouse-udf-rs/releases/download/<version>/clickhouse_udf_vin_v<version>_x86_64-unknown-linux-musl.tar.gz
  $ tar zxvf clickhouse_udf_vin_v<version>_x86_64-unknown-linux-musl.tar.gz

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
  $ wget https://github.com/duyet/clickhouse-udf-rs/releases/download/<version>/clickhouse_udf_url_v<version>_x86_64-unknown-linux-musl.tar.gz
  $ tar zxvf clickhouse_udf_url_v<version>_x86_64-unknown-linux-musl.tar.gz

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
  $ wget https://github.com/duyet/clickhouse-udf-rs/releases/download/<version>/clickhouse_udf_array_v<version>_x86_64-unknown-linux-musl.tar.gz
  $ tar zxvf clickhouse_udf_array_v<version>_x86_64-unknown-linux-musl.tar.gz

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



# License

MIT

Done
