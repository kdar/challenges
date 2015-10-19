f arr = sum [x | x <- arr, x `mod` 2 /= 0]

-- way #2
-- f = sum . filter odd

-- This part handles the Input/Output and can be used as it is. Do not change or modify it.
main = do
   inputdata <- getContents
   putStrLn $ show $ f $ map (read :: String -> Int) $ lines inputdata
