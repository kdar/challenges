-- fact n = product [1..n]
-- f r c = fact r `quot` (fact c * fact (r-c))
-- pascalRow r = map (f r) [0..r]
-- pascal n = map pascalRow [0..n]

-- The root element is 1.
-- Every other element is the sum of the one or two above it (diagonally left and diagonally right).
nextRow row = zipWith (+) (0:row) (row ++ [0])
pascal = iterate nextRow [1]

main = do
  input <- getLine
  putStr $ unlines $ map unwords $ map (map show) $ take ((read :: String -> Int) $ input) pascal
  --putStr $ unlines $ map unwords $ map (map show) $ pascal . (read :: String -> Int) $ input
