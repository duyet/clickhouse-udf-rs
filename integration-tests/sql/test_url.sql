-- URL Functions Tests

-- Test extractUrl
SELECT 'Test 1: extractUrl simple http' AS test_name, extractUrl('http://example.org') AS result;
-- Expected: http://example.org

SELECT 'Test 2: extractUrl simple https' AS test_name, extractUrl('https://example.org') AS result;
-- Expected: https://example.org

SELECT 'Test 3: extractUrl with text before' AS test_name, extractUrl('check this https://duyet.net awesome') AS result;
-- Expected: https://duyet.net

SELECT 'Test 4: extractUrl with path' AS test_name, extractUrl('https://example.org/abc/def something') AS result;
-- Expected: https://example.org/abc/def

SELECT 'Test 5: extractUrl no URL' AS test_name, extractUrl('no url here') AS result;
-- Expected: (empty or NULL)

-- Test hasUrl
SELECT 'Test 6: hasUrl with URL' AS test_name, hasUrl('extract from this https://duyet.net') AS result;
-- Expected: true

SELECT 'Test 7: hasUrl without URL' AS test_name, hasUrl('no url here') AS result;
-- Expected: false

SELECT 'Test 8: hasUrl ftp protocol' AS test_name, hasUrl('ftp://example.org') AS result;
-- Expected: true
