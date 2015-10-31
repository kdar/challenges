rec x n step
  | x < stepn = 0
  | x == stepn = 1
  | otherwise = rec (x - stepn) n (step+1) + rec x n (step+1)
  where stepn = step^n

sumsOfPowers x n = rec x n 1

-- countTheWays 0 _ = 0
-- countTheWays x n = count x $ map (^n) [1..]
--   where
--     count 0 _ = 1
--     count _ [] = 0
--     count x (y:ys) | x >= y = count (x - y) ys + count x ys
--                    | otherwise = 0

main = do
  x <- readLn :: IO Int
  n <- readLn :: IO Int
  print $ sumsOfPowers x n
  -- print $ countTheWays x n
