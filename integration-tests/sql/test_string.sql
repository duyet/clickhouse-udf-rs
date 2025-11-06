-- String Functions Tests

-- Test extractPhone
SELECT 'Test 1: extractPhone with plus sign' AS test_name, extractPhone('Call me at +123 456 7890') AS result;
-- Expected: 1234567890

SELECT 'Test 2: extractPhone with dashes' AS test_name, extractPhone('My number is 123-456-7890.') AS result;
-- Expected: 1234567890

SELECT 'Test 3: extractPhone plain number' AS test_name, extractPhone('My number is 1234567890.') AS result;
-- Expected: 1234567890

SELECT 'Test 4: extractPhone no number' AS test_name, extractPhone('No phone number here.') AS result;
-- Expected: (empty or NULL)

SELECT 'Test 5: extractPhone short number' AS test_name, extractPhone('123-456') AS result;
-- Expected: (empty or NULL)
