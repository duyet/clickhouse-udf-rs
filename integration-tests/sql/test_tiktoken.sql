-- Tiktoken Functions Tests
-- These tests verify that tiktoken functions work correctly

-- Test tiktokenCount
SELECT 'Test 1: tiktokenCount simple text' AS test_name, tiktokenCount('Hello, world!') AS result;
-- Expected: 4 (token count)

SELECT 'Test 2: tiktokenCount empty string' AS test_name, tiktokenCount('') AS result;
-- Expected: 0

SELECT 'Test 3: tiktokenCount single word' AS test_name, tiktokenCount('Hello') AS result;
-- Expected: 1

-- Test tiktokenEncode
SELECT 'Test 4: tiktokenEncode simple text' AS test_name, tiktokenEncode('Hello') AS result;
-- Expected: token IDs as string (e.g., "9906" or similar)

SELECT 'Test 5: tiktokenEncode empty string' AS test_name, tiktokenEncode('') AS result;
-- Expected: empty string or empty result
