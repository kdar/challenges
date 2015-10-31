import Control.Monad

permute [] = []
permute (a:b:xs) = b:a:(permute xs)

main = do
  t <- readLn
  replicateM_ t $ do
     s <- getLine
     putStrLn $ permute s
