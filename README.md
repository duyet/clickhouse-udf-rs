# ClickHouse UDF written in Rust 

Collection of some useful UDFs for ClickHouse written in Rust.

Compile into binary

```bash
cargo build --release

lls -lhp target/release | grep -v '/\|\.d'
-rwxr-xr-x    1 duet  staff   434K Feb 23 18:28 extract-url
-rwxr-xr-x    1 duet  staff   434K Feb 23 18:28 has-url
-rwxr-xr-x    1 duet  staff   515K Feb 22 23:57 read-wkt-linestring
-rwxr-xr-x    1 duet  staff   2.3M Feb 22 23:25 vin-cleaner
-rwxr-xr-x    1 duet  staff   2.3M Feb 22 23:25 vin-manuf
-rwxr-xr-x    1 duet  staff   2.3M Feb 22 23:25 vin-year
```

### 1. Put the binaries into `user_scripts` folder

Binary file inside `user_scripts` folder (`/var/lib/clickhouse/user_scripts/` with default path settings).

```bash
cp target/release/vin* /var/lib/clickhouse/user_scripts/
cp target/release/*wkt* /var/lib/clickhouse/user_scripts/
cp target/release/*url* /var/lib/clickhouse/user_scripts/
```

### 2. Creating UDF using XML configuration

File `custom_udf_function.xml` (`/etc/clickhouse-server/custom_udf_function.xml` with default path settings,
file name must be matched `*_function.xml`).


```xml
<functions>
    <!-- WKT -->
    <function>
        <name>readWktLineString</name>
        <type>executable</type>
        <return_type>Array(Point)</return_type>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <format>TabSeparated</format>
        <command>read-wkt-linestring</command>
    </function>

    <!-- VIN -->
    <function>
        <name>vinCleaner</name>
        <type>executable</type>
        <command>vin-cleaner</command>
        <format>TabSeparated</format>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
    </function>
    <function>
        <name>vinYear</name>
        <type>executable</type>
        <command>vin-year</command>
        <format>TabSeparated</format>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
    </function>
    <function>
        <name>vinManuf</name>
        <type>executable</type>
        <format>TabSeparated</format>
        <command>vin-manuf</command>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
    </function>

    <!-- URL -->
    <function>
        <name>extractUrl</name>
        <type>executable</type>
        <format>TabSeparated</format>
        <command>extract-url</command>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
    </function>
    <function>
        <name>hasUrl</name>
        <type>executable</type>
        <format>TabSeparated</format>
        <command>has-url</command>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>Boolean</return_type>
    </function>
</functions>
```

### 3. Query

```sql
SELECT
    readWktLineString('LINESTRING(1 1, 2 2)') AS ls,
    toTypeName(ls)

Query id: acf46ce7-c782-469c-9f13-dc4adffa69cc

┌─ls────────────┬─toTypeName(readWktLineString('LINESTRING(1 1, 2 2)'))─┐
│ [(1,1),(2,2)] │ Array(Point)                                          │
└───────────────┴───────────────────────────────────────────────────────┘
```


```sql
SELECT vinCleaner('AHTEB6CB802500000 (abc)');

┌─vinCleaner('AHTEB6CB802500000 (abc)')─┐
│ AHTEB6CB802500000                     │
└───────────────────────────────────────┘

SELECT
    vinYear('1GKKRNED9EJ260000') AS year,
    vinManuf('1GKKRNED9EJ260000') AS manuf

Query id: 429dfa20-f658-4c98-ba1d-f0b7fc2648a8

┌─year─┬─manuf──────────────┐
│ 2014 │ General Motors USA │
└──────┴────────────────────┘
```

```sql
SELECT
    extractUrl('abc https://duyet.net def') AS u,
    hasUrl('is this contains url https://duyet.net ?')

Query id: 5840589f-2f8f-4213-ab44-dba8bdb46a29

┌─u─────────────────┬─hasUrl('is this contains url https://duyet.net ?')─┐
│ https://duyet.net │ true                                               │
└───────────────────┴────────────────────────────────────────────────────┘
```
