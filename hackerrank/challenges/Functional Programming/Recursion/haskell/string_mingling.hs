main = do
  input1 <- getLine
  input2 <- getLine
  putStrLn $ concat $ zipWith (\x y -> x:y:[]) input1 input2
