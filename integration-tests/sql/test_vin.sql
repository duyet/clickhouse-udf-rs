-- VIN (Vehicle Identification Number) Tests

-- Test vinCleaner
SELECT 'Test 1: vinCleaner uppercase' AS test_name, vinCleaner('1G1ND52F14M712344') AS result;
-- Expected: 1G1ND52F14M712344

SELECT 'Test 2: vinCleaner lowercase' AS test_name, vinCleaner('1g1nd52f14m712344') AS result;
-- Expected: 1G1ND52F14M712344

SELECT 'Test 3: vinCleaner with extra text' AS test_name, vinCleaner('1G1ND52F14M712344 (ok)') AS result;
-- Expected: 1G1ND52F14M712344

SELECT 'Test 4: vinCleaner with spaces' AS test_name, vinCleaner('  1G1ND52F14M712344  ') AS result;
-- Expected: 1G1ND52F14M712344

-- Test vinManuf
SELECT 'Test 5: vinManuf Chevrolet' AS test_name, vinManuf('1GKKRNED9EJ262581') AS result;
-- Expected: General Motors USA

SELECT 'Test 6: vinManuf Mazda' AS test_name, vinManuf('JM1BL1M72C1587426') AS result;
-- Expected: Mazda

SELECT 'Test 7: vinManuf Ford' AS test_name, vinManuf('1FTEW1CM9BFA74557') AS result;
-- Expected: Ford Motor Company

-- Test vinYear
SELECT 'Test 8: vinYear 2014' AS test_name, vinYear('1GKKRNED9EJ262581') AS result;
-- Expected: 2014

SELECT 'Test 9: vinYear 2012' AS test_name, vinYear('JM1BL1M72C1587426') AS result;
-- Expected: 2012

SELECT 'Test 10: vinYear 2011' AS test_name, vinYear('1FTEW1CM9BFA74557') AS result;
-- Expected: 2011
