import Control.Monad

shift l n = drop n l ++ take n l
rotate _ [] = []
rotate 0 _ = []
rotate n xs = [shifted] ++ rotate (n-1) shifted
  where shifted = shift xs 1

main = do
  t <- readLn
  replicateM_ t $ do
     s <- getLine
     putStrLn $ unwords $ rotate (length s) s
