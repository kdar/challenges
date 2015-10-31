import Control.Monad

fullOfColors [] r g y b = r==g && y==b
fullOfColors (x:xs) r g y b
  | abs (r-g) > 1 = False
  | abs (y-b) > 1 = False
  | otherwise = fullOfColors xs (r+rcount) (g+gcount) (y+ycount) (b+bcount)
  where rcount = if x == 'R' then 1 else 0
        gcount = if x == 'G' then 1 else 0
        ycount = if x == 'Y' then 1 else 0
        bcount = if x == 'B' then 1 else 0

main = do
  t <- readLn
  replicateM_ t $ do
     s <- getLine
     print $ fullOfColors s 0 0 0 0

-- main = do
--   print $ fullOfColors "RGGR" 0 0 0 0
--   print $ fullOfColors "RYBG" 0 0 0 0
--   print $ fullOfColors "RYRB" 0 0 0 0
--   print $ fullOfColors "YGYGRBRB" 0 0 0 0
