# ClickHouse UDF written in Rust 

Collection of some useful UDFs for ClickHouse written in Rust.

Compile into binary

```bash
$ cargo build --release

$ ls -lhp target/release | grep -v '/\|\.d'
{% for project in projects -%}{% for bin in project.bins -%}
-rwxr-xr-x    1 duet  staff   434K Feb 24 21:26 {{ bin}}
{% endfor %}{% endfor %}
```

{% for project in projects -%}
{{ loop.index }}. [{{ project.name }}](#{{ loop.index }}-{{ project.name | slugify }})
{% endfor %}

# Usage

{% for project in projects -%}

## {{ loop.index }}. `{{ project.name }}`


<details>
  <summary>
    Put the <strong>{{ project.name }}</strong> binaries into <code>user_scripts</code> folder (<code>/var/lib/clickhouse/user_scripts/</code> with default path settings).
  </summary>

  ```bash
  $ cd /var/lib/clickhouse/user_scripts/
  $ wget https://github.com/duyet/clickhouse-udf-rs/releases/download/{{ version }}/clickhouse_udf_{{ project.name }}_v{{ version }}_x86_64-unknown-linux-musl.tar.gz
  $ tar zxvf clickhouse_udf_{{ project.name }}_v{{ version }}_x86_64-unknown-linux-musl.tar.gz

  {% for bin in project.bins -%}
  {{ bin }}
  {% endfor %}
  ```
</details>

<details>
  <summary>
    Creating UDF using XML configuration <code>custom_udf_{{ project.name }}_function.xml</code>
  </summary>

  define udf config file `{{ project.name }}_udf_function.xml` (`/etc/clickhouse-server/custom_udf_{{ project.name }}_function.xml` with default path settings,
  file name must be matched `*_function.xml`).


  ```xml
  <functions>
    <!-- {{ project.name }} -->
    {% for bin in project.bins -%}
    {% if bin is ending_with("-chunk-header") %}{% continue %}{% endif -%}
    <function>
        <name>{{ bin | to_clickhouse_function }}</name>
        <type>executable_pool</type>
        <command>{{ bin }}</command>
        <format>tabseparated</format>
        <argument>
            <type>string</type>
            <name>value</name>
        </argument>
        <return_type>string</return_type>
    </function>
    {% endfor %}
  </functions>
  ```
</details>

<details>
  <summary>UDF config with <code>{{ "<send_chunk_header>1</send_chunk_header>" | escape }}</code></summary>

  ```xml
  <functions>
      <!-- {{ project.name }} -->
      {% for bin in project.bins -%}
      {% if bin is not ending_with("-chunk-header") %}{% continue %}{% endif %}
      <function>
          <name>{{ bin | trim_end_matches(pat="-chunk-header") | to_clickhouse_function }}</name>
          <type>executable_pool</type>

          <command>{{ bin }}</command>
          <send_chunk_header>1</send_chunk_header>

          <format>TabSeparated</format>
          <argument>
              <type>String</type>
              <name>value</name>
          </argument>
          <return_type>String</return_type>
      </function>
      {% endfor -%}
  </functions>
  ```

</details>

<details>
  <summary>ClickHouse example queries</summary>

  ```sql
  {% for bin in project.bins -%}
  {% if bin is ending_with("-chunk-header") %}{% continue %}{% endif -%}
  SELECT {{ bin | to_clickhouse_function }}('value');
  {% endfor -%}
  ```
</details>

{% endfor %}

# License

MIT
