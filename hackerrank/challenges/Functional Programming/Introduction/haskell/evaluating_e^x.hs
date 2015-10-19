factorial :: Int -> Int
factorial 0 = 1
factorial n = n * factorial (n - 1)

ex :: Double -> Int -> [Double]
ex _ 0 = [1]
ex x n = ex x (n-1) ++ [(x^n)/fromIntegral (factorial n)]

solve :: Double -> Double
solve x = sum (ex x 9) -- 0 indexed

main :: IO ()
main = getContents >>= mapM_ print. map solve. map (read::String->Double). tail. words
