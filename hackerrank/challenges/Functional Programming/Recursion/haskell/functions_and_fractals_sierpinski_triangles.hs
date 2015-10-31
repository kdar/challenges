-- method 1
-- this method works by recursively flipping the tophalf of the full triangle
-- and then using that to cut out the bottom half.

-- -- draws the base triangle given the width w and starting
-- -- row r
-- baseTriangle w r
--   | ones > w = []
--   | otherwise = [replicate blanks '_' ++ replicate ones '1' ++ replicate blanks '_'] ++ baseTriangle w (r+1)
--   where ones = r+r-1; blanks = (w-ones) `div` 2
--
-- -- flips the top half of the passed rows and then if there is
-- -- a '1' in the flipped and original version, then change it
-- -- to '_'
-- flipAndCut rows = let
--     height = length rows `div` 2
--     width = length $ head rows
--     flipped = (replicate height (replicate width '_')) ++ (reverse $ take height rows)
--   in
--     zipWith (zipWith (\x y -> if y == '1' && x == '1' then '_' else x)) rows flipped
--
-- -- this works by recursively flipping and cutting triangles.
-- sierpinski tri i n
--   | n == 0 = tri
--   | i > n = []
--   | null tophalf = new -- if the next iteration is an empty list, just use what we just cut.
--   | otherwise = tophalf ++ bottomhalf
--   where
--     new = flipAndCut tri
--     height = length tri `div` 2 -- we cut the tri into two pieces and recurse on it
--     tophalf = sierpinski (take height new) (i+1) n -- next recursion on top half
--     bottomhalf = sierpinski (drop height new) (i+1) n -- next recursion on bottom half
--
-- main = do
--   input <- getLine
--   putStr $ unlines $ sierpinski (baseTriangle 63 1) 1 ((read :: String -> Int) $ input)


-- method 2
-- this method functions by first working with only half of the triangle split vertically.
-- then we recursively append the top half of the triangle and the
-- mirror of the top half
-- e.g. the starting triangle:
-- 1_______
-- 11______
-- 111_____
-- 1111____
-- 11111___
-- 111111__
-- 1111111_
-- 11111111
-- the first iteration will take this top half, which is
-- 1_______
-- 11______
-- 111_____
-- 1111____
-- and mirror it while adding a _ to the front, which is
-- ____1___
-- ___111__
-- __11111_
-- _1111111
-- and then just appending the top and new bottom:
-- 1_______
-- 11______
-- 111_____
-- 1111____
-- ____1___
-- ___111__
-- __11111_
-- _1111111
-- we keep doing this recursively.
-- at the end, we just mirror this thing to make our triangle:
-- _______1_______
-- ______111______
-- _____11111_____
-- ____1111111____
-- ___1_______1___
-- __111_____111__
-- _11111___11111_
-- 1111111_1111111

-- creates a triangle. e.g. triangle 2 = ["1", "11"]
triangle n = map (flip replicate '1') [1..n]

sierpinski h n = case n of
    0 -> undefined
    1 -> applyUnderscores $ triangle h
    n -> let s = sierpinski (h `div` 2) (n - 1)
         in applyUnderscores $ s ++ (map ('_':) $ mirror s)

-- takes a triangle and appends underscores to the end of each line.
-- e.g. if we have ["1", "11", "111"], we get out ["1__", "11_", "111"]
applyUnderscores x = let l = maximum (map length x) in map (take l . (++ repeat '_')) x

-- simply takes each element in a list and mirrors it.
-- e.g. if we have ["11__"], we get out ["__1111__"]
mirror x = map (\l -> reverse (tail l) ++ l) x

main = do
  readLn >>= mapM putStrLn . mirror . sierpinski 32 . succ
