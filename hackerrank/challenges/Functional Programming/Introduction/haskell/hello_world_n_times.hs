hello_worlds n = mapM_ putStrLn (take n (repeat "Hello World"))

-- way #2
-- hello_worlds n = mapM putStrLn $ replicate n "Hello World"

-- way #3
-- hello_world 1 = "Hello World"
-- hello_world n = "Hello World\n" ++ hello_world(n - 1)
-- hello_worlds n = putStrLn (hello_world (n))

-- way #4
-- hello_worlds n = putStrLn (concat (replicate n "Hello World\n"))

-- This part is related to the Input/Output and can be used as it is
-- Do not modify it
main = do
   n <- readLn :: IO Int
   hello_worlds n
