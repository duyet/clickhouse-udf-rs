#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
CLICKHOUSE_CONTAINER="${CLICKHOUSE_CONTAINER:-clickhouse-test}"
CLICKHOUSE_CLIENT="${CLICKHOUSE_CLIENT:-docker exec $CLICKHOUSE_CONTAINER clickhouse-client}"
TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SUMMARY_FILE="${SUMMARY_FILE:-$TEST_DIR/test-summary.md}"
GENERATE_EXPECTED="${GENERATE_EXPECTED:-false}"

# Check if --generate-expected flag is passed
for arg in "$@"; do
    if [ "$arg" == "--generate-expected" ]; then
        GENERATE_EXPECTED=true
        break
    fi
done

echo "================================================"
echo "ClickHouse UDF Integration Tests"
echo "================================================"
echo "Container: $CLICKHOUSE_CONTAINER"
echo "Test Directory: $TEST_DIR"
echo ""

# Wait for ClickHouse to be ready
echo -n "Waiting for ClickHouse to be ready..."
max_attempts=30
attempt=0
until $CLICKHOUSE_CLIENT --query="SELECT 1" > /dev/null 2>&1; do
    if [ $attempt -eq $max_attempts ]; then
        echo -e "${RED}FAILED${NC}"
        echo "ClickHouse did not become ready in time"
        exit 1
    fi
    sleep 1
    attempt=$((attempt + 1))
    echo -n "."
done
echo -e "${GREEN}OK${NC}"
echo ""

# Get ClickHouse version
CLICKHOUSE_VERSION=$($CLICKHOUSE_CLIENT --query="SELECT version()" 2>/dev/null || echo "unknown")
echo -e "${BLUE}ClickHouse Version: $CLICKHOUSE_VERSION${NC}"
echo ""

# Initialize summary file
cat > "$SUMMARY_FILE" <<EOF
### 📊 ClickHouse UDF Integration Test Results

> **Version:** \`$CLICKHOUSE_VERSION\`
> **Test Date:** \`$(date -u +"%Y-%m-%d %H:%M:%S UTC")\`

EOF

# Verify UDF functions are registered (console output only, not in report)
echo "Verifying UDF functions are registered..."
echo ""

# Function to check if UDF exists (console output only)
check_udf() {
    local function_name=$1
    if $CLICKHOUSE_CLIENT \
        --query="SELECT name FROM system.functions WHERE name = '$function_name'" 2>/dev/null | grep -q "$function_name"; then
        echo -e "  ✓ ${GREEN}$function_name${NC} registered"
        return 0
    else
        echo -e "  ✗ ${RED}$function_name${NC} not found"
        return 1
    fi
}

# List of expected UDF functions
udfs=(
    "readWktLineString"
    "vinCleaner"
    "vinManuf"
    "vinYear"
    "extractUrl"
    "hasUrl"
    "arrayTopK"
    "extractPhone"
    "tiktokenCount"
    "tiktokenEncode"
)

missing_udfs=0
for udf in "${udfs[@]}"; do
    if ! check_udf "$udf"; then
        missing_udfs=$((missing_udfs + 1))
    fi
done

if [ $missing_udfs -gt 0 ]; then
    echo ""
    echo -e "${YELLOW}Warning: $missing_udfs UDF(s) not registered${NC}"
    echo "Tests for missing UDFs may fail"
fi

echo ""
echo "================================================"
echo "Running SQL Tests"
echo "================================================"
echo ""

cat >> "$SUMMARY_FILE" <<EOF

## Test Results

| Test | Status |
|------|--------|
EOF

# Run each SQL test file
total_tests=0
failed_tests=0
passed_tests=0
declare -a failed_test_names
declare -A failed_test_outputs

for test_file in "$TEST_DIR/sql"/test_*.sql; do
    if [ ! -f "$test_file" ]; then
        continue
    fi

    test_name=$(basename "$test_file")
    total_tests=$((total_tests + 1))

    echo -n "Running $test_name... "

    # Check if expected output file exists
    expected_file="${test_file%.sql}.expected"

    # Run the test and capture output
    if output=$($CLICKHOUSE_CLIENT --multiquery < "$test_file" 2>&1); then
        # Generate expected file mode
        if [ "$GENERATE_EXPECTED" == "true" ]; then
            echo "$output" > "$expected_file"
            echo -e "${GREEN}Generated $expected_file${NC}"
            passed_tests=$((passed_tests + 1))
        # Check if we have an expected output file to compare
        elif [ -f "$expected_file" ]; then
            # Compare actual output with expected
            expected_output=$(cat "$expected_file")
            if diff_output=$(diff -u <(echo "$expected_output") <(echo "$output") 2>&1); then
                echo -e "${GREEN}PASSED${NC}"
                passed_tests=$((passed_tests + 1))
                echo "| $test_name | ✅ PASSED |" >> "$SUMMARY_FILE"
            else
                echo -e "${RED}FAILED (output mismatch)${NC}"
                failed_tests=$((failed_tests + 1))
                failed_test_names+=("$test_name")
                failed_test_outputs["$test_name"]="Output mismatch:\n\n$diff_output"
                echo "| $test_name | ❌ FAILED (output mismatch) |" >> "$SUMMARY_FILE"
                echo "Output mismatch:"
                echo "$diff_output"
                echo ""
            fi
        else
            # No expected file, just check if command succeeded
            echo -e "${GREEN}PASSED${NC} (no expected output file)"
            passed_tests=$((passed_tests + 1))
            echo "| $test_name | ✅ PASSED |" >> "$SUMMARY_FILE"

            # Show hint to generate expected file
            echo -e "${YELLOW}  Hint: Run with --generate-expected to create ${expected_file}${NC}"
        fi
    else
        echo -e "${RED}FAILED${NC}"
        failed_tests=$((failed_tests + 1))
        failed_test_names+=("$test_name")
        failed_test_outputs["$test_name"]="$output"
        echo "| $test_name | ❌ FAILED |" >> "$SUMMARY_FILE"
        echo "Error output:"
        echo "$output"
        echo ""
    fi
done

echo ""
echo "================================================"
echo "Test Summary"
echo "================================================"
echo "Total tests: $total_tests"
echo -e "Passed: ${GREEN}$passed_tests${NC}"
echo -e "Failed: ${RED}$failed_tests${NC}"
echo ""

# Add summary to file
success_rate=$(awk "BEGIN {printf \"%.1f\", ($passed_tests/$total_tests)*100}")

# Determine status emoji based on success rate
if [ "$failed_tests" -eq 0 ]; then
    status_emoji="🎉"
else
    status_emoji="⚠️"
fi

cat >> "$SUMMARY_FILE" <<EOF

| Metric | Value |
|--------|------:|
| **Total Tests** | \`$total_tests\` |
| **Passed** | \`$passed_tests\` ✅ |
| **Failed** | \`$failed_tests\` ❌ |
| **Success Rate** | **$success_rate%** |

EOF

if [ $failed_tests -gt 0 ]; then
    echo -e "${RED}Some tests failed!${NC}"
    cat >> "$SUMMARY_FILE" <<EOF

---

### ⚠️ Failed Tests

The following tests did not pass:

EOF
    for test in "${failed_test_names[@]}"; do
        echo "" >> "$SUMMARY_FILE"
        echo "<details>" >> "$SUMMARY_FILE"
        echo "<summary>❌ <code>$test</code></summary>" >> "$SUMMARY_FILE"
        echo "" >> "$SUMMARY_FILE"
        echo '```' >> "$SUMMARY_FILE"
        echo "${failed_test_outputs[$test]}" >> "$SUMMARY_FILE"
        echo '```' >> "$SUMMARY_FILE"
        echo "" >> "$SUMMARY_FILE"
        echo "</details>" >> "$SUMMARY_FILE"
    done
    cat >> "$SUMMARY_FILE" <<EOF

> **Action Required:** Please review the test failures above and check the logs for details.

EOF

    echo "Summary written to: $SUMMARY_FILE"
    exit 1
else
    echo -e "${GREEN}All tests passed!${NC}"
    cat >> "$SUMMARY_FILE" <<EOF

---

### 🎉 All Tests Passed!

All integration tests completed successfully. The UDF functions are working correctly with ClickHouse \`$CLICKHOUSE_VERSION\`.

EOF

    echo "Summary written to: $SUMMARY_FILE"
    exit 0
fi
