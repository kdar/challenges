import Control.Monad

data Tree a = Empty | Node a (Tree a) (Tree a) deriving (Show)

-- swap nodes given a depth list (x:xs)
swapNodes _ _ [] = []
swapNodes node d (x:xs) = newroot : swapNodes newroot d xs
  where
    newroot = swapNodes' node d x

-- swap at a single depth (n)
swapNodes' Empty _ _ = Empty
swapNodes' (Node a l r) d n
  | d `mod` n == 0 = Node a (swapNodes' r (d+1) n) (swapNodes' l (d+1) n)
  | otherwise = Node a (swapNodes' l (d+1) n) (swapNodes' r (d+1) n)

-- treePrettyPrint (Empty)
--     = "Empty root."
-- treePrettyPrint t = unlines $ treePrettyPrintHelper t
--
-- treePrettyPrintHelper (Node node left right)
--     = (show node) : (treePrettyPrint_subtree left right)
--         where
--             treePrettyPrint_subtree left right =
--                 ((pad "+- " "|  ") (treePrettyPrintHelper right))
--                     ++ ((pad "`- " "   ") (treePrettyPrintHelper left))
--             pad first rest = zipWith (++) (first : repeat rest)
-- treePrettyPrintHelper (Empty)
--     = []

treePrint Empty = []
treePrint (Node a l r) = treePrint l ++ [show a] ++ treePrint r

-- convert an array to a tree. we start at 1 for our
-- root node
treefromarr :: [[Int]] -> Tree Int
treefromarr arr = treefromarr' arr 1

-- this builds a tree from array starting at `at`.
-- e.g. for this tree:
 --   1
 --  / \
 -- 2   3
 --  \   \
 --   4   5
 -- the array version (the input given to the program) would look like:
 -- [[2,3],[-1,4],[-1,5],[-1,-1],[-1,-1]]
 -- so this algorithm will start with at = 1, and then transverse
 -- the above array. it will recursively create a tree and get the next
 -- part of the array that corresponds to the root it last read. So
 -- for example, when we evaluate:
 -- Node 1 (treefromarr' arr (arr !! 0 !! 0)) (treefromarr' (arr !! 0 !! 1))
 -- we get:
 -- Node at (treefromarr' arr 2) (treefromarr' arr 3)
 -- when we run again with at = 2, we get:
 -- Node 2 (treefromarr' arr (arr || 1 || 0)) (treefromarr' (arr || 1 || 1))
 -- which evaluates to
 -- Node 2 (treefromarr' arr -1) (treefromarr' arr 4)
 -- and with at = 3:
 -- Node 3 (treefromarr' arr -1) (treefromarr' arr 5)
 -- and so on.
treefromarr' :: [[Int]] -> Int -> Tree Int
treefromarr' arr at
  | at < 0 = Empty
  | otherwise =
      Node at
      (treefromarr' arr (arr !! (at-1) !! 0))
      (treefromarr' arr (arr !! (at-1) !! 1))

readninput = do
  line <- getLine
  let nlines = read line :: Int
  sequence [getLine | _ <- [1..nlines]]

main = do
  lines <- readninput
  let childlist = map (map (read :: String -> Int) . words) lines
  let root = treefromarr childlist

  lines <- readninput
  let transforms = map (read :: String -> Int) lines
  putStrLn $ unlines $ map (unwords . treePrint) (swapNodes root 1 transforms)
