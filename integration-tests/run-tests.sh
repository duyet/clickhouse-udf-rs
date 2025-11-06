#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
CLICKHOUSE_HOST="${CLICKHOUSE_HOST:-localhost}"
CLICKHOUSE_PORT="${CLICKHOUSE_PORT:-8123}"
CLICKHOUSE_CLIENT="${CLICKHOUSE_CLIENT:-clickhouse-client}"
TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "================================================"
echo "ClickHouse UDF Integration Tests"
echo "================================================"
echo "Host: $CLICKHOUSE_HOST:$CLICKHOUSE_PORT"
echo "Test Directory: $TEST_DIR"
echo ""

# Wait for ClickHouse to be ready
echo -n "Waiting for ClickHouse to be ready..."
max_attempts=30
attempt=0
until $CLICKHOUSE_CLIENT --host="$CLICKHOUSE_HOST" --port="$CLICKHOUSE_PORT" --query="SELECT 1" > /dev/null 2>&1; do
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

# Verify UDF functions are registered
echo "Verifying UDF functions are registered..."
echo ""

# Function to check if UDF exists
check_udf() {
    local function_name=$1
    if $CLICKHOUSE_CLIENT --host="$CLICKHOUSE_HOST" --port="$CLICKHOUSE_PORT" \
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

# Run each SQL test file
total_tests=0
failed_tests=0
passed_tests=0

for test_file in "$TEST_DIR/sql"/test_*.sql; do
    if [ ! -f "$test_file" ]; then
        continue
    fi

    test_name=$(basename "$test_file")
    total_tests=$((total_tests + 1))

    echo -n "Running $test_name... "

    # Run the test and capture output
    if output=$($CLICKHOUSE_CLIENT --host="$CLICKHOUSE_HOST" --port="$CLICKHOUSE_PORT" \
        --multiquery < "$test_file" 2>&1); then
        echo -e "${GREEN}PASSED${NC}"
        passed_tests=$((passed_tests + 1))

        # Optionally show output (uncomment to debug)
        # echo "$output" | head -20
    else
        echo -e "${RED}FAILED${NC}"
        failed_tests=$((failed_tests + 1))
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

if [ $failed_tests -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
fi
