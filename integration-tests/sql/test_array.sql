-- Array Functions Tests
-- Test arrayTopK function

-- Test 1: arrayTopK with k=3
SELECT 'Test 1: arrayTopK(3)' AS test_name, arrayTopK(3)([1, 1, 2, 2, 3, 4, 5]) AS result;
-- Expected: [2,1,3] or similar (most frequent elements)

-- Test 2: arrayTopK with k=1
SELECT 'Test 2: arrayTopK(1)' AS test_name, arrayTopK(1)([2, 3, 4, 5]) AS result;
-- Expected: [2] or any single element

-- Test 3: arrayTopK with k=2
SELECT 'Test 3: arrayTopK(2)' AS test_name, arrayTopK(2)([1, 1, 2, 2, 2]) AS result;
-- Expected: [2,1]

-- Test 4: arrayTopK with empty array
SELECT 'Test 4: arrayTopK(3) empty array' AS test_name, arrayTopK(3)([]) AS result;
-- Expected: []

-- Test 5: arrayTopK with k=0
SELECT 'Test 5: arrayTopK(0)' AS test_name, arrayTopK(0)([1, 2, 3]) AS result;
-- Expected: []
