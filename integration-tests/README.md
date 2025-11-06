# ClickHouse UDF Integration Tests

This directory contains integration tests for ClickHouse User-Defined Functions (UDFs) written in Rust.

## Overview

The integration tests verify that all UDF binaries work correctly when deployed to a live ClickHouse instance. Tests run automatically on GitHub Actions for every push and pull request.

## Directory Structure

```
integration-tests/
├── config/           # XML UDF configuration files
│   ├── wkt_function.xml
│   ├── vin_function.xml
│   ├── url_function.xml
│   ├── array_function.xml
│   ├── string_function.xml
│   └── tiktoken_function.xml
├── sql/              # SQL test scripts
│   ├── test_wkt.sql
│   ├── test_vin.sql
│   ├── test_url.sql
│   ├── test_array.sql
│   ├── test_string.sql
│   └── test_tiktoken.sql
├── run-tests.sh      # Test runner script
└── README.md         # This file
```

## Running Tests Locally

### Prerequisites

1. **Docker** - To run ClickHouse container
2. **Rust toolchain** - To build UDF binaries
3. **ClickHouse client** - To execute SQL queries

### Steps

1. **Start ClickHouse with Docker:**

```bash
# Using custom ClickHouse image (recommended)
docker run -d --name clickhouse-test \
  -p 8123:8123 -p 9000:9000 \
  ghcr.io/duyet/docker-images:clickhouse_25.8

# Or use official ClickHouse image
docker run -d --name clickhouse-test \
  -p 8123:8123 -p 9000:9000 \
  clickhouse/clickhouse-server:latest
```

2. **Build UDF binaries:**

```bash
cargo build --release
```

3. **Deploy binaries to ClickHouse container:**

```bash
# Create user_scripts directory
docker exec clickhouse-test mkdir -p /var/lib/clickhouse/user_scripts/

# Copy binaries
for binary in target/release/{read-wkt-linestring,vin-*,extract-url,has-url,array-topk,extract-phone,tiktoken-*}; do
  [ -f "$binary" ] && docker cp "$binary" clickhouse-test:/var/lib/clickhouse/user_scripts/
done

# Make binaries executable
docker exec clickhouse-test chmod +x /var/lib/clickhouse/user_scripts/*
```

4. **Deploy XML configurations:**

```bash
# Copy XML configs
for config in integration-tests/config/*.xml; do
  docker cp "$config" clickhouse-test:/etc/clickhouse-server/
done

# Reload ClickHouse config
docker exec clickhouse-test clickhouse-client --query "SYSTEM RELOAD CONFIG"
```

5. **Run tests:**

```bash
cd integration-tests
./run-tests.sh
```

### Clean up

```bash
docker stop clickhouse-test
docker rm clickhouse-test
```

## CI/CD Integration

The integration tests run automatically on GitHub Actions via the `clickhouse-integration.yaml` workflow.

### ClickHouse Versions Tested

The CI pipeline tests against multiple ClickHouse versions using custom images:
- **ClickHouse 25.6** - `ghcr.io/duyet/docker-images:clickhouse_25.6`
- **ClickHouse 25.7** - `ghcr.io/duyet/docker-images:clickhouse_25.7`
- **ClickHouse 25.8** - `ghcr.io/duyet/docker-images:clickhouse_25.8` (latest)

### Workflow Steps

1. **Build**: Compile all UDF binaries in release mode
2. **Deploy**: Start ClickHouse container (matrix strategy for multiple versions)
3. **Install**: Deploy binaries to `/var/lib/clickhouse/user_scripts/`
4. **Configure**: Deploy XML configs and reload ClickHouse
5. **Test**: Run all SQL test scripts
6. **Report**: Display test results and logs on failure

### Viewing CI Results

- Check the "Actions" tab in GitHub repository
- Look for the "ClickHouse Integration Tests" job
- Review test output and ClickHouse logs if tests fail

## Writing New Tests

To add tests for a new UDF:

1. **Create XML configuration** in `config/`:
   ```xml
   <functions>
       <function>
           <name>myNewFunction</name>
           <type>executable_pool</type>
           <command>my-new-binary</command>
           <format>TabSeparated</format>
           <argument>
               <type>String</type>
               <name>value</name>
           </argument>
           <return_type>String</return_type>
       </function>
   </functions>
   ```

2. **Create SQL test file** in `sql/test_mynew.sql`:
   ```sql
   -- Test description
   SELECT 'Test 1: basic test' AS test_name, myNewFunction('input') AS result;
   -- Expected: output
   ```

3. **Update GitHub Actions workflow** to include the new binary in deployment step

4. **Run tests locally** to verify everything works

## Troubleshooting

### UDF not registered

- Check that binary was copied to `/var/lib/clickhouse/user_scripts/`
- Verify binary is executable (`chmod +x`)
- Check XML configuration syntax
- Reload ClickHouse config: `SYSTEM RELOAD CONFIG`

### Test failures

- Check ClickHouse logs: `docker logs clickhouse-test`
- Verify expected vs actual output in test results
- Test UDF binary directly: `echo "input" | ./my-binary`

### Permission issues

- Ensure ClickHouse container has execute permissions on binaries
- Check that ClickHouse user can access `/var/lib/clickhouse/user_scripts/`

## References

- [ClickHouse UDF Documentation](https://clickhouse.com/docs/en/sql-reference/functions/udf)
- [Project README](../README.md)
- [CLAUDE.md](../CLAUDE.md) - Project guidelines
