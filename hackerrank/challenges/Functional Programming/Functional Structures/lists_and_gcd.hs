import Data.List

-- turns a list like [1,2,3,4] into [(1,2),(3,4)]
-- where for this problem, the first element of the tuple
-- is the base number, and the second is the exponent
pair [] = []
pair (a:b:xs) = (a,b) : pair xs

primegcd [] _ = []
primegcd _ [] = []
-- put together a list of bases and the minimum of the exponents from the two lists
primegcd xs ys = zipWith (\a b -> (fst a, min (snd a) (snd b))) lst1 lst2
  where
    cmp x y = fst x == fst y -- we only care about the base number and not exponent
    lst1 = intersectBy cmp xs ys -- get common elements of xs and ys
    lst2 = intersectBy cmp ys xs -- get common elements of yx and xs

-- folds the list calling primegcd on each pair
primegcdLst (x:xs) = foldl (primegcd) x xs

main = do
  _ <- getLine
  content <- getContents
  let lst = map (\x -> pair . map (read::String->Int) $ words x) (lines content)
      answer = primegcdLst lst
      display = unwords $ map show $ concat [[x, y] | (x,y) <- answer]
  putStrLn display
