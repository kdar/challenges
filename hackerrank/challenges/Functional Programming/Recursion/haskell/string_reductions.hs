reduce [] _ = []
reduce (x:xs) seen = if x `elem` seen then reduce xs seen else x:(reduce xs (x:seen))

main = do
  input <- getLine
  putStrLn $ reduce input []
