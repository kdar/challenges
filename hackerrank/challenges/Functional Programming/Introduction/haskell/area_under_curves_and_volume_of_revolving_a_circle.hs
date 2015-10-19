import Text.Printf (printf)

step :: Double
step = 0.001

f :: Double -> [Int] -> [Int] -> Double
f x [] [] = 0
f x (x1:xs1) (x2:xs2) = fromIntegral (x1) * x^^fromIntegral (x2) + (f x xs1 xs2)

-- returns a list of the stepping we need to take. we can't use ranges
-- because of floating point precision.
steprange :: Double -> Double -> Double -> [Double]
steprange l r s = if l <= r then l:steprange (l+s) r s else []

area :: Int -> Int -> [Int] -> [Int] -> Double
area l r a b = sum (map (\x -> (f x a b) * step) (steprange (fromIntegral l) (fromIntegral r) step))

volume l r a b = sum (map (\x -> pi * (f x a b) ** 2 * step) (steprange (fromIntegral l) (fromIntegral r) step))

-- This function should return a list [area, volume].
solve :: Int -> Int -> [Int] -> [Int] -> [Double]
solve l r a b = [area l r a b, volume l r a b]

--Input/Output.
main :: IO ()
main = getContents >>= mapM_ (printf "%.1f\n"). (\[a, b, [l, r]] -> solve l r a b). map (map read. words). lines
