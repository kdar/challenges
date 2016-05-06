solveMeFirst a b = a + b

main = do
    val1 <- readLn
    val2 <- readLn
    let sum = solveMeFirst val1 val2
    print sum

-- import Control.Monad
--
--
-- main = do
--   t <- readLn
--   replicateM_ t $ do
--      s <- getLine
--      putStrLn $ s

-- reading in space delimited numbers:
-- input <- getLine
-- map (read :: String -> Int) $ words input


-- readninput = do
--   line <- getLine
--   let nlines = read line :: Int
--   sequence [getLine | _ <- [1..nlines]]
-- lines <- readninput
