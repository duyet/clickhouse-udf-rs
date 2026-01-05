-- WKT (Well-Known Text) Tests
-- Test readWktLineString function

-- Test 1: Valid LINESTRING
SELECT 'Test 1: Valid LINESTRING' AS test_name, readWktLineString('LINESTRING(0 0, 1 1, 2 2)') AS result;
-- Expected: [(0,0),(1,1),(2,2)]

-- Test 2: Valid LINESTRING with spaces
SELECT 'Test 2: Valid LINESTRING with spaces' AS test_name, readWktLineString('LINESTRING(30 10, 10 30, 40 40)') AS result;
-- Expected: [(30,10),(10,30),(40,40)]

-- Test 3: Invalid LINESTRING (missing closing parenthesis)
SELECT 'Test 3: Invalid LINESTRING' AS test_name, readWktLineString('LINESTRING(0 0, 1 1, 2 2') AS result;
-- Expected: (empty string)

-- Test 4: Empty string
SELECT 'Test 4: Empty string' AS test_name, readWktLineString('') AS result;
-- Expected: (empty string)
