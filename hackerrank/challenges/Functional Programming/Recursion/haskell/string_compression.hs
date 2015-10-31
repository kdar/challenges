compress [] = []
compress (x:[]) = [x]
compress (x:s)
  | x == head s =
    let removed = length $ takeWhile (x==) s
    in [x] ++ (show $ succ $ removed) ++ compress (drop removed s)
  | otherwise = x:compress s

main = do
  str <- getLine
  putStrLn $ compress str
