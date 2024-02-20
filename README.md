# ClickHouse UDF written in Rust 

Collection of some useful UDFs for ClickHouse written in Rust.

Compile into binary

```bash
cargo build --release

lls -lhp target/release | grep -v '/\|\.d'
-rwxr-xr-x   1 duet  staff   515K Feb 22 23:25 read-wkt-linestring
-rwxr-xr-x   1 duet  staff   2.3M Feb 22 23:25 vin-cleaner
-rwxr-xr-x   1 duet  staff   2.3M Feb 22 23:25 vin-manuf
-rwxr-xr-x   1 duet  staff   2.3M Feb 22 23:25 vin-year
```

### 1. Put the binaries into `user_scripts` folder

Binary file inside `user_scripts` folder (`/var/lib/clickhouse/user_scripts/` with default path settings).

```bash
cp target/release/vin* /var/lib/clickhouse/user_scripts/
cp target/release/read* /var/lib/clickhouse/user_scripts/
```

### 2. Creating UDF using XML configuration

File `custom_udf_function.xml` (`/etc/clickhouse-server/custom_udf_function.xml` with default path settings,
file name must be matched `*_function.xml`).


```xml
<functions>
    <function>
        <type>executable</type>
        <name>readWktLineString</name>
        <command>read-wkt-linestring</command>
        <return_type>Array(Point)</return_type>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <format>TabSeparated</format>
    </function>
    <function>
        <type>executable</type>
        <name>vin_cleaner</name>
        <command>vin-cleaner</command>
        <format>TabSeparated</format>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
    </function>
    <function>
        <type>executable</type>
        <name>vin_year</name>
        <command>vin-year</command>
        <format>TabSeparated</format>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
    </function>
    <function>
        <type>executable</type>
        <name>vin_manuf</name>
        <format>TabSeparated</format>
        <command>vin-manuf</command>
        <argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <return_type>String</return_type>
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
SELECT vin_cleaner('AHTEB6CB802500000 (abc)');

┌─vin_cleaner('AHTEB6CB802500000 (abc)')─┐
│ AHTEB6CB802500000                      │
└────────────────────────────────────────┘

SELECT
    vin_year('1GKKRNED9EJ262581') AS year,
    vin_manuf('1GKKRNED9EJ262581') AS manuf

Query id: a66251a3-7daa-4b03-b56c-a140753c4111

┌─year─┬─manuf──────────────┐
│ 2014 │ General Motors USA │
└──────┴────────────────────┘
```
