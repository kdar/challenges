f :: Int -> [Int] -> [Int]
f n [x] = replicate n x
f n (x:xs) = (replicate n x) ++ f n xs

-- way #2
-- f n = concat . map (replicate n)

-- This part handles the Input and Output and can be used as it is. Do not modify this part.
main :: IO ()
main = getContents >>=
       mapM_ print. (\(n:arr) -> f n arr). map read. words
