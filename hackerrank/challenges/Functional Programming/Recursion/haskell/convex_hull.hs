import Data.List (sort)
import Text.Printf

type R = Double
type R2 = (R, R)

clockwise :: R2 -> R2 -> R2 -> Bool
clockwise o a b = (a `sub` o) `cross` (b `sub` o) <= 0

cross :: R2 -> R2 -> R
cross (x1, y1) (x2, y2) = x1 * y2 - x2 * y1

sub :: R2 -> R2 -> R2
sub (x1, y1) (x2, y2) = (x1 - x2, y1 - y2)

convexHull :: [R2] -> [R2]
convexHull [] = []
convexHull [p] = [p]
convexHull points = lower ++ upper
  where
    sorted = sort points
    lower = chain sorted
    upper = chain (reverse sorted)

chain :: [R2] -> [R2]
chain = go []
  where
    go :: [R2] -> [R2] -> [R2]
    go acc@(r1:r2:rs) (x:xs) =
      if clockwise r2 r1 x
        then go (r2:rs) (x:xs)
        else go (x:acc) xs
    go acc (x:xs) = go (x:acc) xs
    go acc [] = reverse $ tail acc

shift l n = drop n l ++ take n l
perimeter points = foldl (\z ((x1,y1),(x2,y2)) -> z + sqrt ((x1-x2)^2+(y1-y2)^2)) 0 $ zip points (shift points 1)

main :: IO ()
main = do
  n <- readLn :: IO Int
  content <- getContents
  let
    points = map (\[x, y] -> (x, y)). map (map (read::String->Double)). map words. lines $ content
    ans = convexHull points
  printf "%.1f\n" $ perimeter $ ans
