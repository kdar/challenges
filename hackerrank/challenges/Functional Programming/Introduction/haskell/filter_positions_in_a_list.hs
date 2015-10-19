f :: [Int] -> [Int]
f [] = []
f (_:xs) = (head xs):(f (tail xs))

-- way #2
-- f [] = []
-- f [x] = []
-- f (a:b:lst) = b : f lst

-- way #
-- f lst = [x | (x,y) <- zip lst [1..length lst], even y]

-- This part deals with the Input and Output and can be used as it is. Do not modify it.
main = do
   inputdata <- getContents
   mapM_ (putStrLn. show). f. map read. lines $ inputdata
