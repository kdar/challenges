import Control.Monad

maxSubarray _ m2 [] = m2
maxSubarray m1 m2 (x:xs) = maxSubarray maxHere maxSoFar xs
  where
    maxHere = max 0 (m1 + x)
    maxSoFar = max m2 maxHere

main = do
  t <- readLn
  replicateM_ t $ do
    s <- getLine
    s <- getLine
    let list = map read $ words s
        maxSingle = maximum list
        m1 = maxSubarray 0 0 list
        m2 = sum $ filter (>0) list
    putStrLn $ show (if m1 > 0 then m1 else maxSingle) ++ " " ++ show (if m2 > 0 then m2 else maxSingle)
