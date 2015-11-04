import Control.Monad

-- Given 5 people, the first one shakes with 4, second with 3, third with 2,
-- and fourth with 1. e.g. 4+3+2+1 = 10.
-- https://en.wikipedia.org/wiki/1_%2B_2_%2B_3_%2B_4_%2B_%E2%8B%AF

handshake 0 = 0
handshake 1 = 0
handshake n = n*(n-1) `div` 2

main = do
  t <- readLn
  replicateM_ t $ do
     n <- readLn :: IO Integer
     print $ handshake n
