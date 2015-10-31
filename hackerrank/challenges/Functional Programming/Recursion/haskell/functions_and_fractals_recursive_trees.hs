-- method 1
-- -- This method works by recursively creating the base tree and then
-- -- the trees above it, merging it all together.
-- -- To make this easier, it will create each separate tree on a complete
-- -- canvas (63 rows and 100 cols) with it positioned correctly within it,
-- -- and then merge all the trees together.
--
-- maxRows = 63
-- maxCols = 100
--
-- -- takes two trees and merges t2 into t1 if there is a '1' present
-- mergeTrees t1 t2 = zipWith (zipWith (\x y -> if y == '1' then y else x)) t1 t2
--
-- -- this is a function that takes the last N elements of a list
-- lastN n xs = let m = length xs in drop (m-n) xs
--
-- -- fills in the top of the passed tree with '_'s until it fills up
-- -- the remaining space
-- fillTop t = lastN maxRows (replicate maxRows (replicate maxCols '_') ++ t)
--
-- -- create the branches of the tree at x,y
-- branches x y height =
--   map (\c -> replicate (x-c-1) '_' ++ '1':replicate (c+c-1) '_' ++ "1" ++ replicate (maxCols-x-c) '_')
--     $ reverse [1,2..height]
--
-- tree _ _ 0 _ = fillTop []
-- tree x y depth height = let
--   hh = height `div` 2
--   tree1 = tree (x-hh) (y+hh+hh) (depth-1) hh -- get the next left tree
--   tree2 = tree (x+hh) (y+hh+hh) (depth-1) hh -- get the next right tree
--   in
--   -- merge the next trees with this tree
--   mergeTrees
--     (mergeTrees tree1 tree2) -- merge left and right next trees
--     -- create this tree
--     (fillTop (
--       branches x y hh -- draw branches
--       ++
--       replicate hh (replicate (x-1) '_' ++ "1" ++ replicate (maxCols-x) '_') -- draw stem
--       ++
--       replicate y (replicate maxCols '_') -- effectively moves the tree up by y
--     ))
--
-- main = do
--   n <- readLn
--   putStrLn $ unlines $ tree 50 0 n 32

-- method 2
-- this method works by drawing each line top to bottom, left to right recursively.
-- The function that does all the work is `check`. Given a column length n, size k,
-- and coordinate x and y, it is able to determine if a 1 belongs there.

check 0 _ _ _ = False
check n k y x
  | y < k = 0 == x -- check if it's the stem
  | y < k*2 = y - k + 1 == abs x -- check if it's in the two branches
    -- check recursively up the tree split into two branches
  | otherwise = check (n-1) (div k 2) (y-k*2) (x-k) || check (n-1) (div k 2) (y-k*2) (x+k)

-- draw each column. l is the row (starting from max), c is the column (starting from max)
col n l 0 = []
col n l c =
  (if check n 16 (l-1) (c-51) then '1' else '_'):(col n l (c-1))

-- draw the line. l is the row (starting from max).
line n 0 = []
line n l =
  col n l 100 : line n (l-1)

main = do
  --print $ map (\x -> if check 1 16 16 x then '1' else '_') [-51..51]
  n <- readLn :: IO Int
  putStrLn $ unlines $ line n 63
